use bevy::prelude::{EventReader, Query, With};

use crate::{
    input::event::FFXIVGameInputEvent,
    player::component::{PlayerController, PlayerInfo, PlayerTag},
};

pub fn player_controller_input_system(
    mut ev_ffxiv_game_input_writer: EventReader<FFXIVGameInputEvent>,
    mut query_players: Query<(&mut PlayerController, &PlayerInfo), With<PlayerTag>>,
) {
    for game_input in ev_ffxiv_game_input_writer.read() {
        if let Some(input_actions) = game_input.input_data.input_actions_set {
            let player_id = game_input.player_id;

            for (mut player_controller, player_info) in query_players.iter_mut() {
                if player_info.player_id == player_id {
                    let input_diff = input_actions ^ player_controller.input_pressed;
                    let just_pressed = input_actions & input_diff;
                    let just_released = (!input_actions) & input_diff;

                    player_controller.input_just_pressed = just_pressed;
                    player_controller.input_just_released = just_released;
                    player_controller.input_pressed = input_actions;
                }
            }
        }
    }
}
