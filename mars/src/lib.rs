pub struct Store<Action, State> {
    dispatch: std::sync::mpsc::Sender<Action>,
    state: State,
}

impl<Action, State> Store<Action, State> {
    pub fn new(state: State) -> Store<Action, State> {
        let (tx, _rx) = std::sync::mpsc::channel::<Action>();
        Store {
            dispatch: tx,
            state,
        }
    }
    pub fn update(&self, action: Action) {
        self.dispatch.send(action).unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_create_store() {
        enum Action {}
        let s = Vec::<usize>::new();
        let store = Some(Store::<Action, Vec<usize>>::new(s));
        assert!(store.is_some());
    }

    #[test]
    fn can_dispatch_same_thread() {
        assert_eq!(false, true);
    }

    #[test]
    fn can_dispatch_different_thread() {
        assert_eq!(false, true);
    }
}
