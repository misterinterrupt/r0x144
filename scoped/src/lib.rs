// allow dead_code when checking the lib without tests
// cargo-analyzer runs `cargo check` for lib the lib with and without tests.
// dead_code warnings in the ide won't show if the code is used in tests.
// #![cfg_attr(not(test), allow(dead_code))]
#![allow(dead_code)]
mod store {
    extern crate im;
    use std::{
        cell::RefCell,
        marker::PhantomData,
        sync::mpsc::{self, Sender},
    };

    pub struct Store<'b, T: 'b, C: 'b + FnMut(T, T) -> bool> {
        dispatch: RefCell<Option<Sender<T>>>,
        state: T,
        phantom_data: PhantomData<&'b T>,
        phantom_closure: PhantomData<&'b C>,
    }

    impl<'b, 'a, T, C: 'b + FnMut(T, T) -> bool + std::marker::Sync> Store<'b, T, C>
    where
        Self: 'b,
        T: 'b + Send + Sync + Clone,
        C: 'b + Send + Sync,
    {
        pub fn new(initial_state: T) -> Self {
            Self {
                dispatch: RefCell::new(None),
                state: initial_state,
                phantom_data: PhantomData,
                phantom_closure: PhantomData,
            }
        }

        pub fn start(&'b self, event_loop: C) {
            std::thread::scope(move |scope| {
                let (tx, rx) = mpsc::channel::<T>();
                self.dispatch.replace(Some(tx.clone()));
                let state = self.state.clone();
                let mut event_loop = event_loop;
                _ = scope
                    .spawn(move || {
                        println!("event loop started");
                        loop {
                            match rx.try_recv() {
                                Ok(new_state) => match event_loop(state.clone(), new_state) {
                                    true => {
                                        println!("got a state update");
                                    }
                                    false => break,
                                },
                                Err(_) => {
                                    // print!(".");
                                }
                            }
                        }
                    })
                    .join();
            });
        }

        pub fn get_dispatcher(&self) -> Option<Sender<T>> {
            self.dispatch.borrow().clone()
        }
    }
}

// #[cfg(test)]
mod tests {
    use im::Vector;

    use crate::store::Store;
    use std::thread;

    // #[test]
    fn vec_thread_access() {
        let initial_state = Vector::<i32>::new();
        let store = Store::new(initial_state);
        store.start(move |mut state, new_state| {
            println!("state len: {}", state.len());
            println!("new_state len: {}", new_state.len());
            state = new_state;
            if state.len() == 10 {
                return false;
            }
            true
        });
        println!("duh");
        let dispatcher = store.get_dispatcher();
        println!("got dispatcher {}", dispatcher.is_some());
        _ = thread::spawn(move || {
            if let Some(dispatch) = dispatcher {
                println!("reader started");
                let mut count = 0;
                loop {
                    let mut new_state = Vector::<i32>::new();
                    new_state.push_front(count);
                    _ = dispatch.send(new_state);
                    count += 1;
                    if count == 10 {
                        break;
                    }
                }
            } else {
                panic!("no dispatcher");
            }
        })
        .join();
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
