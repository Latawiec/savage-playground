use std::time::Duration;

use bevy::{app::{MainScheduleOrder, PostUpdate, PreUpdate}, ecs::schedule::ScheduleLabel, prelude::Plugin, time::TimePlugin};

use super::{
    event::{ClientInputEvent, GameOutputEvent, RoomInputEvent}, resource::TimeSlicedIoConfig, system::{game_input_system, game_output_system, io_exchange_system}, time_sliced_io::TimeSlicedIO
};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputRead;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutputWrite;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct IoExchange;

#[derive(Default)]
pub struct IOPlugin;
impl Plugin for IOPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_schedule(InputRead)
            .init_schedule(OutputWrite)
            .init_schedule(IoExchange);

        app
            .insert_resource(TimeSlicedIoConfig {
                io_exchange_duration: Duration::from_millis(3),
            })
            .insert_non_send_resource(TimeSlicedIO::default());
            

        let mut scheduler = app.world_mut().resource_mut::<MainScheduleOrder>();
        scheduler.insert_after(PreUpdate, InputRead);
        scheduler.insert_after(PostUpdate, OutputWrite);
        scheduler.insert_after(OutputWrite, IoExchange);

        app
            .add_event::<ClientInputEvent>()
            .add_event::<RoomInputEvent>()
            .add_event::<GameOutputEvent>()
            .add_systems(InputRead, game_input_system)
            .add_systems(OutputWrite, game_output_system)
            .add_systems(IoExchange, io_exchange_system);
    }
}
