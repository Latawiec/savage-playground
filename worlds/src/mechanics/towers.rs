use bevy::{
    prelude::{
        BuildChildren, Children, Commands, Entity, EventReader,
        EventWriter, Local, Parent, Plugin, PostUpdate, Query, Res, Transform, Update, Vec3, With, Component, Bundle, Event,
    },
    time::{Time, Timer, TimerMode},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::RapierContext;
use framework::{
    components::{
        collision::{hitbox::PlayerHitboxTag, aoe::AreaOfEffectBundle},
        lifetime::SelfDestruct,
        player::identity::Identity,
    },
    systems::lifetime::self_destruct_system,
    types::environment::WorldDirection,
    utils::{locals::EntitySetTracket, rand::rnd_two_of_vec},
};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::{collections::BTreeSet, time::Duration};

#[derive(Component, Default)]
pub struct PoisonPool;

#[derive(Bundle)]
pub struct PoisonPoolBundle {
    pool: PoisonPool,
    aoe: AreaOfEffectBundle,
    destruct: SelfDestruct,
}

impl PoisonPoolBundle {
    const POSION_POOL_RADIUS: f32 = 25.0;
    const POISON_POOL_LIFETIME: Duration = Duration::new(10, 0);
}
impl Default for PoisonPoolBundle {
    fn default() -> Self {
        Self {
            pool: PoisonPool::default(),
            aoe: AreaOfEffectBundle::circle(Self::POSION_POOL_RADIUS),
            destruct: SelfDestruct::new(Self::POISON_POOL_LIFETIME),
        }
    }
}

#[derive(Component)]
pub struct TowerHit {
    impact_timer: Timer,
}
impl TowerHit {
    pub fn new(countdown: Duration) -> Self {
        TowerHit {
            impact_timer: Timer::new(countdown, TimerMode::Once),
        }
    }
    pub fn tick(&mut self, delta_time: Duration) {
        self.impact_timer.tick(delta_time);
    }
    pub fn finished(&self) -> bool {
        self.impact_timer.finished()
    }
    pub fn just_finished(&self) -> bool {
        self.impact_timer.just_finished()
    }
}

#[derive(Bundle)]
pub struct TowerHitBundle {
    tower_hit: TowerHit,
    aoe: AreaOfEffectBundle,
    destruct: SelfDestruct,
}

impl TowerHitBundle {
    const TOWER_HIT_RADIUS: f32 = 25.0;
    const TOWER_HIT_COUNTDOWN: Duration = Duration::new(10, 0);
    const TOWER_HIT_LIFETIME: Duration = Duration::new(10, 0);
}

impl Default for TowerHitBundle {
    fn default() -> Self {
        Self {
            tower_hit: TowerHit::new(Self::TOWER_HIT_COUNTDOWN),
            aoe: AreaOfEffectBundle::circle(Self::TOWER_HIT_RADIUS),
            destruct: SelfDestruct::new(Self::TOWER_HIT_LIFETIME),
        }
    }
}

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

pub enum TowerCircle {
    Inner,
    Outer,
}

#[derive(Component)]
struct TowerInfo {
    pub position: WorldDirection,
    pub circle: TowerCircle,
}

#[derive(Component)]
pub struct Tower {
    pub poison_pool_entity: Entity,
    pub tower_hit_entity: Entity,
}

impl Tower {
    pub fn spawn(
        commands: &mut Commands,
        transform: Transform,
        position: WorldDirection,
        circle: TowerCircle,
    ) -> Entity {
        let poison_pool_entity = commands.spawn(PoisonPoolBundle::default()).id();
        let tower_hit_entity = commands.spawn(TowerHitBundle::default()).id();

        commands
            .spawn(Tower {
                poison_pool_entity,
                tower_hit_entity,
            })
            .insert(TowerInfo { position, circle })
            .insert(TransformBundle::from_transform(transform))
            .add_child(tower_hit_entity)
            .add_child(poison_pool_entity)
            .id()
    }
}

#[derive(Component, Default)]
pub struct TowerSet;

impl TowerSet {
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

    pub fn spawn(commands: &mut Commands, config_opt: Option<Config>) -> Entity {
        let mut seed = config_opt.unwrap_or(Config::default()).seed;

        let main = commands
            .spawn(TowerSet::default())
            .insert(TransformBundle::default())
            .id();

        // Inner
        for pos in rnd_two_of_vec(&mut Self::INNER_POSITIONS_DIR.clone(), &mut seed) {
            let translation = pos.vec() * Self::INNER_DISTANCE;
            let tower = Tower::spawn(
                commands,
                Transform::from_translation(Vec3::new(translation.x, translation.y, 0.0)),
                *pos,
                TowerCircle::Inner,
            );
            commands.entity(main).add_child(tower);
        }

        // Outer
        for pos in rnd_two_of_vec(&mut Self::OUTER_POSITIONS_DIR.clone(), &mut seed) {
            let translation = pos.vec() * Self::OUTER_DISTANCE;
            let tower = Tower::spawn(
                commands,
                Transform::from_translation(Vec3::new(translation.x, translation.y, 0.0)),
                *pos,
                TowerCircle::Outer,
            );
            commands.entity(main).add_child(tower);
        }

        main
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
            .add_systems(Update, update)
            .add_systems(PostUpdate, self_destruct_system);
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
            }
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
            }
            Err(e) => {
                tracing::error!("Couldn't get players identity: {}", e);
            }
        }
    }

    for ev_hit in ev_hit.iter() {
        if let Ok((_, player)) = query_player_hitboxes.get(ev_hit.player_hitbox) {
            if let Ok(player_id) = query_player_identity.get(player.get()) {
                tracing::info!("Hit: {}", player_id.name);
            }
        }
    }

    for cured_player in &poisoned_players_tracker.0.just_removed {
        commands.entity(*cured_player).remove::<PoisonDebuff>();

        match query_player_identity.get(*cured_player) {
            Ok(player_identity) => {
                tracing::info!("Player \"{}\" cured!", player_identity.name);
            }
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
    mut query_tower_sets: Query<(Entity, &Children), With<TowerSet>>,
    query_towers: Query<(&Tower, &TowerInfo)>,
    query_poison_pools: Query<Entity, With<PoisonPool>>,
    mut query_tower_hits: Query<(Entity, &mut TowerHit)>,
) {
    let delta_time = time.delta();

    for (root_entity, towers) in query_tower_sets.iter_mut() {
        if towers.is_empty() {
            // This means children have despawned. Finish up and remove root as well.
            commands.entity(root_entity).despawn();
            continue;
        }

        for &tower in towers.iter() {
            if let Ok((tower_component, tower_info)) = query_towers.get(tower) {
                // Poison pools
                if let Ok(entity) = query_poison_pools.get(tower_component.poison_pool_entity) {
                    for (_, hitbox, intersecting) in rapier_context.intersections_with(entity) {
                        if intersecting {
                            ev_player_poison.send(PlayerPoisonEvent {
                                player_hitbox: hitbox,
                                position: tower_info.position,
                            });
                        }
                    }
                }

                // Tower Hits
                if let Ok((entity, mut tower_hit_component)) =
                    query_tower_hits.get_mut(tower_component.tower_hit_entity)
                {
                    tower_hit_component.tick(delta_time);

                    if tower_hit_component.just_finished() {
                        println!("Finished!");
                        for (_, hitbox, intersecting) in rapier_context.intersections_with(entity) {
                            if intersecting {
                                ev_player_hit.send(PlayerHitEvent {
                                    player_hitbox: hitbox,
                                    position: tower_info.position,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
