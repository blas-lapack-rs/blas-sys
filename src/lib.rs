//! Facilitation of static linking with the [Basic Linear Algebra Subprograms][1].
//!
//! [1]: http://www.netlib.org/blas/

#[allow(unstable)]
extern crate libc;

use libc::{c_char, c_double, c_int};

pub use dgemm_ as dgemm;
pub use dgemv_ as dgemv;

#[link(name = "gfortran")]
extern {
    /// http://www.netlib.org/lapack/explore-html/d7/d2b/dgemm_8f.html
    fn dgemm_(TRANSA: *const c_char, TRANSB: *const c_char, M: *const c_int,
              N: *const c_int, K: *const c_int, ALPHA: *const c_double,
              A: *const c_double, LDA: *const c_int, B: *const c_double,
              LDB: *const c_int, BETA: *const c_double, C: *mut c_double,
              LDC: *const c_int);

    /// http://www.netlib.org/lapack/explore-html/dc/da8/dgemv_8f.html
    fn dgemv_(TRANS: *const c_char, M: *const c_int, N: *const c_int,
              ALPHA: *const c_double, A: *const c_double, LDA: *const c_int,
              X: *const c_double, INCX: *const c_int, BETA: *const c_double,
              Y: *mut c_double, INCY: *const c_int);
}
