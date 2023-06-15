mod imtest {
    use im::{HashMap, Vector};
    use serde::{Deserialize, Serialize};
    #[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
    pub struct State<T: Clone> {
        history: Vector<HashMap<String, T>>,
        data: HashMap<String, T>,
    }
    impl<T> State<T>
    where
        T: Clone + PartialEq + Eq,
    {
        pub fn new() -> Self {
            Self {
                history: Vector::new(),
                data: HashMap::new(),
            }
        }
        pub fn insert(&mut self, key: String, value: T) {
            self.history.push_back(self.data.clone());
            self.data.insert(key, value);
        }
        pub fn len(&self) -> usize {
            self.data.len()
        }
        pub fn hist_len(&self) -> usize {
            self.history.len()
        }
    }
}

#[cfg(test)]
mod tests {
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
        let mut state = State::new();
        state.insert("a".to_string(), 1);
        assert_eq!(state.hist_len(), 1);
        state.insert("a".to_string(), 2);
        assert_eq!(state.hist_len(), 2);
        state.insert("a".to_string(), 3);
        assert_eq!(state.hist_len(), 3);
    }
}
