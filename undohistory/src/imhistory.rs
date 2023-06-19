// allow dead_code when checking the lib without tests
// cargo-analyzer runs `cargo check` for lib the lib with and without tests.
// dead_code warnings in the ide won't show if the code is used in tests.
#![cfg_attr(not(test), allow(dead_code))]

pub(crate) mod immutable {
    use im::Vector;
    pub(crate) struct UndoHistory<T> {
        history: Vector<T>,
        current: usize,
    }
    impl<T> UndoHistory<T>
    where
        T: Clone,
    {
        pub fn new() -> Self {
            UndoHistory {
                history: Vector::new(),
                current: 0,
            }
        }
        fn get(&self, index: usize) -> Option<T> {
            if !self.history.is_empty() && index < self.history.len() {
                return self.history.get(index).cloned();
            }
            return None;
        }
        // save a new state to the history, saves the current state first
        pub fn save(&mut self, value: T) {
            // if the current index is in the past, push the current value to the front first
            if self.current > 0 {
                let current = self.get(self.current);
                self.history.push_front(current.unwrap());
            }
            self.history.push_front(value);
            self.current = 0;
        }
        pub fn current(&self) -> Option<T> {
            if self.history.is_empty() {
                return None;
            }
            self.get(self.current)
        }
        pub fn undo(&mut self) -> Option<T> {
            if self.current < self.history.len() {
                self.current += 1;
            }
            self.current()
        }
        pub fn redo(&mut self) -> Option<T> {
            if self.current > 0 {
                self.current -= 1;
            }
            self.current()
        }
        pub fn load(&mut self, index: usize) -> Option<T> {
            if index < self.history.len() {
                self.current = index;
            }
            self.current()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::immutable::UndoHistory;
    #[test]
    fn undo_redo() {
        let mut history = UndoHistory::new();
        assert_eq!(history.current(), None);
        history.save("z".to_string());
        assert_eq!(history.current().unwrap(), "z".to_string());
        history.undo();
        assert_eq!(history.current(), None);
        history.redo();
        assert_eq!(history.current().unwrap(), "z".to_string());
        history.undo();
        assert_eq!(history.current(), None);
        history.redo();
        assert_eq!(history.current().unwrap(), "z".to_string());
        history.redo();
        assert_eq!(history.current().unwrap(), "z".to_string());
        history.save("y".to_string());
        assert_eq!(history.current().unwrap(), "y".to_string());
        history.undo();
        history.undo();
        assert_eq!(history.current(), None);
        history.redo();
        assert_eq!(history.current().unwrap(), "z".to_string());
    }
    #[test]
    fn select() {
        let mut history = UndoHistory::new();
        assert_eq!(history.current(), None);
        history.save("z".to_string());
        assert_eq!(history.current().unwrap(), "z".to_string());
        history.save("y".to_string());
        assert_eq!(history.current().unwrap(), "y".to_string());
        history.load(0);
        assert_eq!(history.current().unwrap(), "z".to_string());
        history.load(1);
        assert_eq!(history.current().unwrap(), "y".to_string());
        history.load(2);
        assert_eq!(history.current(), None);
    }
}
