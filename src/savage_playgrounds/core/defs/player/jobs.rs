use super::{classes::Class, roles::Role};

pub mod tags {
    use bevy::ecs::component::Component;

    // Tanks
    #[derive(Component)]
    pub struct PLD;
    #[derive(Component)]
    pub struct WAR;
    #[derive(Component)]
    pub struct DRK;
    #[derive(Component)]
    pub struct GNB;

    // Healers
    #[derive(Component)]
    pub struct WHM;
    #[derive(Component)]
    pub struct AST;
    #[derive(Component)]
    pub struct SCH;
    #[derive(Component)]
    pub struct SGE;

    // Melee
    #[derive(Component)]
    pub struct MNK;
    #[derive(Component)]
    pub struct DRG;
    #[derive(Component)]
    pub struct NIN;
    #[derive(Component)]
    pub struct SAM;
    #[derive(Component)]
    pub struct RPR;

    // Physical
    #[derive(Component)]
    pub struct BRD;
    #[derive(Component)]
    pub struct MCH;
    #[derive(Component)]
    pub struct DNC;

    // Magical
    #[derive(Component)]
    pub struct BLM;
    #[derive(Component)]
    pub struct SMN;
    #[derive(Component)]
    pub struct RDM;
    #[derive(Component)]
    pub struct BLU;
}

// Tanks
pub const PALADIN: (Class, Role, tags::PLD) = (Class::Tank, Role::Tank, tags::PLD);
pub const WARRIOR: (Class, Role, tags::WAR) = (Class::Tank, Role::Tank, tags::WAR);
pub const DARK_KNIGHT: (Class, Role, tags::DRK) = (Class::Tank, Role::Tank, tags::DRK);
pub const GUNBREAKER: (Class, Role, tags::GNB) = (Class::Tank, Role::Tank, tags::GNB);

// Healers
pub const WHITE_MAGE: (Class, Role, tags::WHM) = (Class::PureHealer, Role::Healer, tags::WHM);
pub const ASTROLOGIAN: (Class, Role, tags::AST) = (Class::PureHealer, Role::Healer, tags::AST);
pub const SCHOLAR: (Class, Role, tags::SCH) = (Class::ShieldHealer, Role::Healer, tags::SCH);
pub const SAGE: (Class, Role, tags::SGE) = (Class::ShieldHealer, Role::Healer, tags::SGE);

// Melee
pub const MONK: (Class, Role, tags::MNK) = (Class::MeleeDamage, Role::DPS, tags::MNK);
pub const DRAGOON: (Class, Role, tags::DRG) = (Class::MeleeDamage, Role::DPS, tags::DRG);
pub const NINJA: (Class, Role, tags::NIN) = (Class::MeleeDamage, Role::DPS, tags::NIN);
pub const SAMURAI: (Class, Role, tags::SAM) = (Class::MeleeDamage, Role::DPS, tags::SAM);
pub const REAPER: (Class, Role, tags::RPR) = (Class::MeleeDamage, Role::DPS, tags::RPR);

// Physical
pub const BARD: (Class, Role, tags::BRD) = (Class::PhysicalRangedDamage, Role::DPS, tags::BRD);
pub const MACHINIST: (Class, Role, tags::MCH) = (Class::PhysicalRangedDamage, Role::DPS, tags::MCH);
pub const DANCER: (Class, Role, tags::DNC) = (Class::PhysicalRangedDamage, Role::DPS, tags::DNC);

// Magical
pub const BLACK_MAGE: (Class, Role, tags::BLM) = (Class::MagicalRangedDamage, Role::DPS, tags::BLM);
pub const SUMMONER: (Class, Role, tags::SMN) = (Class::MagicalRangedDamage, Role::DPS, tags::SMN);
pub const RED_MAGE: (Class, Role, tags::RDM) = (Class::MagicalRangedDamage, Role::DPS, tags::RDM);
pub const BLUE_MAGE: (Class, Role, tags::BLU) = (Class::MagicalRangedDamage, Role::DPS, tags::BLU);
