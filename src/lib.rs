//! Bindings to the [Basic Linear Algebra Subprograms][1].
//!
//! [1]: http://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms

extern crate libc;

#[cfg(feature = "netlib")]
extern crate netlib_provider as raw;

#[cfg(feature = "openblas")]
extern crate openblas_provider as raw;

#[cfg(feature = "accelerate")]
extern crate accelerate_provider as raw;

mod fortran;

pub use fortran::*;
