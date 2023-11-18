use std::{
    os::fd::AsRawFd,
    time::Duration, io::{BufRead, Read, BufWriter, self},
};

use mio::{Interest, Poll, Token, Events, event, unix::SourceFd};

fn main() {
    //println!("Hello! Let's go!\n");
    let mut lines = io::stdin().lock().lines();
    // lines.for_each(|a| println!("{:?}", a));

    while let Some(_read_bytes) = lines.next() {
        // println!("Read: {:?}", _read_bytes);
        println!("{}", _read_bytes.unwrap());
        // let _ = io::stdout().write_all(_read_bytes.unwrap());
        // input.clear();
    }

    // let mut events = Events::with_capacity(128);

    // let mut poll = Poll::new().unwrap();
    // let stdin_fd = std::io::stdin().as_raw_fd();
    // let mut source_fd = SourceFd(&stdin_fd);

    // let _ = poll
    //     .registry()
    //     .register(&mut source_fd, Token(0), Interest::READABLE);

    // loop {
    //     let _ = poll.poll(&mut events, Some(Duration::from_millis(3)));
    //     // println!("Polled: {:?}", events);
    //     if events.is_empty() {
    //         // println!("Sleep...");
    //         std::thread::sleep(Duration::from_secs(2));
    //     } else {
    //         for event in events.iter() {
    //             let mut input = String::new();
    //             let mut stdin_lock = std::io::stdin().lock();
    //             let _ = stdin_lock.read(unsafe { input.as_bytes_mut() } );
    //             // println!("Read: {}", input);
    //             input.clear();
    //         }
    //     }
    // }

}
