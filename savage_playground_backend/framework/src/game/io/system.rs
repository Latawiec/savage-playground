use std::str::FromStr;

use bevy::prelude::{EventReader, ResMut, EventWriter};
use serde::Deserialize;
use uuid::Uuid;

use crate::{io::event::GameInputMessage, game::io::input::InputStateFlags};

use super::{event::PlayerInputEvent, resource::PlayerInputManager, input::KeyFlag};

#[derive(Deserialize)]
#[serde(tag = "type")]
enum GameMessageType {
    Input {
        keys_down: Option<KeyFlag>,
    },
}

#[derive(Deserialize)]
struct GameMessage {
    client_id: u64,
    #[serde(flatten)]
    message: GameMessageType,
}

pub fn raw_input_parse_system(
    mut ev_input_message: EventReader<GameInputMessage>,
    mut ev_player_input: EventWriter<PlayerInputEvent>,
) {
    for msg in ev_input_message.iter() {
        let str_msg = String::from_utf8_lossy(&msg.0);
        let game_msg: Result<GameMessage, serde_json::Error> = serde_json::from_str(&str_msg);
        
        if let Err(err) = game_msg {
            tracing::error!("Ill formatted game input: {:?}", err);
            continue;
        };
        let game_msg = game_msg.unwrap();

        match &game_msg.message {
            GameMessageType::Input { keys_down } => {
                if keys_down.is_none() { continue; }
                let keys_down = keys_down.unwrap();
                let keys_state: InputStateFlags = keys_down;

                let player_id = Uuid::from_u64_pair(game_msg.client_id, game_msg.client_id);

                ev_player_input.send(PlayerInputEvent {
                    player_id: player_id,
                    new_state: keys_state,
                    timestamp: std::time::Duration::default(),
                });
            },
        }
    }
}

pub fn input_register_system(
    mut ev_player_input: EventReader<PlayerInputEvent>,
    mut res_player_input: ResMut<PlayerInputManager>,
) {
    for ev in ev_player_input.iter() {
        res_player_input.set_input_state(ev.player_id, ev.new_state);
    }
}
