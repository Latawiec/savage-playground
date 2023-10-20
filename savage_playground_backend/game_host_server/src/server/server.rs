use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{atomic::AtomicU64, Arc, RwLock},
};



use tokio::{
    sync::broadcast,
    task::JoinHandle,
};
use warp::Filter;

use super::{
    client::ClientID,
    room::{RoomHandle, RoomID}, message::{ServerMessage, ClientMessage},
};

#[derive(Clone)]
pub enum ServerNotification {
    RoomCreated{ room_id: RoomID, client_id: ClientID },
    RoomEmpty{ room_id: RoomID },
    RoomClosed{ room_id: RoomID },
}

#[derive(Clone)]
pub struct ServerHandle {
    pub addr: SocketAddr,
    server_notification_sender: broadcast::Sender<ServerNotification>,
    rooms: Arc<RwLock<HashMap<RoomID, RoomHandle>>>,
}

mod error {
    use warp::{http::StatusCode, reject::Reject, reply::Reply, Rejection};

    #[derive(Debug)]
    pub enum Error {
        RoomAlreadyExists,
        RoomDoesNotExist,
        RoomIsFull,

        LockError,
    }

    impl Reject for Error {}

    pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
        if let Some(_error) = r.find::<Error>() {
            Ok(warp::reply::with_status(
                "NOT_IMPLEMENTED",
                StatusCode::FORBIDDEN,
            ))
        } else {
            Err(r)
        }
    }
}

impl ServerHandle {
    const SERVER_NOTIFICATION_CHANNEL_CAPACITY: usize = 128;

    pub fn get_room_channels(&self, room_id: RoomID) -> Option<(broadcast::Receiver<ClientMessage>, broadcast::Sender<ServerMessage>)> {
        if let Some(room) = self.rooms.read().unwrap().get(&room_id) {
            Some((room.receiver(), room.sender()))
        } else {
            None
        }
    }

    pub fn get_room_handle(&self, room_id: RoomID) -> Option<RoomHandle> {
        if let Some(room_handle) = self.rooms.read().unwrap().get(&room_id) {
            Some(room_handle.clone())
        } else {
            None
        }
    }

    pub fn check_room_exists(&self, room_id: RoomID) -> bool {
        if let Some(_room_handle) = self.rooms.read().unwrap().get(&room_id) {
            true
        } else {
            false
        }
    }

    pub fn insert_room_handle(&self, room_id: RoomID, room_handle: RoomHandle) -> Result<(), error::Error> {
        match self.rooms.write() {
            Err(error) => {
                tracing::error!("Couldn't lock rooms for insertion: {:?}", error);
                Err(error::Error::LockError)
            },
            Ok(mut lock) => {
                lock.insert(room_id, room_handle);
                Ok(())
            },
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ServerNotification> {
        self.server_notification_sender.subscribe()
    }

    pub fn close_room(&self, room_id: u64) -> bool {
        self.rooms.write().unwrap().remove(&room_id).is_some()
    }

    pub fn new(server_address: SocketAddr) -> (ServerHandle, JoinHandle<()>) {

        let room_id_gen: Arc<AtomicU64> = Default::default();
        let client_id_gen: Arc<AtomicU64> = Default::default();
        let (server_notification_sender, _) = broadcast::channel::<ServerNotification>(Self::SERVER_NOTIFICATION_CHANNEL_CAPACITY);

        let room_id_gen_filter =
            warp::any().map(move || { room_id_gen.fetch_add(1, std::sync::atomic::Ordering::Relaxed) });
        let client_id_gen_filter = warp::any()
            .map(move || client_id_gen.fetch_add(1, std::sync::atomic::Ordering::Relaxed));

        let server_handle = ServerHandle {
            addr: server_address,
            server_notification_sender,
            rooms: Default::default(),
        };

        let server_handle_clone = server_handle.clone();
        let server_handle_filter = warp::any().map(move || server_handle_clone.clone());

        let join_room = warp::path!("join" / RoomID)
            .and(client_id_gen_filter.clone())
            .and(warp::ws())
            .and(warp::addr::remote())
            .and(server_handle_filter.clone())
            .and_then(Self::join_room)
            .recover(error::return_error);

        let create_room = warp::path!("create")
            .and(room_id_gen_filter)
            .and(client_id_gen_filter.clone())
            .and(warp::ws())
            .and(warp::addr::remote())
            .and(server_handle_filter.clone())
            .and_then(Self::create_room)
            .recover(error::return_error);

        let routes = join_room.or(create_room);

        let server_join_handle = tokio::spawn(async move {
            warp::serve(routes).bind(server_address).await;
        });

        (server_handle, server_join_handle)
    }

    async fn join_room(
        room_id: RoomID,
        client_id: ClientID,
        ws: warp::ws::Ws,
        addr: Option<SocketAddr>,
        server_handle: ServerHandle,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if let None = server_handle.get_room_handle(room_id) {
            return Err(warp::reject::custom(error::Error::RoomDoesNotExist));
        }
        // Between first check and the next one room might disappear. I think... We'll see
        Ok(ws.on_upgrade(move |websocket| async move {
            let room_handle = { 
                if let Some(room_handle) = server_handle.get_room_handle(room_id) {
                    Some(room_handle.clone())
                } else {
                    None
                }
            };
            if let Some(room_handle) = room_handle {
                let _ = room_handle.create_room_client(client_id, addr, websocket).await;
            } else {
                tracing::error!("Room does not exist.");
                return;
            }
        }))
    }

    async fn create_room(
        room_id: RoomID,
        client_id: ClientID,
        ws: warp::ws::Ws,
        addr: Option<SocketAddr>,
        server_handle: ServerHandle,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        {
            if server_handle.check_room_exists(room_id) {
                return Err(warp::reject::custom(error::Error::RoomAlreadyExists));
            }

            let new_room = RoomHandle::new(room_id);
            server_handle.insert_room_handle(room_id, new_room.clone());
            let _ = server_handle.server_notification_sender.send(ServerNotification::RoomCreated { room_id, client_id });

            tokio::spawn(Self::empty_room_watch(server_handle.clone(), new_room.clone()));
        }

        Self::join_room(
            room_id,
            client_id,
            ws,
            addr,
            server_handle,
        ).await
    }

    async fn empty_room_watch(server_handle: ServerHandle, room_handle: RoomHandle) {
        let mut receiver = room_handle.receiver();
        while let Ok(msg) = receiver.recv().await {
            match msg {
                ClientMessage::LeftRoom { client_id: _, room_id } => {
                    if room_handle.is_empty() {
                        let _ = server_handle.server_notification_sender.send(ServerNotification::RoomEmpty { room_id });
                    }
                },
                _ => {},
            }
        }
    }
}
