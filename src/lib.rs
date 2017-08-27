//! Bindings to the [Basic Linear Algebra Subprograms][blas].
//!
//! [blas]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms
#![no_std]

extern crate libc;

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;

/// A complex number with 64-bit parts.
#[allow(bad_style)]
pub type c_double_complex = [libc::c_double; 2];

/// A complex number with 32-bit parts.
#[allow(bad_style)]
pub type c_float_complex = [libc::c_float; 2];

pub mod c;
pub mod fortran;
