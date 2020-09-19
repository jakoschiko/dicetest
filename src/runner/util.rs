use crate::hints::{self, Hints};
use crate::stats::{self, Stats};

pub fn collect_hints<R>(enabled: bool, f: impl FnOnce() -> R) -> (R, Option<Hints>) {
    if enabled {
        let (result, hints) = hints::collect(f);
        (result, Some(hints))
    } else {
        let result = f();
        (result, None)
    }
}

pub fn collect_stats<R>(enabled: bool, f: impl FnOnce() -> R) -> (R, Option<Stats>) {
    if enabled {
        let (result, stats) = stats::collect(f);
        (result, Some(stats))
    } else {
        let result = f();
        (result, None)
    }
}
