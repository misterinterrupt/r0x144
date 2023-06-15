// allow dead_code when checking the lib without tests
// cargo-analyzer runs `cargo check` for lib the lib with and without tests.
// dead_code warnings in the ide won't show if the code is used in tests.
#![cfg_attr(not(test), allow(dead_code))]

extern crate basedrop;

use basedrop::{Collector, Shared};
fn main() {
    println!("running basedrop test");
    let (tx_send, rx_send) = std::sync::mpsc::channel::<Shared<i32>>();
    let max = 100;
    let writer_handle = std::thread::spawn(move || {
        let mut current = 1;
        let mut collector = Collector::new();
        loop {
            let data = current;
            let state = Shared::new(&collector.handle(), data);
            match tx_send.send(state.clone()) {
                Ok(_) => {
                    println!("{} sent from collector thread", *state);
                    current += 1;
                }
                Err(_) => println!("failed to send ptr from collector thread"),
            }
            if current > max {
                println!("all shared ptrs sent to collector thread");
                break;
            }
            collector.collect();
        }
    });
    let reader_handle = std::thread::spawn(move || loop {
        let res: Result<Shared<i32>, std::sync::mpsc::TryRecvError> = rx_send.try_recv().to_owned();
        if let Ok(ptr) = res {
            println!("{} recieved in user thread", *ptr);
            if *ptr >= max {
                println!("all shared ptrs recieved in user thread");
                std::process::exit(1);
            }
        }
    });
    writer_handle.join().unwrap();
    reader_handle.join().unwrap();
    std::process::exit(1);
}

#[cfg(test)]
mod test {
    #[test]
    fn do_it() {
        assert_eq!(2, 2);
    }
}
