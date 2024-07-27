use bevy::{app::{MainScheduleOrder, Plugin}, ecs::schedule::ScheduleLabel, prelude::IntoSystemConfigs};
use game_plugins::time_sliced_io::plugin::{IOPlugin, InputRead};

use crate::{input::event::FFXIVGameInputEvent, player::system::controller_input::player_controller_input_system};

use crate::input::system::ffxiv_game_input_system;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FfxivToolkitInputRead;

pub struct FfxivToolkitPlugin;
impl Plugin for FfxivToolkitPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(IOPlugin);

        let mut scheduler = app.world_mut().resource_mut::<MainScheduleOrder>();
        scheduler.insert_after(InputRead, FfxivToolkitInputRead);

        app.add_event::<FFXIVGameInputEvent>();

        app.add_systems(
            FfxivToolkitInputRead,
            (
                ffxiv_game_input_system,
                player_controller_input_system.after(ffxiv_game_input_system)
            )
        );
    }
}