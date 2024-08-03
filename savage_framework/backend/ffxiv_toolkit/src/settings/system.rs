use bevy::prelude::{EventReader, EventWriter, ResMut};
use game_plugins::time_sliced_io::event::RoomInputEvent;

use super::{event::{PlayerJoinedEvent, PlayerLeftEvent}, resource::GameSettings};

pub fn settings_input_system(
    mut ev_room_input_reader: EventReader<RoomInputEvent>,
    mut ev_player_joined_writer: EventWriter<PlayerJoinedEvent>,
    mut ev_player_left_writer: EventWriter<PlayerLeftEvent>,
    mut settings: ResMut<GameSettings>,
) {
    for room_input in ev_room_input_reader.read() {
        let room_input = &room_input.0;
        if let Some(game_master_id) = room_input.game_master_id {
            settings.game_master_id = Some(game_master_id);
        }

        for joined_player_id in &room_input.players_joined {
            ev_player_joined_writer.send(PlayerJoinedEvent { player_id: joined_player_id.clone() });
        }
        
        for left_player_id in &room_input.players_left {
            ev_player_left_writer.send(PlayerLeftEvent { player_id: left_player_id.clone() });
        }
    }
}