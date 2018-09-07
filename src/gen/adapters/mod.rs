//! A collection of `GenOnce` and `Gen` adapters.

mod map_once;
pub use self::map_once::MapOnce;

mod map;
pub use self::map::Map;

mod flatten;
pub use self::flatten::Flatten;

mod flatten_once;
pub use self::flatten_once::FlattenOnce;

mod flat_map;
pub use self::flat_map::FlatMap;

mod flat_map_once;
pub use self::flat_map_once::FlatMapOnce;
