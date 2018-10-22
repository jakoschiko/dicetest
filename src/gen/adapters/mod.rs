//! A collection of `GenOnce` and `Gen` adapters.

mod map_gen_once;
pub use self::map_gen_once::MapGenOnce;

mod map_gen;
pub use self::map_gen::MapGen;

mod flatten_gen_once;
pub use self::flatten_gen_once::FlattenGenOnce;

mod flatten_gen;
pub use self::flatten_gen::FlattenGen;

mod flat_map_gen_once;
pub use self::flat_map_gen_once::FlatMapGenOnce;

mod flat_map_gen;
pub use self::flat_map_gen::FlatMapGen;

mod dyn_gen_once;
pub use self::dyn_gen_once::DynGenOnce;

mod dyn_gen;
pub use self::dyn_gen::DynGen;

mod dyn_rc_gen;
pub use self::dyn_rc_gen::DynRcGen;

mod dyn_arc_gen;
pub use self::dyn_arc_gen::DynArcGen;
