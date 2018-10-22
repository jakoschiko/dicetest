//! A collection of `GenOnce` and `Gen` adapters.

mod gen_map_once;
pub use self::gen_map_once::GenMapOnce;

mod gen_map;
pub use self::gen_map::GenMap;

mod gen_flatten_once;
pub use self::gen_flatten_once::GenFlattenOnce;

mod gen_flatten;
pub use self::gen_flatten::GenFlatten;

mod gen_flat_map_once;
pub use self::gen_flat_map_once::GenFlatMapOnce;

mod gen_flat_map;
pub use self::gen_flat_map::GenFlatMap;

mod dyn_gen_once;
pub use self::dyn_gen_once::DynGenOnce;

mod dyn_gen;
pub use self::dyn_gen::DynGen;

mod dyn_rc_gen;
pub use self::dyn_rc_gen::DynRcGen;

mod dyn_arc_gen;
pub use self::dyn_arc_gen::DynArcGen;
