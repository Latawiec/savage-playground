use std::time;

use bevy::{ecs::system::NonSendMut, prelude::{EventReader, EventWriter}};

use super::{
    event::{GameInputMessage, GameOutputMessage}, time_sliced_io::TimeSlicedIO
};

pub fn io_exchange_system(
    mut ev_input_message: EventWriter<GameInputMessage>,
    mut ev_output_message: EventReader<GameOutputMessage>,
    mut io: NonSendMut<TimeSlicedIO>,
) {
    for out_message in ev_output_message.read() {
        io.stdout(out_message.0.clone());
    }
    io.run_for(time::Duration::from_millis(5));

    while let Some(in_message) = io.stdin() {
        ev_input_message.send(GameInputMessage(in_message));
    }
}
