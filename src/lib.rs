//! The library facilitates static linking with the [Basic Linear Algebra
//! Subprograms][1].
//!
//! [1]: http://www.netlib.org/blas/

extern crate libc;

use libc::{c_char, c_double, c_int};

pub use dgemm_ as dgemm;
pub use dgemv_ as dgemv;

#[link(name = "gfortran")]
extern {
    /// http://www.netlib.org/lapack/explore-html/dc/da8/dgemv_8f.html
    fn dgemv_(trans: *const c_char, m: *const c_int, n: *const c_int,
              alpha: *const c_double, a: *const c_double, lda: *const c_int,
              x: *const c_double, incx: *const c_int, beta: *const c_double,
              y: *mut c_double, incy: *const c_int);

    /// http://www.netlib.org/lapack/explore-html/d7/d2b/dgemm_8f.html
    fn dgemm_(transa: *const c_char, transb: *const c_char, m: *const c_int,
              n: *const c_int, k: *const c_int, alpha: *const c_double,
              a: *const c_double, lda: *const c_int, b: *const c_double,
              ldb: *const c_int, beta: *const c_double, c: *mut c_double,
              ldc: *const c_int);
}
