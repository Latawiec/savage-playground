use bevy::{app::{MainScheduleOrder, Plugin, Update}, ecs::schedule::ScheduleLabel, prelude::{IntoSystemConfigs, States}, time::TimePlugin};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use game_plugins::time_sliced_io::plugin::{IOPlugin, InputRead};

use crate::{input::event::FFXIVGameInputEvent, player::system::{controller_input::player_controller_input_system, player_motion::player_motion_system}, settings::{event::{PlayerJoinedEvent, PlayerLeftEvent}, resource::GameSettings, system::settings_input_system}};

use crate::input::system::ffxiv_game_input_system;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FfxivToolkitInputRead;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FfxivGameState {
    InitializationScreen,
    LoadingScreen,
    InGame,
    SummaryScreen,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FfxivPauseState {
    Paused,
    Running
}

pub struct FfxivToolkitPlugin;
impl Plugin for FfxivToolkitPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if !app.is_plugin_added::<TimePlugin>() {
            app.add_plugins(TimePlugin);
        }

        if !app.is_plugin_added::<RapierPhysicsPlugin::<NoUserData>>() {
            app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        }

        app
            .add_plugins(IOPlugin)
            ;

        let mut scheduler = app.world_mut().resource_mut::<MainScheduleOrder>();
        scheduler.insert_after(InputRead, FfxivToolkitInputRead);

        app.add_event::<FFXIVGameInputEvent>();
        app.add_event::<PlayerJoinedEvent>();
        app.add_event::<PlayerLeftEvent>();

        app.insert_resource(GameSettings::default());

        app
            .add_systems(
                FfxivToolkitInputRead,
                (
                    ffxiv_game_input_system,
                    player_controller_input_system.after(ffxiv_game_input_system),
                    settings_input_system.after(ffxiv_game_input_system)
                )
            )
            .add_systems(Update, player_motion_system);
    }
}