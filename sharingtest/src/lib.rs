pub mod sharingtest {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::thread::{self, JoinHandle};

    pub struct App {
        audio_thread_handle: Option<JoinHandle<()>>,
        // ui_thread_handle: Option<JoinHandle<()>>,
        shared_state: Arc<Vec<i32>>,
        should_stop: Arc<AtomicBool>,
    }

    impl App {
        pub fn new(state: Vec<i32>) -> App {
            App {
                audio_thread_handle: None,
                // ui_thread_handle: None,
                shared_state: Arc::new(state),
                should_stop: Arc::new(AtomicBool::new(false)),
            }
        }

        pub fn run(&mut self) {
            let shared_state = self.shared_state.clone();
            let should_stop = self.should_stop.clone();
            if self.audio_thread_handle.is_some() {
                return;
            };
            self.audio_thread_handle.replace(thread::spawn(move || {
                while !should_stop.load(Ordering::Relaxed) {
                    // Audio thread logic here
                    let shared_state_ref = shared_state.as_ref();
                    // Use shared_state_ref as a read-only reference to the shared state
                    println!("state: {:?}", shared_state_ref);
                }
            }));
        }

        pub fn stop(&mut self) {
            self.should_stop.store(true, Ordering::Relaxed);
            match self.audio_thread_handle.take() {
                Some(handle) => handle.join().unwrap(),
                None => (),
            }
        }
        // swap the state atomically
        pub fn swap_state(&mut self, new_state: Vec<i32>) {
            let shared_state_mut = Arc::make_mut(&mut self.shared_state);
            *shared_state_mut = new_state;
        }
        pub fn get_state(&self) -> &Vec<i32> {
            self.shared_state.as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sharingtest::*;
    #[test]
    fn share_swap_immutable_state() {
        let state: Vec<i32> = [23, 42, 14].to_vec();
        let mut app = App::new(state);
        app.run();
        app.swap_state([14, 23, 42].to_vec());
        app.swap_state([42, 14, 23].to_vec());
        app.stop();
        assert_eq!(&[42, 14, 23].to_vec(), app.get_state())
    }
}
