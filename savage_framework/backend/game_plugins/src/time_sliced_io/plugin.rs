use bevy::{app::{MainScheduleOrder, PostUpdate, PreUpdate}, ecs::schedule::ScheduleLabel, prelude::Plugin};

use super::{
    event::{ClientInputEvent, GameOutputEvent},
    system::{client_input_system, game_output_system, io_exchange_system},
};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct InputRead;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct OutputWrite;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct IoExchange;

#[derive(Default)]
pub struct IOPlugin;
impl Plugin for IOPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_schedule(InputRead)
            .init_schedule(OutputWrite)
            .init_schedule(IoExchange);

        let mut scheduler = app.world_mut().resource_mut::<MainScheduleOrder>();
        scheduler.insert_after(PreUpdate, InputRead);
        scheduler.insert_after(PostUpdate, OutputWrite);
        scheduler.insert_after(OutputWrite, IoExchange);

        app
            .add_event::<ClientInputEvent>()
            .add_event::<GameOutputEvent>()
            .add_systems(InputRead, client_input_system)
            .add_systems(OutputWrite, game_output_system)
            .add_systems(IoExchange, io_exchange_system);
    }
}
