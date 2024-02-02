use game_host_server::room_host::{room_host::RoomHost, traits::{room_host_info::{self, RoomHostInfo}, room_host_management::{self, RoomHostManagement}}, types::room_host_notification::{self, RoomHostNotification}};

#[tokio::test]
async fn create_unique_clients() {
    // Given
    let room_host = RoomHost::default();
    let room_host_manager = room_host.get_host_manager();
    let room_host_info = room_host.get_host_info();
    // When
    let first_client = room_host_manager.create_client().await;
    let second_client = room_host_manager.create_client().await;
    // Then
    assert!(first_client.is_ok());
    assert!(second_client.is_ok());
    let first_client = first_client.unwrap();
    let second_client = second_client.unwrap();

    assert_ne!(first_client, second_client);
    let clients = room_host_info.get_clients().await;
    assert!(clients.is_ok());
    let clients = clients.unwrap();
    assert!(clients.contains(&first_client));
    assert!(clients.contains(&second_client));
}

#[tokio::test]
async fn create_client_notification() {
    // Given
    let room_host = RoomHost::default();
    let room_host_manager = room_host.get_host_manager();
    let room_host_info = room_host.get_host_info();
    // When
    let room_host_info_subscriber = room_host_info.subscribe_host_info();
    let created_client = room_host_manager.create_client().await;
    // Then
    assert!(room_host_info_subscriber.is_ok());
    let room_host_info_notification = room_host_info_subscriber.unwrap().try_recv();
    
    assert!(room_host_info_notification.is_ok());
    let room_host_info_notification = room_host_info_notification.unwrap();

    if let RoomHostNotification::ClientCreated { client } = room_host_info_notification {
        assert!(created_client.is_ok());
        assert_eq!( created_client.unwrap(), client);
    } else {
        panic!("Unexpected notification type.");
    }
}

#[tokio::test]
async fn create_unique_rooms() {
    // Given
    let room_host = RoomHost::default();
    let room_host_manager = room_host.get_host_manager();
    let room_host_info = room_host.get_host_info();
    // When
    let first_room = room_host_manager.create_room().await;
    let second_room = room_host_manager.create_room().await;
    // Then
    assert!(first_room.is_ok());
    assert!(second_room.is_ok());
    let first_room = first_room.unwrap();
    let second_room = second_room.unwrap();

    assert_ne!(first_room, second_room);
    let rooms = room_host_info.get_rooms().await;
    assert!(rooms.is_ok());
    let rooms = rooms.unwrap();
    assert!(rooms.contains(&first_room));
    assert!(rooms.contains(&second_room));
}

#[tokio::test]
async fn create_room_notification() {
    // Given
    let room_host = RoomHost::default();
    let room_host_manager = room_host.get_host_manager();
    let room_host_info = room_host.get_host_info();
    // When
    let room_host_info_subscriber = room_host_info.subscribe_host_info();
    let created_room = room_host_manager.create_room().await;
    // Then
    assert!(room_host_info_subscriber.is_ok());
    let room_host_info_notification = room_host_info_subscriber.unwrap().try_recv();
    
    assert!(room_host_info_notification.is_ok());
    let room_host_info_notification = room_host_info_notification.unwrap();

    if let RoomHostNotification::RoomCreated { room } = room_host_info_notification {
        assert!(created_room.is_ok());
        assert_eq!( created_room.unwrap(), room);
    } else {
        panic!("Unexpected notification type.");
    }
}

#[tokio::test]
async fn join_room() {
    // Given
    let room_host = RoomHost::default();
    let room_host_manager = room_host.get_host_manager();
    let room_host_info = room_host.get_host_info();

    let client = room_host_manager.create_client().await.unwrap();
    let room = room_host_manager.create_room().await.unwrap();

    // When
    let result = room_host_manager.join_room(client, room).await;

    // Then
    assert!(result.is_ok());

    let room_clients = room_host_info.get_room_clients(room).await.unwrap();
    assert!(room_clients.contains(&client));

    let client_rooms = room_host_info.get_client_rooms(client).await.unwrap();
    assert!(client_rooms.contains(&room));
}

#[tokio::test]
async fn join_room_notification() {
    // Given
    let room_host = RoomHost::default();
    let room_host_manager = room_host.get_host_manager();
    let room_host_info = room_host.get_host_info();

    let created_client = room_host_manager.create_client().await.unwrap();
    let created_room = room_host_manager.create_room().await.unwrap();

    // When
    let room_host_info_subscriber = room_host_info.subscribe_host_info();
    let _result = room_host_manager.join_room(created_client, created_room).await.unwrap();

    // Then
    assert!(room_host_info_subscriber.is_ok());
    let room_host_info_notification = room_host_info_subscriber.unwrap().try_recv();

    assert!(room_host_info_notification.is_ok());
    let room_host_info_notification = room_host_info_notification.unwrap();

    if let RoomHostNotification::ClientJoined { room, client } = room_host_info_notification {
        assert_eq!(created_room, room);
        assert_eq!(created_client, client);
    } else {
        panic!("Unexpected notification type.");
    }
}
