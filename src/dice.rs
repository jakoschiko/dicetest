//! The standard collection of `DieOnce` and `Die` implementations.

mod from;
pub use from::*;

mod just;
pub use just::*;

mod zip;
pub use zip::*;

mod one_of;
pub use one_of::*;

mod bool;
pub use self::bool::*;

mod integer;
pub use integer::*;

mod float;
pub use float::*;

mod char;
pub use self::char::*;

mod array;
pub use array::*;

mod option;
pub use option::*;

mod result;
pub use result::*;

mod size;
pub use size::*;

mod terms_of;
pub use terms_of::*;

mod collection;
pub use collection::*;

mod vec;
pub use vec::*;

mod vec_deque;
pub use vec_deque::*;

mod linked_list;
pub use linked_list::*;

mod hash_map;
pub use hash_map::*;

mod b_tree_map;
pub use b_tree_map::*;

mod hash_set;
pub use hash_set::*;

mod b_tree_set;
pub use b_tree_set::*;

mod binary_heap;
pub use binary_heap::*;

mod string;
pub use string::*;

mod shuffle;
pub use shuffle::*;

mod split;
pub use split::*;

mod fn_builder;
pub use fn_builder::*;

mod index_of;
pub use index_of::*;

#[cfg(any(feature = "rand_full", all(feature = "rand_core", feature = "rand")))]
mod rand;
#[cfg(any(feature = "rand_full", all(feature = "rand_core", feature = "rand")))]
pub use self::rand::*;

#[cfg(any(
    feature = "quickcheck_full",
    all(feature = "rand_core", feature = "quickcheck")
))]
mod quickcheck;
#[cfg(any(
    feature = "quickcheck_full",
    all(feature = "rand_core", feature = "quickcheck")
))]
pub use self::quickcheck::*;
