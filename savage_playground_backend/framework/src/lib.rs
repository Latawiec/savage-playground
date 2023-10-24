pub mod blueprints;
pub mod communication;
pub mod components;
pub mod resources;
pub mod systems;
pub mod types;
pub mod utils;
pub mod events;
pub mod io;

// #[cfg(debug_assertions)]
#[path = "./.debug/mod.rs"]
pub mod debug;
pub mod plugin;
