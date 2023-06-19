// allow dead_code when checking the lib without tests
// cargo-analyzer runs `cargo check` for lib the lib with and without tests.
// dead_code warnings in the ide won't show if the code is used in tests.
#![cfg_attr(not(test), allow(dead_code))]

use std::{fmt::Debug, sync::mpsc};

pub trait Reducer<State, Action> {
    fn reduce(&self, state: State, action: Action) -> State;
}

impl<F, State, Action> Reducer<State, Action> for F
where
    F: Fn(State, Action) -> State,
{
    fn reduce(&self, state: State, action: Action) -> State {
        self(state, action)
    }
}

pub struct Store<Action, State, RootReducer> {
    actions: mpsc::Receiver<Action>,
    dispatcher: mpsc::Sender<Action>,
    state: State,
    pending_actions: Vec<Action>,
    root_reducer: RootReducer,
}

impl<Action, State, RootReducer> Store<Action, State, RootReducer>
where
    Action: Debug + Clone,
    State: Debug + Clone,
    RootReducer: Reducer<State, Action> + Send + Sync + 'static,
{
    pub fn new(root_reducer: RootReducer, state: State) -> Self {
        let (tx, rx) = mpsc::channel::<Action>();
        Store {
            actions: rx,
            dispatcher: tx,
            pending_actions: Vec::<Action>::new(),
            state,
            root_reducer,
        }
    }
    pub fn dispatch(&self, action: Action) -> Result<(), mpsc::SendError<Action>>
    where
        Action: Send + Sync,
    {
        println!("{:?} Dispatched", &action);
        self.dispatcher.send(action)
    }
    pub fn tick(&mut self) {
        self.pending_actions = self.actions.try_iter().collect::<Vec<Action>>();
        println!(
            "Store Update: {} actions in the queue",
            self.pending_actions.len()
        );
    }
    // called by owning thread to collect & process updates the state with the reducer
    pub fn update(&mut self) {
        while let Some(action) = self.pending_actions.pop() {
            self.state = self.root_reducer.reduce(self.state.clone(), action.clone());
            println!(
                "Store Update: {:?} applied to state. New state: {:?}",
                action, self.state
            );
        }
    }
}

#[cfg(test)]
mod tests {

    use std::thread;

    use super::*;

    #[test]
    fn can_create_store() {
        #[derive(Clone, Debug)]
        enum Action {}
        type R = fn(i32, Action) -> i32;
        let r: R = |state: i32, _: Action| state;
        let store = Some(Store::<Action, i32, R>::new(r, 0));
        assert!(store.is_some());
    }

    #[test]
    fn can_dispatch_different_thread() {
        #[derive(Clone, Debug)]
        enum Action {
            Add(i32),
            Remove(i32),
        }
        type R = fn(i32, Action) -> i32;
        let r: R = |state: i32, action: Action| match action {
            Action::Add(n) => state + n,
            Action::Remove(n) => state - n,
        };
        let mut store = Store::<Action, i32, R>::new(r, 0);
        let dispatcher_1 = store.dispatcher.clone();
        let dispatcher_2 = store.dispatcher.clone();
        let thread_1 = thread::spawn(move || {
            let _ = dispatcher_1.send(Action::Remove(1));
            let _ = dispatcher_1.send(Action::Add(1));
            let _ = dispatcher_1.send(Action::Add(1));
        });
        let thread_2 = thread::spawn(move || {
            let _ = dispatcher_2.send(Action::Remove(1));
            let _ = dispatcher_2.send(Action::Add(1));
            let _ = dispatcher_2.send(Action::Add(1));
        });
        thread_1.join().unwrap();
        thread_2.join().unwrap();
        store.tick();
        assert_eq!(store.pending_actions.len(), 6);
        store.update();
        assert_eq!(store.state, 2);
        assert_eq!(store.pending_actions.len(), 0);
    }
}
pub mod simple;