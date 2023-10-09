use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{atomic::AtomicU64, Arc}, time::Duration,
};

use tokio::{
    sync::{RwLock, broadcast},
    task::JoinHandle,
};
use warp::Filter;

use super::{
    client::ClientID,
    room::{RoomHandle, RoomID, self}, message::{ServerMessage, ClientMessage},
};

#[derive(Clone)]
pub enum ServerNotification {
    RoomCreated{ room_id: RoomID },
    RoomEmpty{ room_id: RoomID },
    RoomClosed{ room_id: RoomID },
}

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
    }

    impl Reject for Error {}

    pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
        if let Some(error) = r.find::<Error>() {
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

    pub async fn get_room_channels(&self, room_id: RoomID) -> Option<(broadcast::Receiver<ClientMessage>, broadcast::Sender<ServerMessage>)> {
        if let Some(room) = self.rooms.read().await.get(&room_id) {
            Some((room.receiver(), room.sender()))
        } else {
            None
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ServerNotification> {
        self.server_notification_sender.subscribe()
    }

    pub async fn close_room(&self, room_id: u64) -> bool {
        self.rooms.write().await.remove(&room_id).is_some()
    }

    pub fn new(server_address: SocketAddr) -> (ServerHandle, JoinHandle<()>) {

        let rooms: Arc<RwLock<HashMap<RoomID, RoomHandle>>> = Default::default();
        let room_id_gen: Arc<AtomicU64> = Default::default();
        let client_id_gen: Arc<AtomicU64> = Default::default();
        let (server_notification_sender, _) = broadcast::channel::<ServerNotification>(Self::SERVER_NOTIFICATION_CHANNEL_CAPACITY);

        let server_notification_sender_clone = server_notification_sender.clone();
        let rooms_clone = rooms.clone();

        let rooms_filter = warp::any().map(move || rooms_clone.clone());
        let room_id_gen_filter =
            warp::any().map(move || room_id_gen.clone().fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        let client_id_gen_filter = warp::any()
            .map(move || client_id_gen.clone().fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        let server_notification_sender_filter = warp::any().map(move|| server_notification_sender_clone.clone());

        let server_handle = ServerHandle {
            addr: server_address,
            server_notification_sender,
            rooms,
        };

        let join_room = warp::path!("join" / RoomID)
            .and(client_id_gen_filter.clone())
            .and(warp::ws())
            .and(warp::addr::remote())
            .and(rooms_filter.clone())
            .and(server_notification_sender_filter.clone())
            .and_then(Self::join_room)
            .recover(error::return_error);

        let create_room = warp::path!("create")
            .and(client_id_gen_filter.clone())
            .and(room_id_gen_filter.clone())
            .and(warp::ws())
            .and(warp::addr::remote())
            .and(rooms_filter.clone())
            .and(server_notification_sender_filter.clone())
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
        rooms: Arc<RwLock<HashMap<RoomID, RoomHandle>>>,
        server_notification_sender: broadcast::Sender<ServerNotification>
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if let None = rooms.read().await.get(&room_id) {
            return Err(warp::reject::custom(error::Error::RoomDoesNotExist));
        }
        // Between first check and the next one room might disappear. I think... We'll see
        Ok(ws.on_upgrade(move |websocket| async move {
            // let mut receiver;
            // let mut sender;
            if let Some(room_handle) = rooms.read().await.get(&room_id) {
                let _ = room_handle.create_room_client(client_id, addr, websocket).await;
                // receiver = room_handle.receiver();
                // sender = room_handle.sender();
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
        rooms: Arc<RwLock<HashMap<RoomID, RoomHandle>>>,
        server_notification_sender: broadcast::Sender<ServerNotification>
    ) -> Result<impl warp::Reply, warp::Rejection> {
        {
            let mut rooms_lock = rooms.write().await;
            if rooms_lock.contains_key(&room_id) {
                return Err(warp::reject::custom(error::Error::RoomAlreadyExists));
            } else {
                let new_room = RoomHandle::new(room_id);
                let room_msg_receiver = new_room.receiver();
                rooms_lock.insert(room_id, new_room);
                let _ = server_notification_sender.send(ServerNotification::RoomCreated { room_id });

                let rooms_clone = rooms.clone();
                let server_notification_sender_clone = server_notification_sender.clone();
                let mut receiver = room_msg_receiver;

                tokio::spawn(async move {

                    while let Ok(msg) = receiver.recv().await {
                        match msg {
                            ClientMessage::Connected { client_id } => {},
                            ClientMessage::Disconnected { client_id } => {},
                            ClientMessage::JoinedRoom { client_id, room_id } => {},
                            ClientMessage::LeftRoom { client_id, room_id } => {
                                let rooms_lock = rooms_clone.read().await;
                                if let Some(room) = rooms_lock.get(&room_id) {
                                    if room.is_empty().await {
                                        let _ = server_notification_sender_clone.send(ServerNotification::RoomEmpty { room_id });
                                    }
                                }
                            },
                            ClientMessage::String { client_id, text } => {},
                            ClientMessage::Binary { client_id, data } => {},
                        }
                    }
                });
            }
        }

        Self::join_room(
            room_id,
            client_id,
            ws,
            addr,
            rooms,
            server_notification_sender
        ).await
    }
}
