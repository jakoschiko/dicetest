use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use std::panic::{UnwindSafe, catch_unwind};

use util::workers::{SingleResult, JoinedResult};

enum Msg<R> {
    Timeout,
    Finished(SingleResult<R>),
}

pub fn run<R, F>(
    funs: Vec<F>,
    timeout: Duration,
) -> JoinedResult<R>
where
    R: Send + 'static,
    F: FnOnce() -> R + Send + UnwindSafe + 'static,
{
    if funs.is_empty() {
        let joined_result = JoinedResult {
            timeout: false,
            results: Vec::new(),
        };
        return joined_result;
    }

    let (sender, receiver) = channel();
    let fun_count = funs.len();

    for (index, fun) in funs.into_iter().enumerate() {
        spawn_thread(index as u64, fun, Sender::clone(&sender))
    }

    spawn_timeout_thread::<R>(timeout, Sender::clone(&sender));

    let mut joined_result = JoinedResult::new();

    loop {
        let msg = receiver.recv()
            .expect("Failed to receive message from worker thread or timeout thread");

        match msg {
            Msg::Timeout => {
                joined_result.timeout = true;
                break;
            },
            Msg::Finished(single_result) => {
                joined_result.results.push(single_result);
                if joined_result.results.len() == fun_count {
                    break;
                }
            }
        }
    }

    joined_result
}

fn spawn_timeout_thread<R>(timeout: Duration, tx: Sender<Msg<R>>)
where
    R: Send + 'static,
{
    thread::spawn(move || {
        thread::sleep(timeout);

        tx.send(Msg::Timeout)
            .expect("Worker thread failed to send its result message")
    });
}

fn spawn_thread<R, F>(index: u64, fun: F, tx: Sender<Msg<R>>)
where
    R: Send + 'static,
    F: FnOnce() -> R + Send + UnwindSafe + 'static,
{
    thread::spawn(move || {
        let result = catch_unwind(fun);
        let single_result = SingleResult { index, result };
        tx.send(Msg::Finished(single_result))
            .expect("Timeout thread failed to send its timeout message");
    });
}
