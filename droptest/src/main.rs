extern crate basedrop;

use basedrop::{Collector, Shared};
fn main() {
    println!("running basedrop test");
    let (tx_send, rx_send) = std::sync::mpsc::channel::<Shared<i32>>();
    let max = 100;
    let collector_handle = std::thread::spawn(move || {
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
                println!("all shared ptrs returned to collector thread");
                std::process::exit(1);
            }
            collector.collect();
        }
    });
    let user_handle = std::thread::spawn(move || loop {
        let res: Result<Shared<i32>, std::sync::mpsc::TryRecvError> = rx_send.try_recv().to_owned();
        if let Ok(ptr) = res {
            let data = *ptr;
            println!("{} recieved in user thread", data);
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
