#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(
    const_fn_trait_bound,
    const_generics,
    platform_intrinsics,
    repr_simd,
    simd_ffi,
    staged_api,
    stdsimd
)]
#![cfg_attr(
    feature = "const_evaluatable_checked",
    feature(const_evaluatable_checked)
)]
#![warn(missing_docs)]
#![unstable(feature = "portable_simd", issue = "86656")]
//! Portable SIMD module.

#[macro_use]
mod permute;
#[macro_use]
mod reduction;

mod select;
pub use select::Select;

#[cfg(feature = "const_evaluatable_checked")]
mod to_bytes;

mod comparisons;
mod fmt;
mod intrinsics;
mod iter;
mod math;
mod ops;
mod round;
mod vendor;

mod lane_count;
pub use lane_count::*;

mod masks;
pub use masks::*;

mod vector;
pub use vector::*;
