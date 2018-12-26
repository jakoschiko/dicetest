//! A collection of `GenOnce` and `Gen` adapters.

mod map_gen;
pub use self::map_gen::MapGen;

mod flatten_gen;
pub use self::flatten_gen::FlattenGen;

mod flat_map_gen;
pub use self::flat_map_gen::FlatMapGen;

mod boxed_gen_once;
pub use self::boxed_gen_once::BoxedGenOnce;

mod boxed_gen;
pub use self::boxed_gen::BoxedGen;

mod rc_gen;
pub use self::rc_gen::RcGen;

mod arc_gen;
pub use self::arc_gen::ArcGen;
