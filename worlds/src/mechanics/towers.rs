use bevy::{
    prelude::{
        BuildChildren, Bundle, Children, Commands, Component, Entity, Event, EventReader,
        EventWriter, GlobalTransform, Query, Res, Transform, Vec2, Vec3, Plugin, Update,
    },
    time::{Time, Timer, TimerMode},
};
use bevy_rapier2d::prelude::{
    Collider, CollisionGroups, Group, QueryFilter, RapierContext, Sensor,
};
use framework::{components::collision::collision_groups::*, types::environment::WorldDirection, blueprints::player::Player};
use rand::{seq::SliceRandom, thread_rng};
use std::time::Duration;

#[derive(Component)]
pub struct TowerComponent {
    countdown: Timer,
    position_tag: WorldDirection,
}

impl TowerComponent {
    const COUNTDOWN_DURATION: Duration = Duration::new(10, 0);

    pub fn tick(&mut self, delta_time: Duration) {
        self.countdown.tick(delta_time);
    }

    pub fn new(position_tag: WorldDirection) -> Self {
        TowerComponent {
            countdown: Timer::new(Self::COUNTDOWN_DURATION, TimerMode::Once),
            position_tag,
        }
    }
}

#[derive(Bundle)]
struct TowerBundle {
    tower_component: TowerComponent,
    transform: Transform,
    global_transform: GlobalTransform,
    collider: Collider,
    sensor: Sensor,
}

impl TowerBundle {
    const TOWER_COLLIDER_RADIUS: f32 = 1.0;

    fn new(position_tag: WorldDirection, translation: &Vec2) -> Self {
        Self {
            tower_component: TowerComponent::new(position_tag),
            transform: Transform::from_translation(Vec3::new(translation.x, translation.y, 0.0)),
            global_transform: GlobalTransform::default(),
            collider: Collider::ball(Self::TOWER_COLLIDER_RADIUS),
            sensor: Sensor::default(),
        }
    }
}

#[derive(Component)]
pub struct Towers {
    warmup_timer: Timer,
}

impl Towers {
    const WARMUP_DURATION: Duration = Duration::new(1, 0);

    const INNER_DISTANCE: f32 = 0.5;
    const INNER_POSITIONS_DIR: [WorldDirection; 4] = [
        WorldDirection::North,
        WorldDirection::South,
        WorldDirection::East,
        WorldDirection::West,
    ];

    const OUTER_DISTANCE: f32 = 1.0;
    const OUTER_POSITIONS_DIR: [WorldDirection; 4] = [
        WorldDirection::NorthWest,
        WorldDirection::NorthEast,
        WorldDirection::SouthWest,
        WorldDirection::SouthEast,
    ];

    fn rnd_two_of_vec<T>(slice: &mut [T]) -> [&T; 2] {
        slice.shuffle(&mut thread_rng());
        [&slice[0], &slice[1]]
    }

    pub fn spawn(commands: &mut Commands) {
        let main = commands.spawn(Towers::default()).id();

        // Inner
        for pos in Self::rnd_two_of_vec(&mut Self::INNER_POSITIONS_DIR.clone()) {
            let translation = pos.vec() * Self::INNER_DISTANCE;
            let tower = commands.spawn(TowerBundle::new(*pos, &translation)).id();
            commands.entity(main).add_child(tower);
        }

        // Outer
        for pos in Self::rnd_two_of_vec(&mut Self::OUTER_POSITIONS_DIR.clone()) {
            let translation = pos.vec() * Self::OUTER_DISTANCE;
            let tower = commands.spawn(TowerBundle::new(*pos, &translation)).id();
            commands.entity(main).add_child(tower);
        }
    }
}

impl Default for Towers {
    fn default() -> Self {
        Self {
            warmup_timer: Timer::new(Self::WARMUP_DURATION, TimerMode::Once),
        }
    }
}

#[derive(Event)]
struct PlayerHitEvent {
    pub player: Entity,
    pub position: WorldDirection,
}

#[derive(Event)]
struct PlayerPoisonEvent {
    pub player: Entity,
    pub position: WorldDirection,
}


pub struct TowersMechanicPlugin;

impl Plugin for TowersMechanicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<PlayerHitEvent>()
            .add_event::<PlayerPoisonEvent>()
            .add_systems(Update, check)
            .add_systems(Update, update);
    }
}

fn check(
    mut commands: Commands,
    mut ev_player_hit: EventReader<PlayerHitEvent>,
    mut ev_player_poison: EventReader<PlayerPoisonEvent>,
) {
    for ev_poison in ev_player_poison.iter() {}

    for ev_hit in ev_player_hit.iter() {}
}

fn update(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut ev_player_hit: EventWriter<PlayerHitEvent>,
    mut ev_player_poison: EventWriter<PlayerPoisonEvent>,
    mut query_towers_root: Query<(Entity, &mut Towers, &Children)>,
    mut query_towers: Query<(&mut TowerComponent, &GlobalTransform, &Collider)>,
) {
    let delta_time = time.delta();

    for (root_entity, mut root_component, child_towers) in query_towers_root.iter_mut() {
        root_component.warmup_timer.tick(delta_time);

        // We're not yet started. Still warming up.
        if !root_component.warmup_timer.finished() {
            continue;
        }

        if child_towers.is_empty() {
            // This means children have despawned. Finish up and remove root as well.
            commands.entity(root_entity).despawn();
            continue;
        }

        for &tower in child_towers.iter() {
            if let Ok((mut tower_component, tower_global_transform, tower_collider)) =
                query_towers.get_mut(tower)
            {
                let tower_timer = &mut tower_component.countdown;
                tower_timer.tick(delta_time);

                let tower_timer = &tower_component.countdown;

                if tower_timer.finished() && !tower_timer.just_finished() {
                    // We're past "just_finished". I don't think anything else is gonna happen.
                    commands.entity(tower).despawn();
                    continue;
                }

                let global_translation = tower_global_transform.translation();
                let shape_pos = Vec2::new(global_translation.x, global_translation.y);
                let shape_rot = tower_global_transform
                    .compute_transform()
                    .rotation
                    .to_euler(bevy::prelude::EulerRot::XYZ)
                    .2;
                let shape = tower_collider;
                let filter = QueryFilter::default()
                    .exclude_solids()
                    .groups(CollisionGroups {
                        memberships: Group::from(Group::from_bits_truncate(SENSOR_AOE_HITBOX)),
                        filters: Group::from(Group::from_bits_truncate(SENSOR_PLAYER_HITBOX)),
                    });

                let mut players_touching = vec![];
                rapier_context.intersections_with_shape(
                    shape_pos,
                    shape_rot,
                    shape,
                    filter,
                    |entity| {
                        players_touching.push(entity);
                        true
                    },
                );

                if !tower_timer.finished() {
                    for &player in &players_touching {
                        ev_player_poison.send(PlayerPoisonEvent {
                            player,
                            position: tower_component.position_tag,
                        })
                    }
                }

                if tower_timer.just_finished() {
                    for &player in &players_touching {
                        ev_player_hit.send(PlayerHitEvent {
                            player,
                            position: tower_component.position_tag,
                        })
                    }
                }
            }
        }
    }
}
