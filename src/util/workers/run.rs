use std::time::Duration;
use std::panic::UnwindSafe;

use util::workers::{JoinedResult, without_timeout, with_timeout};

/// Runs each function on a separate thread.
/// Blocks until all threads are terminated or the timeout is reached.
pub fn run<R, F>(
    funs: Vec<F>,
    timeout: Option<Duration>,
) -> JoinedResult<R>
where
    R: Send + 'static,
    F: FnOnce() -> R + Send + UnwindSafe + 'static,
{
    match timeout {
        None => without_timeout::run(funs),
        Some(timeout) => with_timeout::run(funs, timeout),
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, u64};

    use super::*;

    const MAX_DURATION: Duration = Duration::from_secs(u64::MAX);

    fn get_timeout(with_timeout: bool) -> Option<Duration> {
        if with_timeout { Some(MAX_DURATION) } else { None }
    }

    fn ok(with_timeout: bool) {
        fn fun() -> impl FnOnce() -> () + Send + 'static {
            || ()
        }

        let result = run(
            vec![fun(), fun()],
            get_timeout(with_timeout),
        );

        assert!(!result.timeout);
        assert_eq!(result.results.len(), 2);
        assert!(result.results[0].result.is_ok());
        assert!(result.results[1].result.is_ok());
    }

    fn err(with_timeout: bool) {
        fn fun(panic: bool) -> impl FnOnce() -> () + Send + 'static {
            move || if panic { panic!() }
        }

        let result = run(
            vec![fun(false), fun(true)],
            get_timeout(with_timeout),
        );

        assert!(!result.timeout);
        assert_eq!(result.results.len(), 2);
        assert!(result.results[0].result.is_err() || result.results[1].result.is_err());
    }

    #[test]
    fn ok_without_timeout() {
        ok(false)
    }

    #[test]
    fn err_without_timeout() {
        err(false)
    }

    #[test]
    fn ok_with_timeout() {
        ok(true)
    }

    #[test]
    fn err_with_timeout() {
        err(true)
    }

    #[test]
    fn timeout() {
        fn fun() -> impl FnOnce() -> () + Send + 'static {
            || thread::sleep(MAX_DURATION)
        }

        let result = run(
            vec![fun(), fun()],
            Some(Duration::from_millis(10)),
        );

        assert!(result.timeout);
    }
}
