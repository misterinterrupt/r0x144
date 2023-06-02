extern crate im;

mod Store {
    use std::{
        cell::RefCell,
        marker::PhantomData,
        sync::mpsc::{self, Receiver, Sender},
        sync::{Arc, Mutex},
        thread::{Scope, ScopedJoinHandle},
    };

    pub struct Store<'b, 'a, T: 'b, C: 'b + FnMut(Arc<T>, T) -> ()> {
        dispatch: RefCell<Option<Sender<T>>>,
        state: Arc<T>,
        scope: &'a Scope<'a, 'b>,
        scope_handle: RefCell<Option<ScopedJoinHandle<'a, T>>>,
        phantom_data: PhantomData<&'b T>,
        phantom_closure: PhantomData<&'b C>,
    }

    impl<'b, 'a, T, C: 'b + FnMut(Arc<T>, T) -> () + std::marker::Sync> Store<'b, 'a, T, C>
    where
        T: 'b + Send + Sync,
        C: 'b + Send + Sync,
    {
        pub fn new(scope: &'a Scope<'a, 'b>, initial_state: T) -> Self {
            Self {
                dispatch: RefCell::new(None),
                state: Arc::<T>::new(initial_state),
                scope,
                scope_handle: RefCell::new(None),
                phantom_data: PhantomData,
                phantom_closure: PhantomData,
            }
        }

        pub fn start(&'b self, event_loop: C) {
            let (tx, rx) = mpsc::channel::<T>();
            self.dispatch.replace(Some(tx));
            let rx = Arc::new(Mutex::new(rx));
            let state = self.state.clone();
            self.scope_handle.replace(Some(self.scope.spawn(move || {
                let state = state;
                let mut event_loop = event_loop;
                let q = rx.lock().unwrap();
                loop {
                    match q.try_recv() {
                        Ok(new_state) => {
                            event_loop(state, new_state);
                        }
                        Err(_) => (),
                    }
                }
            })));
        }

        pub fn get_dispatcher(&self) -> Sender<T> {
            self.dispatch.borrow().clone().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use im::Vector;

    use crate::Store::Store;
    use std::sync::Arc;

    #[test]
    fn vec_thread_access() {
        // create a new store with a scope
        std::thread::scope(|scope| {
            let mut idx = 1;
            let initial_state = Vector::<i32>::new();
            let store = Store::new(scope, initial_state, move |state, new_state| {
                println!("state len: {}", state.len());
                idx += 1;
            });
            &store.start();
            let reader = scope.spawn(move || {
                let dispatcher = &store.get_dispatcher();
                let mut count = 0;
                loop {
                    let mut new_state = Vector::<i32>::new();
                    new_state.push_front(0);
                    _ = dispatcher.send(new_state);
                    count += 1;
                }
            });
        });
    }
}

// type ClosureTrait<T> = dyn Fn(&T) -> ();
// struct MyType<'a, T, ConcreteClosure: ClosureTrait<T>> {
//   data: T,
//   closure:
// }
// impl<'a, T> MyType<'a, T> {
//   fn new(closure: MyClosure<ArgType>) -> Self {

//   }
// }
