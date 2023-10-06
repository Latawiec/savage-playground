use std::time::Duration;

use config::ProgramConfig;
use server::ServerHandle;
use tracing_subscriber::fmt::format::FmtSpan;


mod config;
mod server;

fn main() {
    // let config = ProgramConfig::parse_config();
    // if let Err(error) = config {
    //     println!("{}", error.0);
    //     return;
    // }

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    let rt_handle = rt.handle();

    let (server_handle, join_handle) = ServerHandle::new(rt_handle);

    let mut receiver = server_handle.subscribe();
    let sender = server_handle.server_msg_sender;

    rt_handle.spawn(async move {
        while let Ok(msg) = receiver.recv().await {
            tracing::info!("Got message {:?}", msg);
        }
    });

    rt_handle.spawn(async move {
        loop {
            match sender.send(server::ServerMessage::Broadcast) {
                Ok(_) => {},
                Err(e) => {
                    println!("{}", e);
                },
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    rt.block_on(async{ 
        let _ = tokio::join!(join_handle);
    });
}