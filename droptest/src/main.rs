extern crate basedrop;

use basedrop::{Collector, Shared};
fn main() {
    println!("running basedrop test");
    let (tx_send, rx_send) = std::sync::mpsc::channel::<Shared<i32>>();
    let (tx_return, rx_return) = std::sync::mpsc::channel::<Shared<i32>>();
    let collector_handle = std::thread::spawn(move || {
        let max = 100;
        let mut current = 0;
        let mut collector = Collector::new();
        let data = current;
        let state = Shared::new(&collector.handle(), data);
        loop {
            match tx_send.send(state.clone()) {
                Ok(_) => {
                    println!("ptr #{} sent from collector thread", current);
                }
                Err(_) => println!("failed to send ptr from collector thread"),
            }
            rx_return.try_iter().for_each(|p| {
                current += 1;
                println!("ptr #{} returned to collector thread", p.to_string());
            });
            if current >= max {
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
            if res.is_ok() {
                let ptr = res.unwrap();
                println!("ptr #{} recieved in user thread", ptr.to_string());
                match tx_return.send(ptr) {
                    Ok(_) => {
                        count += 1;
                        println!("user thread returned ptr #{}", count);
                        if count >= 100 {
                            break;
                        }
                    }
                    Err(_) => println!("user thread failed to return ptr"),
                }
            } else {
                println!("no ptr received in user thread");
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
