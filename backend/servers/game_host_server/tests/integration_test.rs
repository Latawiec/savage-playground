
mod component_test;

#[tokio::test]
async fn simple_direct_msg() {
    // Given
    // let config = Config {
    //     game_host_server_port: 3000,
    //     asset_server_port: 3001,
    //     game_dir_mapping_file: "integration_test.rs".to_owned(),
    //     server_name: "testing".to_owned(),
    // };
    // let server = BundledServer::new_with_config(config).unwrap();
    // let server_join = tokio::spawn(async {
    //     println!("Starting server");
    //     server.serve().await
    // });

    // let ws_fut = connect_async("ws://127.0.0.1:3000/create?a=1&a=12&hello=World");
    // let ws_check = tokio::spawn( async move{
    //     let (ws_stream, _) = ws_fut.await.expect("Failed to connect");
    //     let (mut write, mut read) = ws_stream.split();

    //     write.send(Message::Text("Helloooo".to_owned())).await.unwrap();

    //     while let Some(msg) = read.next().await {
    //         println!("{:?}", msg);
    //     }
    // });

    // When

    // Then
    // _ = tokio::join!(ws_check, server_join);
}
