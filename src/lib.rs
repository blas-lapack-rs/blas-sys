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

#[allow(bad_style)]
pub type c_double_complex = [libc::c_double; 2];

#[allow(bad_style)]
pub type c_float_complex = [libc::c_float; 2];

mod c;
mod fortran;

pub use c::*;
pub use fortran::*;
