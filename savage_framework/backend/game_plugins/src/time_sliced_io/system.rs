use bevy::{ecs::system::NonSendMut, prelude::{EventReader, EventWriter, NonSend, Res}};
use game_interface::proto::game_input::{ClientInput, GameInput};
use prost::Message;

use super::{
    event::{ClientInputEvent, GameOutputEvent, RoomInputEvent}, resource::TimeSlicedIoConfig, time_sliced_io::TimeSlicedIO
};

pub fn game_input_system(
    _time_sliced_io_config: Res<TimeSlicedIoConfig>,
    mut ev_client_input_writer: EventWriter<ClientInputEvent>,
    mut ev_room_input_writer: EventWriter<RoomInputEvent>,
    mut io: NonSendMut<TimeSlicedIO>,
) {
    while let Some(game_input_buffer) = io.stdin() {
        match GameInput::decode(game_input_buffer.as_slice()) {
            Ok(game_input) => {
                if let Some(client_input) = game_input.client_input {
                    let client_input = ClientInputEvent(client_input);
                    ev_client_input_writer.send(client_input);
                }
                if let Some(_room_input) = game_input.room_input {
                    let room_input = RoomInputEvent();
                    ev_room_input_writer.send(room_input);
                }
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
    tracing::info!("Running IO Exchange for {:?}", time_sliced_io_config.io_exchange_duration);
    io.run_for(time_sliced_io_config.io_exchange_duration);
}