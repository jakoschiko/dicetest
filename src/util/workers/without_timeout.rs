use std::thread;

use util::workers::{SingleResult, JoinedResult};

pub fn run<R, F>(
    funs: Vec<F>,
) -> JoinedResult<R>
where
    R: Send + 'static,
    F: FnOnce() -> R + Send + 'static,
{
    let threads: Vec<_> = funs
        .into_iter()
        .enumerate()
        .map(move |(index, fun)| {
            let thread = thread::spawn(move || {
                fun()
            });
            (index as u64, thread)
        })
        .collect();

    let results: Vec<_> = threads
        .into_iter()
        .map(|(index, thread)| {
            let result = thread.join();
            SingleResult { index, result }
        })
        .collect();

    JoinedResult {
        timeout: false,
        results
    }
}
