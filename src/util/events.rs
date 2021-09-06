use std::cell::RefCell;
use std::thread::LocalKey;

use crate::util::finalizer::Finalizer;

pub trait Events {
    fn new() -> Self;
    fn take(&mut self) -> Self;
}

pub type Stack<E> = RefCell<Vec<E>>;

pub fn new_stack<E: Events>() -> Stack<E> {
    RefCell::new(Vec::new())
}

pub type Local<E> = LocalKey<Stack<E>>;

pub fn collect<E: Events, R>(local: &'static Local<E>, f: impl FnOnce() -> R) -> (R, E) {
    local.with(move |cell| {
        {
            let events = E::new();
            let mut events_stack = cell.borrow_mut();
            events_stack.push(events);
        }

        // Removes the events even in case of panic
        let finalizer = Finalizer::new(|| {
            local.with(move |cell| {
                let mut events_stack = cell.borrow_mut();
                events_stack.pop();
            })
        });

        // This function may panic
        let result = f();

        let events = {
            let mut events_stack = cell.borrow_mut();
            let events = events_stack.last_mut().unwrap();
            events.take()
        };

        drop(finalizer);

        (result, events)
    })
}

fn enabled_with_cell<E: Events>(cell: &RefCell<Vec<E>>) -> bool {
    !cell.borrow().is_empty()
}

pub fn enabled<E: Events>(local: &'static Local<E>) -> bool {
    local.with(move |cell| enabled_with_cell(cell))
}

pub fn modify<E: Events>(local: &'static Local<E>, f: impl FnOnce(&mut Vec<E>)) {
    local.with(move |cell| {
        if enabled_with_cell(cell) {
            let mut events_stack = cell.borrow_mut();
            f(&mut events_stack);
        }
    });
}

#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;

    use crate::util::events::{self, Events, Stack};

    struct Counter(u32);

    impl Events for Counter {
        fn new() -> Self {
            Counter(0)
        }

        fn take(&mut self) -> Self {
            Counter(self.0)
        }
    }

    thread_local! {
        static LOCAL: Stack<Counter> = events::new_stack();
    }

    fn enabled() -> bool {
        events::enabled(&LOCAL)
    }

    fn collect(f: impl FnOnce()) -> u32 {
        let ((), Counter(n)) = events::collect(&LOCAL, f);
        n
    }

    fn inc() {
        events::modify(&LOCAL, |stack| stack.iter_mut().for_each(|c| c.0 += 1));
    }

    #[test]
    fn collect_and_modify() {
        let n = collect(|| {
            inc();
            inc();

            let m = collect(|| inc());

            assert_eq!(1, m);
        });

        assert_eq!(3, n);
    }

    #[test]
    fn is_only_enabled_during_collection() {
        assert!(!enabled());
        collect(|| {
            assert!(enabled());
        });
        assert!(!enabled());
    }

    #[test]
    fn logger_is_not_enabled_after_panic() {
        let _ = catch_unwind(|| {
            collect(|| {
                panic!();
            })
        });
        assert!(!enabled());
    }
}
