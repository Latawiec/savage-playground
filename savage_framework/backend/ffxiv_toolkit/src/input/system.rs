use bevy::prelude::{EventReader, EventWriter};
use game_plugins::time_sliced_io::event::ClientInputEvent;
use prost::Message;

use crate::proto::ffxiv_toolkit::FfxivGameInput;

use super::event::FFXIVGameInputEvent;

const FFXIV_GAME_INPUT_TYPE_URL: &'static str = "savage_playgrounds/ffxiv_game_input";

pub fn ffxiv_game_input_system(
    mut ev_game_input_reader: EventReader<ClientInputEvent>,
    mut ev_ffxiv_game_input_writer: EventWriter<FFXIVGameInputEvent>
) {
    for game_input in ev_game_input_reader.read() {
        let game_input = &game_input.0;
        let sender_id = game_input.sender_id;
        let message = &game_input.game_input_message;

        if let Some(message) = message {
            if message.type_url == FFXIV_GAME_INPUT_TYPE_URL {
                match FfxivGameInput::decode(message.value.as_slice()) {
                    Ok(input_data) => {
                        let ffxiv_input_event = FFXIVGameInputEvent {
                            player_id: sender_id,
                            input_data,
                        };
                        ev_ffxiv_game_input_writer.send(ffxiv_input_event);
                    },
                    Err(error) => {
                        tracing::warn!("Couldn't decode input message: {}", error);
                        continue;
                    },
                }
            }
        }
    }
}