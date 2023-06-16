// allow dead_code when checking the lib without tests
// cargo-analyzer runs `cargo check` for lib the lib with and without tests.
// dead_code warnings in the ide won't show if the code is used in tests.
#![cfg_attr(not(test), allow(dead_code))]

mod imtest {
    use im::{HashMap, Vector};
    use serde::{Deserialize, Serialize};
    #[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
    pub struct State<T>
    where
        T: Clone + Send + Sync,
    {
        history: Vector<HashMap<String, T>>,
        data: HashMap<String, T>,
    }
    impl<T> State<T>
    where
        T: Clone + Send + Sync + PartialEq + Eq,
    {
        pub fn new() -> Self {
            Self {
                history: Vector::new(),
                data: HashMap::new(),
            }
        }
        pub fn insert(&self, key: String, value: T) {
            self.history.clone().push_back(self.data.clone());
            self.data.clone().insert(key, value);
        }
        pub fn len(&self) -> usize {
            self.data.len()
        }
        pub fn hist_len(&self) -> usize {
            self.history.len()
        }
        pub fn get(&self, key: &str) -> Option<T> {
            self.data.get(key).cloned()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::imtest::State;

    #[test]
    fn ser_de() {
        let state = State::<i32>::new();
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "{\"history\":[],\"data\":{}}");
        let state: State<i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(state.len(), 0);
    }

    #[test]
    fn insert_makes_history_after_one() {
        let state = State::new();
        state.insert("a".to_string(), 1);
        assert_eq!(state.hist_len(), 1);
        state.insert("a".to_string(), 2);
        assert_eq!(state.hist_len(), 2);
        state.insert("a".to_string(), 3);
        assert_eq!(state.hist_len(), 3);
    }

    #[test]
    fn send_sync_two_thread_edit() {
        let state = Arc::new(State::new());
        state.insert("t1a".to_string(), 1);
        state.insert("t1b".to_string(), 2);
        state.insert("t2a".to_string(), 3);
        state.insert("t2b".to_string(), 4);
        let s1 = state.clone();
        let s2 = s1.clone();
        let s3 = s1.clone();
        let s4 = s1.clone();

        let (tx1, rx1) = std::sync::mpsc::channel::<Arc::<State<i32>>>();

        let t1 = std::thread::spawn(move || {
            s1.insert("t1a".to_string(), 5);
            s3.insert("t1b".to_string(), 6);
            tx1.send(s1).unwrap();
        });
        let t2 = std::thread::spawn(move || {
            s2.insert("t2a".to_string(), 7);
            s4.insert("t2b".to_string(), 8);
            match rx1.try_recv() {
                Ok(state) => {
                    assert_eq!(state.get("t1a").unwrap(), 5);
                    assert_eq!(state.get("t1b").unwrap(), 6);
                }
                Err(_) => {}
            }
        });
        (t1.join().unwrap(), t2.join().unwrap());
        assert_eq!(state.get("t1a").unwrap(), 5);
        assert_eq!(state.get("t1b").unwrap(), 6);
        assert_eq!(state.get("t2a").unwrap(), 7);
        assert_eq!(state.get("t2b").unwrap(), 8);
    }
}
