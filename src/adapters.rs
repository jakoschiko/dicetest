//! A collection of [`DieOnce`] and [`Die`] adapters.
//!
//! [`DieOnce`]: crate::DieOnce
//! [`Die`]: crate::Die

mod map_die;
pub use map_die::MapDie;

mod flatten_die;
pub use flatten_die::FlattenDie;

mod flat_map_die;
pub use flat_map_die::FlatMapDie;

mod boxed_die_once;
pub use boxed_die_once::BoxedDieOnce;

mod boxed_die;
pub use boxed_die::BoxedDie;

mod rc_die;
pub use rc_die::RcDie;

mod arc_die;
pub use arc_die::ArcDie;
