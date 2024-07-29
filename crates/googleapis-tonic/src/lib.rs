#[cfg(all(feature = "btree-map", feature = "hash-map"))]
compile_error!("feature \"btree-map\" and feature \"hash-map\" cannot be enabled at the same time");

#[cfg(feature = "btree-map")]
mod btree_map;
#[cfg(feature = "hash-map")]
mod hash_map;

#[cfg(feature = "btree-map")]
pub use self::btree_map::*;
#[cfg(feature = "hash-map")]
pub use self::hash_map::*;
