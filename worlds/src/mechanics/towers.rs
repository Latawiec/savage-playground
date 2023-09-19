use bevy::{
    prelude::{
        BuildChildren, Bundle, Children, Commands, Component, Entity, Event, EventReader,
        EventWriter, GlobalTransform, Local, Plugin, Query, Res, Transform, Update, Vec2, Vec3, Parent, With,
    },
    time::{Time, Timer, TimerMode}, transform::TransformBundle, ecs::query, 
};
use bevy_rapier2d::prelude::{
    Collider, CollisionGroups, Group, QueryFilter, RapierContext, Sensor,
};
use framework::{
    components::{collision::{collision_groups::*, hitbox::PlayerHitboxTag}, player::identity::Identity},
    types::environment::WorldDirection, blueprints::player, utils::locals::EntitySetTracket,
};
use rand::{seq::SliceRandom, thread_rng};
use std::{
    collections::BTreeSet,
    time::Duration, ops::{Deref, DerefMut},
};

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
    transform: TransformBundle,
    // global_transform: GlobalTransform,
    collider: Collider,
    sensor: Sensor,
    collision_groups: CollisionGroups,
}

impl TowerBundle {
    pub const TOWER_COLLIDER_RADIUS: f32 = 60.0;

    fn new(position_tag: WorldDirection, translation: &Vec2) -> Self {
        Self {
            tower_component: TowerComponent::new(position_tag),
            transform: TransformBundle::from(Transform::from_xyz(translation.x, translation.y, 0.0)),
            collider: Collider::ball(Self::TOWER_COLLIDER_RADIUS),
            sensor: Sensor::default(),
            collision_groups: Self::collision_groups(),
        }
    }

    fn collision_groups() -> CollisionGroups {
        CollisionGroups {
            memberships: Group::from(Group::from_bits_truncate(SENSOR_AOE_HITBOX)),
            filters: Group::from(Group::from_bits_truncate(SENSOR_PLAYER_HITBOX)),
        }
    }
}

#[derive(Component)]
pub struct Towers {
    warmup_timer: Timer,
}

impl Towers {
    const WARMUP_DURATION: Duration = Duration::new(1, 0);

    const INNER_DISTANCE: f32 = 70.5;
    const INNER_POSITIONS_DIR: [WorldDirection; 4] = [
        WorldDirection::North,
        WorldDirection::South,
        WorldDirection::East,
        WorldDirection::West,
    ];

    const OUTER_DISTANCE: f32 = 300.0;
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
        let main = commands.spawn(Towers::default()).insert(TransformBundle::default()).id();

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
    pub player_hitbox: Entity,
    pub position: WorldDirection,
}

#[derive(Event)]
struct PlayerPoisonEvent {
    pub player_hitbox: Entity,
    pub position: WorldDirection,
}

#[derive(Default)]
pub struct TowersMechanicPlugin;

impl Plugin for TowersMechanicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PlayerHitEvent>()
            .add_event::<PlayerPoisonEvent>()
            .add_systems(Update, check)
            .add_systems(Update, update);
    }
}

#[derive(Default)]
struct PlayersOnTowersTracker(EntitySetTracket);

#[derive(Component, Default)]
struct PoisonDebuff {}

fn check(
    mut commands: Commands,
    mut poisoned_players_tracker: Local<PlayersOnTowersTracker>,
    mut ev_hit: EventReader<PlayerHitEvent>,
    mut ev_poison: EventReader<PlayerPoisonEvent>,
    query_player_hitboxes: Query<(Entity, &Parent), With<PlayerHitboxTag>>,
    query_player_identity: Query<&Identity>,
) {
    let mut poisoned_players = BTreeSet::default();
    for poison in ev_poison.iter() {
        let player_hitbox = poison.player_hitbox;
        match query_player_hitboxes.get(player_hitbox) {
            Ok((_, hitbox_parent)) => {
                let player_entity = hitbox_parent.get();
                poisoned_players.insert(player_entity);
            },
            Err(e) => {
                tracing::error!("Couldn't get parent entity from player hitbox: {}", e);
            }
        }
    }
    poisoned_players_tracker.0.update(poisoned_players);

    for poisoned_player in &poisoned_players_tracker.0.just_added {
        commands
            .entity(*poisoned_player)
            .insert(PoisonDebuff::default());

        match query_player_identity.get(*poisoned_player) {
            Ok(player_identity) => {
                tracing::info!("Player \"{}\" poisoned!", player_identity.name);
            },
            Err(e) => {
                tracing::error!("Couldn't get players identity: {}", e);
            }
        }
    }

    // for ev_hit in ev_player_hit.iter() {
    //     if let Ok((_, player_id)) = query_players.get(ev_hit.player) {
    //         tracing::info!("Hit: {}", player_id.name);
    //     }
    // }

    for cured_player in &poisoned_players_tracker.0.just_removed {
        commands.entity(*cured_player).remove::<PoisonDebuff>();
        
        match query_player_identity.get(*cured_player) {
            Ok(player_identity) => {
                tracing::info!("Player \"{}\" cured!", player_identity.name);
            },
            Err(e) => {
                tracing::error!("Couldn't get players identity: {}", e);
            }
        }
    }
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
                    .exclude_collider(tower)
                    .groups(TowerBundle::collision_groups());

                let mut hitboxes_touching = vec![];
                rapier_context.intersections_with_shape(
                    shape_pos,
                    shape_rot,
                    shape,
                    filter,
                    |entity| {
                        hitboxes_touching.push(entity);
                        true
                    },
                );

                if !tower_timer.finished() {
                    for &player_hitbox in &hitboxes_touching {
                        ev_player_poison.send(PlayerPoisonEvent {
                            player_hitbox,
                            position: tower_component.position_tag,
                        })
                    }
                }

                if tower_timer.just_finished() {
                    for &player_hitbox in &hitboxes_touching {
                        ev_player_hit.send(PlayerHitEvent {
                            player_hitbox,
                            position: tower_component.position_tag,
                        })
                    }
                }
            }
        }
    }
}
