use bevy::prelude::Event;
use game_interface::proto::{game_input::ClientInput, game_output::GameMessage};

#[derive(Event)]
pub struct ClientInputEvent(pub ClientInput);

#[derive(Event)]
pub struct RoomInputEvent();

#[derive(Event)]
pub struct GameOutputEvent(pub GameMessage);
