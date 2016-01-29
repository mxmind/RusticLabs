
#[feature(asm, custom_derive, plugin, repr_simd, test)]
#[feature(augmented_assignments)]
#[feature(op_assign_traits)]
#[cfg_attr(feature = "plugins", plugin(heapsize_plugin))]
#[cfg_attr(feature = "plugins", plugin(serde_macros))]
#[cfg(feature = "plugins")]
#[macro_use]
extern crate heapsize;

#[macro_use]
extern crate log;
extern crate rustc_serialize;
#[cfg(feature = "plugins")]
extern crate serde;

#[cfg(test)]
extern crate rand;
extern crate num as num_lib;

// public exports

// public modules
pub mod scale_factor;
pub mod length;
pub mod point;
pub mod size;
pub mod num;
