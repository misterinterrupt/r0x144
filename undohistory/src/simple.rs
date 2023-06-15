pub mod simple {
    use std::fmt::Debug;
    /*
     manages a very simple undo history of immutable clones of a generic type T
    */
    pub struct UndoHistory<T> {
        history: Vec<T>,
        pub(crate) current: usize,
    }
    impl<T> UndoHistory<T>
    where
        T: Clone + Debug,
    {
        pub fn new(initial: T) -> UndoHistory<T> {
            UndoHistory {
                history: vec![initial],
                current: 0,
            }
        }
        pub fn current(&self) -> &T {
            &self.history[self.current]
        }
        pub fn undo(&mut self) {
            if self.current > 0 {
                self.current -= 1;
            }
        }
        pub fn redo(&mut self) {
            if self.current < self.history.len() - 1 {
                self.current += 1;
            }
        }
        pub fn push(&mut self, new: T) {
            self.history.truncate(self.current + 1);
            self.history.push(new);
            self.current = self.history.len() - 1;
        }
        pub fn len(&self) -> usize {
            self.history.len()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::simple::UndoHistory;
    #[test]
    fn test_undo_history() {
        let mut history = UndoHistory::new(0);
        assert_eq!(*history.current(), 0);
        history.push(1);
        assert_eq!(*history.current(), 1);
        history.push(2);
        assert_eq!(*history.current(), 2);
        history.undo();
        assert_eq!(*history.current(), 1);
        history.undo();
        assert_eq!(*history.current(), 0);
        history.undo();
        assert_eq!(*history.current(), 0);
        history.redo();
        assert_eq!(*history.current(), 1);
        history.redo();
        assert_eq!(*history.current(), 2);
        history.redo();
        assert_eq!(*history.current(), 2);
    }
}
