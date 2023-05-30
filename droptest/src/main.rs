extern crate basedrop;

use basedrop::{Collector, Shared};
fn main() {
    println!("running basedrop test");
    let (tx_send, rx_send) = std::sync::mpsc::channel::<Shared<i32>>();
    let (tx_return, rx_return) = std::sync::mpsc::channel::<Shared<i32>>();
    let max = 100;
    let collector_handle = std::thread::spawn(move || {
        let mut current = 0;
        let mut collector = Collector::new();
        let mut returned = 0;
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
            rx_return.try_iter().for_each(|p| {
                returned += 1;
                println!("{} returned to collector thread", *p);
            });
            if returned >= max {
                println!("all shared ptrs returned to collector thread");
                break;
            }
            collector.collect();
        }
    });
    let user_handle = std::thread::spawn(move || {
        let mut count = 0;
        loop {
            let res: Result<Shared<i32>, std::sync::mpsc::TryRecvError> =
                rx_send.try_recv().to_owned();
            if let Ok(ptr) = res {
                let data = *ptr;
                println!("{} recieved in user thread", data);
                match tx_return.send(ptr) {
                    Ok(_) => {
                        count += 1;
                        println!("user thread returned {}", data);
                        if count >= max {
                            break;
                        }
                    }
                    Err(_) => println!("user thread failed to return ptr"),
                }
            } else {
                // println!("no ptr received in user thread");
            }
        }
    });
    collector_handle.join().unwrap();
    user_handle.join().unwrap();
}

#[cfg(test)]
mod test {
    #[test]
    fn do_it() {
        assert_eq!(2, 2);
    }
}
