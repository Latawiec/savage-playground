use bevy::{ecs::system::NonSendMut, prelude::{EventReader, EventWriter, NonSend, Res}};
use game_interface::proto::game_input::ClientInput;
use prost::Message;

use super::{
    event::{ClientInputEvent, GameOutputEvent}, resource::TimeSlicedIoConfig, time_sliced_io::TimeSlicedIO
};

pub fn client_input_system(
    _time_sliced_io_config: Res<TimeSlicedIoConfig>,
    mut ev_client_input_writer: EventWriter<ClientInputEvent>,
    mut io: NonSendMut<TimeSlicedIO>,
) {
    while let Some(client_input_buffer) = io.stdin() {
        match ClientInput::decode(client_input_buffer.as_slice()) {
            Ok(client_input) => {
                let client_input = ClientInputEvent(client_input);
                ev_client_input_writer.send(client_input);
            },
            Err(error) => {
                tracing::error!("Failed to decode client input: {}", error);
            },
        }
    }
}

pub fn game_output_system(
    _time_sliced_io_config: Res<TimeSlicedIoConfig>,
    mut ev_game_output_reader: EventReader<GameOutputEvent>,
    mut io: NonSendMut<TimeSlicedIO>,
) {
    for game_output in ev_game_output_reader.read() {
        let game_output_buffer = game_output.0.encode_to_vec();
        io.stdout(game_output_buffer);
    }
}

pub fn io_exchange_system(
    time_sliced_io_config: Res<TimeSlicedIoConfig>,
    io: NonSend<TimeSlicedIO>,
) {
    io.run_for(time_sliced_io_config.io_exchange_duration);
}