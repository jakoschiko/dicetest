//! A collection of `DieOnce` and `Die` adapters.

mod map_die;
pub use self::map_die::MapDie;

mod flatten_die;
pub use self::flatten_die::FlattenDie;

mod flat_map_die;
pub use self::flat_map_die::FlatMapDie;

mod boxed_die_once;
pub use self::boxed_die_once::BoxedDieOnce;

mod boxed_die;
pub use self::boxed_die::BoxedDie;

mod rc_die;
pub use self::rc_die::RcDie;

mod arc_die;
pub use self::arc_die::ArcDie;
