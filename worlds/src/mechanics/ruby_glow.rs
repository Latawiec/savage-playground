use std::time::Duration;

use bevy::{
    prelude::{
        BuildChildren, Bundle, Children, Commands, Component, Entity, Plugin, Query, Res,
        Transform, Update, Vec2, Vec3, With,
    },
    time::{Time, Timer, TimerMode},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::RapierContext;
use framework::{
    components::{collision::aoe::AreaOfEffectBundle, lifetime::SelfDestruct},
    types::environment::WorldDirection,
    utils::rand::{rnd_two_of_vec, rnd_one_of_vec},
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct Config {
    pub seed: ChaCha8Rng,
}

impl Config {
    pub fn from_seed(seed: u64) -> Self {
        Config {
            seed: ChaCha8Rng::seed_from_u64(seed),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_seed(rand::random())
    }
}

#[derive(Component)]
pub struct ExplodeTopaz {
    explosion_timer: Timer,
}

impl ExplodeTopaz {
    pub fn new(countdown: Duration) -> Self {
        ExplodeTopaz {
            explosion_timer: Timer::new(countdown, TimerMode::Once),
        }
    }
    pub fn tick(&mut self, delta_time: Duration) {
        self.explosion_timer.tick(delta_time);
    }
    pub fn finished(&self) -> bool {
        self.explosion_timer.finished()
    }
    pub fn just_finished(&self) -> bool {
        self.explosion_timer.just_finished()
    }
}

#[derive(Bundle)]
pub struct ExplodeTopazBundle {
    topaz: ExplodeTopaz,
    aoe: AreaOfEffectBundle,
    destruct: SelfDestruct,
}

#[derive(Component)]
pub struct PoisonTopaz {
    pre_grow_timer: Timer,
    grow_timer: Timer,
}

impl PoisonTopaz {
    pub fn new(warmup: Duration, growth: Duration) -> Self {
        PoisonTopaz {
            pre_grow_timer: Timer::new(warmup, TimerMode::Once),
            grow_timer: Timer::new(growth, TimerMode::Once),
        }
    }
    pub fn tick(&mut self, delta_time: Duration) {
        self.pre_grow_timer.tick(delta_time);
        if self.pre_grow_timer.finished() {
            self.grow_timer.tick(delta_time);
        }
    }
    pub fn growing(&self) -> bool {
        self.pre_grow_timer.finished()
    }
    pub fn grow_progress(&self) -> f32 {
        self.grow_timer.percent()
    }
    pub fn finished(&self) -> bool {
        self.grow_timer.finished()
    }
    pub fn just_finished(&self) -> bool {
        self.grow_timer.just_finished()
    }
}

#[derive(Bundle)]
pub struct PoisonTopazBundle {
    topaz: PoisonTopaz,
    aoe: AreaOfEffectBundle,
    destruct: SelfDestruct,
}

#[derive(Component)]
pub struct RubyGlowOne;

impl RubyGlowOne {
    const POISON_FAR_DISTANCE: f32 = 250.0;
    const POISON_NEAR_DISTANCE: f32 = 100.0;
    const POISON_POSITIONS_DIR: [WorldDirection; 4] = WorldDirection::INTERCARDINALS;
    const POISON_START_RADIUS: f32 = 40.0;
    const POISON_END_RADIUS: f32 = 250.0;
    const POISON_GROWTH_START: Duration = Duration::new(5, 0);
    const POISON_GROWTH_DURATION: Duration = Duration::new(12, 0);
    const POISON_POISON_LIFETIME: Duration = Duration::new(20, 0);

    const EXPLOSION_DISTANCE: f32 = 200.0;
    const EXPLOSION_SPAN: f32 = 140.0;
    const EXPLOSION_POSITIONS_DIR: [WorldDirection; 4] = WorldDirection::INTERCARDINALS;
    const EXPLOSION_COUNTODOWN: Duration = Duration::new(13, 0);
    const EXPLOSION_LIFETIME: Duration = Duration::new(13, 0);
    // const EXPLOSION_CRYSTAL_POSITIONS_DIR: [WorldDirection; 4] // Make it relative to quadrant center.

    fn explode_topaz(position: Vec2) -> ExplodeTopazBundle {
        let transform =
            TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 0.0));

        ExplodeTopazBundle {
            topaz: ExplodeTopaz::new(Self::EXPLOSION_COUNTODOWN),
            aoe: AreaOfEffectBundle::rectangle(Self::EXPLOSION_SPAN, Self::EXPLOSION_SPAN)
                .set_transform(transform),
            destruct: SelfDestruct::new(Self::EXPLOSION_LIFETIME),
        }
    }

    fn poison_topaz(position: Vec2) -> PoisonTopazBundle {
        let transform =
            TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 0.0));

        PoisonTopazBundle {
            topaz: PoisonTopaz::new(Self::POISON_GROWTH_START, Self::POISON_GROWTH_DURATION),
            aoe: AreaOfEffectBundle::circle(Self::POISON_START_RADIUS).set_transform(transform),
            destruct: SelfDestruct::new(Self::POISON_POISON_LIFETIME),
        }
    }

    pub fn spawn(commands: &mut Commands, config_opt: Option<Config>) -> Entity {
        let mut seed = config_opt.unwrap_or(Config::default()).seed;

        let main = commands
            .spawn(RubyGlowOne)
            .insert(TransformBundle::default())
            .id();

        // Explosion
        let explosion_positions = {
            let mut positions = Self::EXPLOSION_POSITIONS_DIR.clone();
            let picked_positions = rnd_two_of_vec(&mut positions, &mut seed);

            for pos in picked_positions {
                let position = pos.vec() * Self::EXPLOSION_DISTANCE;
                let explosion = commands.spawn(Self::explode_topaz(position)).id();

                commands.entity(main).add_child(explosion);
            }
            [*picked_positions[0], *picked_positions[1]]
        };

        // Poison
        {
            // If explosions are on the diagonal, Poisons just need to pick the other diagonal.
            let [close_dir, far_dir] = {
                if explosion_positions[0].is_opposite(&explosion_positions[1]) {
                    // Take the other diagonal then.
                    [
                        explosion_positions[0].perpendicular_clockwise(),
                        explosion_positions[1].perpendicular_clockwise(),
                    ]
                } else {
                    // We can simply shuffle.
                    let mut positions = explosion_positions.clone();
                    let &one = rnd_one_of_vec(&mut positions, &mut seed);
                    [one, one.opposite()]
                }
            };

            let close_position = close_dir.vec() * Self::POISON_NEAR_DISTANCE;
            let far_position = far_dir.vec() * Self::POISON_FAR_DISTANCE;

            let close = commands.spawn(Self::poison_topaz(close_position)).id();
            let far = commands.spawn(Self::poison_topaz(far_position)).id();

            commands.entity(main).add_child(close).add_child(far);
        }

        main
    }

    fn update(
        time: Res<Time>,
        rapier_context: Res<RapierContext>,
        mut commands: Commands,
        mut query_ruby_glow: Query<(Entity, &Children), With<RubyGlowOne>>,
        mut query_explosions: Query<(Entity, &mut ExplodeTopaz)>,
        mut query_poisons: Query<(Entity, &mut PoisonTopaz, &mut Transform)>,
    ) {
        let delta_time = time.delta();

        for (root_entity, topaz_crystals) in query_ruby_glow.iter_mut() {
            if topaz_crystals.is_empty() {
                // This means children have despawned. Finish up and remove root as well.
                commands.entity(root_entity).despawn();
                continue;
            }

            for &topaz in topaz_crystals.iter() {
                // Explosion
                if let Ok((entity, mut explode_topaz)) = query_explosions.get_mut(topaz) {
                    explode_topaz.tick(delta_time);

                    if explode_topaz.just_finished() {
                        for (_, hitbox, intersecting) in rapier_context.intersections_with(entity) {
                            if intersecting {
                                println!("{:?} Exploded", hitbox);
                            }
                        }
                    }
                }

                // Poison
                if let Ok((entity, mut poison_topaz, mut transform)) = query_poisons.get_mut(topaz)
                {
                    poison_topaz.tick(delta_time);

                    let scale_factor = (Self::POISON_START_RADIUS
                        + poison_topaz.grow_progress()
                            * (Self::POISON_END_RADIUS - Self::POISON_START_RADIUS))
                        / Self::POISON_START_RADIUS;
                    transform.scale = Vec3::splat(scale_factor);

                    for (_, hitbox, intersecting) in rapier_context.intersections_with(entity) {
                        if intersecting {
                            println!("{:?} in Poison", hitbox);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct RubyGlowPlugin;
impl Plugin for RubyGlowPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, RubyGlowOne::update); // .add_systems(PostUpdate, self_destruct_system);
    }
}
