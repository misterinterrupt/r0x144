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
        pub fn insert(&mut self, key: String, value: T) {
            self.history.push_back(self.data.clone());
            self.data.clone().insert(key, value);
        }
        pub fn remove(&mut self, key: &str) {
            self.history.push_back(self.data.clone());
            self.data.remove(key);
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
        // return an immutable copy of the data
        pub fn reader(&self) -> &HashMap<String, T> {
            &self.data
        }
    }
    impl<T> Clone for State<T>
    where
        T: Clone + Send + Sync + PartialEq + Eq,
    {
        fn clone(&self) -> Self {
            Self {
                history: self.history.clone(),
                data: self.data.clone(),
            }
        }
    }
    impl<T> Default for State<T>
    where
        T: Clone + Send + Sync + PartialEq + Eq,
    {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use im::HashMap;

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
    fn insert_makes_history() {
        let mut state = State::new();
        state.insert("a".to_string(), 1);
        assert_eq!(state.hist_len(), 1);
        state.insert("a".to_string(), 2);
        assert_eq!(state.hist_len(), 2);
        state.insert("a".to_string(), 3);
        assert_eq!(state.hist_len(), 3);
    }
    #[test]
    fn remove_makes_history() {
        let mut state = State::new();
        state.insert("a".to_string(), 1);
        state.insert("b".to_string(), 2);
        state.insert("c".to_string(), 3);
        state.remove("a");
        assert_eq!(state.hist_len(), 4);
        state.remove("b");
        assert_eq!(state.hist_len(), 5);
        state.remove("c");
        assert_eq!(state.hist_len(), 6);
    }

    #[test]
    fn write_thread_read_thread() {
        // create a new state with a static lifetime
        let mut state = State::<i32>::new();
        state.insert("t1".to_string(), 1);
        state.insert("t2".to_string(), 2);
        state.insert("t3".to_string(), 3);
        state.insert("t4".to_string(), 4);
        std::thread::scope(|scope| {
            // create a mutable reference to the state to send to the other thread
            let ref_state = &mut state;
            let (tx1, rx1) = std::sync::mpsc::channel::<Arc<&HashMap<String, i32>>>();

            let _t1 = scope
                .spawn(move || {
                    // mutate the state
                    ref_state.insert("t1a".to_string(), 5);
                    ref_state.insert("t2a".to_string(), 6);
                    ref_state.insert("t3a".to_string(), 7);
                    ref_state.insert("t4a".to_string(), 8);
                    // send a reader to the other thread
                    tx1.send(Arc::new(ref_state.reader())).unwrap();
                })
                .join()
                .unwrap();
            let _t2 = scope
                .spawn(move || loop {
                    match rx1.recv() {
                        Ok(data) => {
                            if let Some(v) = data.get("t1a") {
                                assert_eq!(*v, 5);
                            }
                            if let Some(v) = data.get("t2a") {
                                assert_eq!(*v, 6);
                            }
                            if let Some(v) = data.get("t3a") {
                                assert_eq!(*v, 7);
                            }
                            if let Some(v) = data.get("t4a") {
                                assert_eq!(*v, 8);
                            }
                            break;
                        }
                        Err(_) => break,
                    }
                })
                .join()
                .unwrap();
        });
    }
}
