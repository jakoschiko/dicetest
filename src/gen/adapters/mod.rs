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

mod gen_boxed_once;
pub use self::gen_boxed_once::GenBoxedOnce;

mod gen_boxed;
pub use self::gen_boxed::GenBoxed;

mod gen_rc;
pub use self::gen_rc::GenRc;

mod gen_arc;
pub use self::gen_arc::GenArc;
