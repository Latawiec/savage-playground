use bevy::prelude::Event;
use game_interface::proto::{game_input::{ClientInput, RoomInput}, game_output::GameMessage};

#[derive(Event)]
pub struct ClientInputEvent(pub ClientInput);

#[derive(Event)]
pub struct RoomInputEvent(pub RoomInput);

#[derive(Event)]
pub struct GameOutputEvent(pub GameMessage);
