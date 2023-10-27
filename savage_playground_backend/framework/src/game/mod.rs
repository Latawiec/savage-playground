pub mod aggro;
pub mod blueprints;
pub mod collision;
pub mod common;
pub mod lifetime;
pub mod player;
pub mod rendering;
pub mod world;

// #[cfg(debug_assertions)]
#[path = "./.debug/mod.rs"]
pub mod debug;