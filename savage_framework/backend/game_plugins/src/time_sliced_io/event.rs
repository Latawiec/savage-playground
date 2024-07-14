use bevy::prelude::Event;

#[derive(Event)]
pub struct GameInputMessage(pub Vec<u8>);

#[derive(Event)]
pub struct GameOutputMessage(pub Vec<u8>);
