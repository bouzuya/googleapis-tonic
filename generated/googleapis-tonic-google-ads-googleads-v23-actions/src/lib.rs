#[cfg(not(any(feature = "btree-map", feature = "hash-map")))]
compile_error!("feature \"btree-map\" or feature \"hash-map\" must be enabled");
#[cfg(all(feature = "btree-map", feature = "hash-map"))]
compile_error!("feature \"btree-map\" and feature \"hash-map\" cannot be enabled at the same time");

#[cfg(not(any(feature = "bytes", feature = "vec-u8")))]
compile_error!("feature \"bytes\" or feature \"vec-u8\" must be enabled");
#[cfg(all(feature = "bytes", feature = "vec-u8"))]
compile_error!("feature \"bytes\" and feature \"vec-u8\" cannot be enabled at the same time");

#[cfg(all(feature = "bytes", feature = "btree-map"))]
mod bytes_btree_map;
#[cfg(all(feature = "bytes", feature = "hash-map"))]
mod bytes_hash_map;
#[cfg(all(feature = "vec-u8", feature = "btree-map"))]
mod vec_u8_btree_map;
#[cfg(all(feature = "vec-u8", feature = "hash-map"))]
mod vec_u8_hash_map;

#[cfg(all(feature = "bytes", feature = "btree-map"))]
pub use self::bytes_btree_map::*;
#[cfg(all(feature = "bytes", feature = "hash-map"))]
pub use self::bytes_hash_map::*;
#[cfg(all(feature = "vec-u8", feature = "btree-map"))]
pub use self::vec_u8_btree_map::*;
#[cfg(all(feature = "vec-u8", feature = "hash-map"))]
pub use self::vec_u8_hash_map::*;
