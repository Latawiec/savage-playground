use bevy::prelude::{EventReader, EventWriter, Local};

use crate::io::interface::{IOInterface, PushVec};

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
        io_interface.write(&out_message.0);
    }

    let mut push_vec = PushVec::<u8>::default();
    io_interface.read(&mut push_vec);

    for in_message in push_vec.iter() {
        ev_input_message.send(GameInputMessage(in_message.to_owned()));
    }
}
