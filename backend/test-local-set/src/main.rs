use tokio::{task, time};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    

    let local = task::LocalSet::new();

    local.spawn_local(async move {
        let mut stdin = tokio::io::stdin();
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(128, 0);

        loop {
            let result = stdin.read(&mut buffer).await;
            match result {
                Ok(len_read) => {
                    let read = &buffer[0..len_read];
                    let res = String::from_utf8(read.to_vec());
                    println!("{:?}", res);
                },
                Err(error) => {
                    println!("{:?}", error);
                },
            }
        }
    });

    local.spawn_local(async move {
        loop {
            time::sleep(time::Duration::from_millis(5000)).await;
            println!("Task 2. working! Doing even crazier IO!");
        }
    });

    loop {
        local.run_until(time::sleep(time::Duration::from_millis(1000))).await;
        time::sleep(time::Duration::from_millis(1000)).await;
    }
}
