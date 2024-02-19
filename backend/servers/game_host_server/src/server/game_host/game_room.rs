use super::{
    handle_gen::HandleGenerator,
    types::{ClientHandle, RoomHandle},
};
use crate::{
    game_launcher::game_instance::game_instance::GameInstance,
    server::connection::client_connection::{
        ClientConnection, ClientConnectionHandle, ConnectionCloseHandle,
    },
};
use rocket_ws::stream::DuplexStream;
use room_server_interface::{
    proto::{client_input::ClientInput, client_output::ClientOutput},
    schema::game_config::GameConfig,
};
use std::{collections::BTreeMap, sync::Mutex};

pub struct GameRoom {
    room_handle: RoomHandle,
    game_room_config: GameConfig,
    game_instance: GameInstance,
    client_handle_gen: HandleGenerator<ClientHandle>,
    client_input_sender: tokio::sync::mpsc::Sender<ClientInput>,
    client_input_receiver: tokio::sync::mpsc::Receiver<ClientInput>,
    client_output_senders: Mutex<BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<ClientOutput>>>,
    open_connections: Mutex<BTreeMap<ClientHandle, ClientConnectionHandle>>,
}

impl GameRoom {
    const CLIENT_MESSAGE_RECEIVER_CAPACITY: usize = 1024;
    const CLIENT_MESSAGE_SENDER_CAPACITY: usize = 128;

    pub fn new(
        room_handle: RoomHandle,
        game_instance: GameInstance,
        game_room_config: GameConfig,
    ) -> GameRoom {
        let (client_input_sender, client_input_receiver) =
            tokio::sync::mpsc::channel(Self::CLIENT_MESSAGE_RECEIVER_CAPACITY);
        GameRoom {
            room_handle,
            game_room_config,
            game_instance,
            open_connections: Default::default(),
            client_handle_gen: Default::default(),
            client_input_sender,
            client_input_receiver,
            client_output_senders: Default::default(),
        }
    }

    pub fn connect(&self, ws_stream: DuplexStream) -> ConnectionCloseHandle {
        let client_handle = self.client_handle_gen.next();
        let (output_sender, output_receiver) =
            tokio::sync::mpsc::channel::<ClientOutput>(Self::CLIENT_MESSAGE_SENDER_CAPACITY);
        let input_sender = self.client_input_sender.clone();

        let (client_connection_handle, close_handle) =
            ClientConnection::start(client_handle, ws_stream, input_sender, output_receiver);

        {
            let mut lock = self.client_output_senders.lock().unwrap();
            lock.insert(client_handle, output_sender);
        }
        {
            let mut lock = self.open_connections.lock().unwrap();
            lock.insert(client_handle, client_connection_handle);
        }

        close_handle
    }
}
