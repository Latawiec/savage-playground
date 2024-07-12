use super::{classes::Class, roles::Role};
use bevy::prelude::Component;

#[derive(Component)]
pub enum Job {
    // Tanks
    PLD,
    WAR,
    DRK,
    GNB,

    // Healers
    WHM,
    AST,
    SCH,
    SGE,

    // Melee
    MNK,
    DRG,
    NIN,
    SAM,
    RPR,

    // Physical
    BRD,
    MCH,
    DNC,

    // Magical
    BLM,
    SMN,
    RDM,
    BLU,
}

// Tanks
pub const PALADIN: (Class, Role, Job) = (Class::Tank, Role::Tank, Job::PLD);
pub const WARRIOR: (Class, Role, Job) = (Class::Tank, Role::Tank, Job::WAR);
pub const DARK_KNIGHT: (Class, Role, Job) = (Class::Tank, Role::Tank, Job::DRK);
pub const GUNBREAKER: (Class, Role, Job) = (Class::Tank, Role::Tank, Job::GNB);

// Healers
pub const WHITE_MAGE: (Class, Role, Job) = (Class::PureHealer, Role::Healer, Job::WHM);
pub const ASTROLOGIAN: (Class, Role, Job) = (Class::PureHealer, Role::Healer, Job::AST);
pub const SCHOLAR: (Class, Role, Job) = (Class::ShieldHealer, Role::Healer, Job::SCH);
pub const SAGE: (Class, Role, Job) = (Class::ShieldHealer, Role::Healer, Job::SGE);

// Melee
pub const MONK: (Class, Role, Job) = (Class::MeleeDamage, Role::DPS, Job::MNK);
pub const DRAGOON: (Class, Role, Job) = (Class::MeleeDamage, Role::DPS, Job::DRG);
pub const NINJA: (Class, Role, Job) = (Class::MeleeDamage, Role::DPS, Job::NIN);
pub const SAMURAI: (Class, Role, Job) = (Class::MeleeDamage, Role::DPS, Job::SAM);
pub const REAPER: (Class, Role, Job) = (Class::MeleeDamage, Role::DPS, Job::RPR);

// Physical
pub const BARD: (Class, Role, Job) = (Class::PhysicalRangedDamage, Role::DPS, Job::BRD);
pub const MACHINIST: (Class, Role, Job) = (Class::PhysicalRangedDamage, Role::DPS, Job::MCH);
pub const DANCER: (Class, Role, Job) = (Class::PhysicalRangedDamage, Role::DPS, Job::DNC);

// Magical
pub const BLACK_MAGE: (Class, Role, Job) = (Class::MagicalRangedDamage, Role::DPS, Job::BLM);
pub const SUMMONER: (Class, Role, Job) = (Class::MagicalRangedDamage, Role::DPS, Job::SMN);
pub const RED_MAGE: (Class, Role, Job) = (Class::MagicalRangedDamage, Role::DPS, Job::RDM);
pub const BLUE_MAGE: (Class, Role, Job) = (Class::MagicalRangedDamage, Role::DPS, Job::BLU);
