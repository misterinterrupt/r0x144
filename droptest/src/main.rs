extern crate basedrop;

use basedrop::{Collector, Owned, Shared};
fn main() {
    println!("running basedrop test");
    println!("making a Collector");
    let collector = Collector::new();
    println!("making an Owned value");
    let three: Owned<i32> = Owned::new(&collector.handle(), 3);
    println!("making a Shared value");
    let sha: Shared<&str> = Shared::new(&collector.handle(), "sha");
    println!("making a thread");
    let handle_1 = std::thread::spawn(move || {
        loop {
            println!("Shared value in thread loop: {}", sha.chars().as_str());
        }
    });
}

#[cfg(test)]
mod test {
    #[test]
    fn do_it() {
        assert_eq!(2, 2);
    }
}
