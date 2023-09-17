use bevy::{
    prelude::{
        BuildChildren, Bundle, Children, Commands, Component, Entity, Event, EventWriter,
        GlobalTransform, Parent, Query, Res, Transform, Vec2, Vec3,
    },
    time::{Time, Timer, TimerMode},
    transform::TransformBundle,
    utils::hashbrown::raw::Global,
};
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext, Sensor};
use rand::{seq::SliceRandom, thread_rng};
use std::time::Duration;

#[derive(Component)]
pub struct TowerComponent {
    countdown: Timer,
}

impl TowerComponent {
    const COUNTDOWN_DURATION: Duration = Duration::new(10, 0);

    pub fn tick(&mut self, delta_time: Duration) {
        self.countdown.tick(delta_time);
    }
}

impl Default for TowerComponent {
    fn default() -> Self {
        TowerComponent {
            countdown: Timer::new(Self::COUNTDOWN_DURATION, TimerMode::Once),
        }
    }
}

#[derive(Bundle, Default)]
struct TowerBundle {
    tower_component: TowerComponent,
    transform: Transform,
    global_transform: GlobalTransform,
    collider: Collider,
    sensor: Sensor,
}

impl TowerBundle {
    const TOWER_COLLIDER_RADIUS: f32 = 1.0;

    fn new(pos: &Vec2) -> Self {
        Self {
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            collider: Collider::ball(Self::TOWER_COLLIDER_RADIUS),
            ..Default::default()
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
    const INNER_POSITIONS_DIR: [Vec2; 4] = [
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, -1.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(-1.0, 0.0),
    ];

    const OUTER_DISTANCE: f32 = 1.0;
    const OUTER_POSITIONS_DIR: [Vec2; 4] = [
        Vec2::new(
            std::f32::consts::FRAC_1_SQRT_2,
            std::f32::consts::FRAC_1_SQRT_2,
        ),
        Vec2::new(
            std::f32::consts::FRAC_1_SQRT_2,
            -std::f32::consts::FRAC_1_SQRT_2,
        ),
        Vec2::new(
            -std::f32::consts::FRAC_1_SQRT_2,
            -std::f32::consts::FRAC_1_SQRT_2,
        ),
        Vec2::new(
            -std::f32::consts::FRAC_1_SQRT_2,
            std::f32::consts::FRAC_1_SQRT_2,
        ),
    ];

    fn rnd_two_of_vec<T>(slice: &mut [T]) -> [&T; 2] {
        slice.shuffle(&mut thread_rng());
        [&slice[0], &slice[1]]
    }

    pub fn spawn(commands: &mut Commands) {
        let main = commands.spawn(Towers::default()).id();

        let inner_pos = Self::rnd_two_of_vec(&mut Self::INNER_POSITIONS_DIR.clone())
            .map(|pos| *pos * Self::INNER_DISTANCE);
        let outer_pos = Self::rnd_two_of_vec(&mut Self::OUTER_POSITIONS_DIR.clone())
            .map(|pos| *pos * Self::OUTER_DISTANCE);
        let positions = [&inner_pos[..], &outer_pos[..]].concat();

        for pos in positions.iter() {
            let tower = commands.spawn(TowerBundle::new(pos)).id();
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
struct TowerHitEvent {
    tower: Entity,
    players_hit: Vec<Entity>,
}

pub fn towers_mechanic(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut ev_tower_hit: EventWriter<TowerHitEvent>,
    mut main_query: Query<(Entity, &mut Towers, &Children)>,
    mut towers: Query<(&mut TowerComponent, &GlobalTransform, &Collider)>,
) {
    let delta_time = time.delta();

    for (main_entity, mut main_towers, children) in main_query.iter_mut() {
        main_towers.warmup_timer.tick(delta_time);

        if main_towers.warmup_timer.finished() {
            for &tower in children.iter() {
                if let Ok((mut tower_component, tower_global_transform, tower_collider)) =
                    towers.get_mut(tower)
                {
                    tower_component.countdown.tick(delta_time);

                    // TODO: check if mechanic was done properly.
                    // Do I want some kind of external class for this?
                    // Maybe use Events and push the results with some data?
                    let global_translation = tower_global_transform.translation();
                    let shape_pos = Vec2::new(global_translation.x, global_translation.y);
                    let shape_rot = tower_global_transform
                        .compute_transform()
                        .rotation
                        .to_euler(bevy::prelude::EulerRot::XYZ)
                        .2;
                    let shape = tower_collider;
                    let filter = QueryFilter::default()
                        .exclude_collider(tower)
                        .exclude_solids();

                    let mut players_hit = vec![];
                    rapier_context.intersections_with_shape(
                        shape_pos,
                        shape_rot,
                        shape,
                        filter,
                        |entity| {
                            players_hit.push(entity);
                            true
                        },
                    );

                    ev_tower_hit.send(TowerHitEvent { tower, players_hit });

                    if tower_component.countdown.finished() {
                        commands.entity(tower).despawn();
                    }
                }
            }

            if children.is_empty() {
                // All towers are gone now. Despawn.
                commands.entity(main_entity).despawn();
            }
        }
    }
}
