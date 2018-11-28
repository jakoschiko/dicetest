#[cfg(not(feature = "disabled_counter"))]
use std::cell::RefCell;

#[cfg(not(feature = "disabled_counter"))]
use crate::util::Finalizer;
use crate::counter::Stats;

#[cfg(not(feature = "disabled_counter"))]
struct Collection {
    stats: Stats,
}

thread_local! {
    #[cfg(not(feature = "disabled_counter"))]
    static LOCAL: RefCell<Vec<Collection>> = RefCell::new(Vec::new());
}

/// Returns all stats that were collected during the evaluation of `f`.
pub fn collect_stats<R>(f: impl FnOnce() -> R) -> (R, Stats) {
    #[cfg(not(feature = "disabled_counter"))]
    {
        LOCAL.with(move |cell| {
            {
                let mut collections = cell.borrow_mut();
                let collection = Collection {
                    stats: Stats::new(),
                };
                collections.push(collection);
            }

            // Removes the collection even in case of panic
            let finalizer = Finalizer::new(|| {
                LOCAL.with(move |cell| {
                    let mut collections = cell.borrow_mut();
                    collections.pop();
                })
            });

            // This function may panic
            let result = f();

            let stats = {
                let mut collections = cell.borrow_mut();
                let collection = collections.last_mut().unwrap();
                collection.stats.take()
            };

            drop(finalizer);

            (result, stats)
        })
    }
    #[cfg(feature = "disabled_counter")]
    {
        (f(), Stats::new())
    }
}

#[cfg(not(feature = "disabled_counter"))]
fn enabled_with_cell(cell: &RefCell<Vec<Collection>>) -> bool {
    !cell.borrow().is_empty()
}

/// Returns if counting is currently enabled.
///
/// Counting is enabled iff this function is executed inside of `collect_stats` and
/// the feature `disabled_counting` is not present.
pub fn enabled() -> bool {
    #[cfg(not(feature = "disabled_counter"))]
    {
        LOCAL.with(move |cell| enabled_with_cell(&cell))
    }
    #[cfg(feature = "disabled_counter")] {
        false
    }
}

/// Evaluates the given value and increases its counter for the given key iff counting is enabled.
pub fn count(key: &'static str, value: impl FnOnce() -> String) {
    #[cfg(not(feature = "disabled_counter"))]
    {
        LOCAL.with(move |cell| {
            if enabled_with_cell(&cell) {
                let mut collections = cell.borrow_mut();
                let value = value();
                let len = collections.len();

                collections[0..len-1]
                    .iter_mut()
                    .for_each(|collection| collection.stats.inc(key, value.clone()));
                collections[len-1].stats.inc(key, value);
            }
        });
    }
    #[cfg(feature = "disabled_counter")] {
        drop((key, value))
    }
}
