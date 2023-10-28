use bevy::prelude::{EventReader, EventWriter, Local};

use crate::io::interface::IOInterface;

use super::{
    event::{GameInputMessage, GameOutputMessage},
    interface::unnamed_pipes::UnnamedPipesGameIO,
};

pub fn io_exchange_system(
    mut ev_input_message: EventWriter<GameInputMessage>,
    mut ev_output_message: EventReader<GameOutputMessage>,
    mut io_interface: Local<UnnamedPipesGameIO>,
) {
    for out_message in ev_output_message.iter() {
        io_interface.write_msg(&out_message.0);
    }

    while let Some(in_message) = io_interface.read_msg() {
        ev_input_message.send(GameInputMessage(in_message.to_owned()));
    }
}
