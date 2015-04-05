//! Bindings to the [Basic Linear Algebra Subprograms][1].
//!
//! This crate binds CBLAS, and the headers aren't particularly good. In some functions, `*mut
//! c_double` is treated as a pointer to a double, and in others as a pointer to a `double
//! _Complex`. Some functions may also be missing.
//!
//! [1]: http://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms

#![allow(bad_style)]

#[cfg(feature = "openblas")]
extern crate openblas_blas_provider as raw;

#[cfg(feature = "netlib")]
extern crate netlib_blas_provider as raw;

extern crate libc;

use libc::{c_uint, c_int, c_char, c_double, c_float, size_t};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct float_complex {
    pub real: c_float,
    pub imag: c_float,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct double_complex {
    pub real: c_double,
    pub imag: c_double,
}

pub type openblas_complex_float = [c_float; 2];
pub type openblas_complex_double = [c_double; 2];

pub type blasint = c_int;

pub type CBLAS_ORDER = c_uint;
pub const CblasRowMajor: c_uint = 101;
pub const CblasColMajor: c_uint = 102;

pub type CBLAS_TRANSPOSE = c_uint;
pub const CblasNoTrans: c_uint = 111;
pub const CblasTrans: c_uint = 112;
pub const CblasConjTrans: c_uint = 113;
pub const CblasConjNoTrans: c_uint = 114;

pub type CBLAS_UPLO = c_uint;
pub const CblasUpper: c_uint = 121;
pub const CblasLower: c_uint = 122;

pub type CBLAS_DIAG = c_uint;
pub const CblasNonUnit: c_uint = 131;
pub const CblasUnit: c_uint = 132;

pub type CBLAS_SIDE = c_uint;
pub const CblasLeft: c_uint = 141;
pub const CblasRight: c_uint = 142;

extern "C" {
    pub fn cblas_sdsdot(n: blasint, alpha: c_float, x: *const c_float, incx: blasint, y: *const c_float, incy: blasint) -> c_float;
    pub fn cblas_dsdot(n: blasint, x: *const c_float, incx: blasint, y: *const c_float, incy: blasint) -> c_double;
    pub fn cblas_sdot(n: blasint, x: *const c_float, incx: blasint, y: *const c_float, incy: blasint) -> c_float;
    pub fn cblas_ddot(n: blasint, x: *const c_double, incx: blasint, y: *const c_double, incy: blasint) -> c_double;
    pub fn cblas_cdotu(n: blasint, x: *const c_float, incx: blasint, y: *const c_float, incy: blasint) -> openblas_complex_float;
    pub fn cblas_cdotc(n: blasint, x: *const c_float, incx: blasint, y: *const c_float, incy: blasint) -> openblas_complex_float;
    pub fn cblas_zdotu(n: blasint, x: *const c_double, incx: blasint, y: *const c_double, incy: blasint) -> openblas_complex_double;
    pub fn cblas_zdotc(n: blasint, x: *const c_double, incx: blasint, y: *const c_double, incy: blasint) -> openblas_complex_double;
    pub fn cblas_cdotu_sub(n: blasint, x: *const c_float, incx: blasint, y: *const c_float, incy: blasint, ret: *mut openblas_complex_float) -> ();
    pub fn cblas_cdotc_sub(n: blasint, x: *const c_float, incx: blasint, y: *const c_float, incy: blasint, ret: *mut openblas_complex_float) -> ();
    pub fn cblas_zdotu_sub(n: blasint, x: *const c_double, incx: blasint, y: *const c_double, incy: blasint, ret: *mut openblas_complex_double) -> ();
    pub fn cblas_zdotc_sub(n: blasint, x: *const c_double, incx: blasint, y: *const c_double, incy: blasint, ret: *mut openblas_complex_double) -> ();
    pub fn cblas_sasum(n: blasint, x: *const c_float, incx: blasint) -> c_float;
    pub fn cblas_dasum(n: blasint, x: *const c_double, incx: blasint) -> c_double;
    pub fn cblas_scasum(n: blasint, x: *const c_float, incx: blasint) -> c_float;
    pub fn cblas_dzasum(n: blasint, x: *const c_double, incx: blasint) -> c_double;
    pub fn cblas_snrm2(N: blasint, X: *const c_float, incX: blasint) -> c_float;
    pub fn cblas_dnrm2(N: blasint, X: *const c_double, incX: blasint) -> c_double;
    pub fn cblas_scnrm2(N: blasint, X: *const c_float, incX: blasint) -> c_float;
    pub fn cblas_dznrm2(N: blasint, X: *const c_double, incX: blasint) -> c_double;
    pub fn cblas_isamax(n: blasint, x: *const c_float, incx: blasint) -> size_t;
    pub fn cblas_idamax(n: blasint, x: *const c_double, incx: blasint) -> size_t;
    pub fn cblas_icamax(n: blasint, x: *const c_float, incx: blasint) -> size_t;
    pub fn cblas_izamax(n: blasint, x: *const c_double, incx: blasint) -> size_t;
    pub fn cblas_saxpy(n: blasint, alpha: c_float, x: *const c_float, incx: blasint, y: *mut c_float, incy: blasint) -> ();
    pub fn cblas_daxpy(n: blasint, alpha: c_double, x: *const c_double, incx: blasint, y: *mut c_double, incy: blasint) -> ();
    pub fn cblas_caxpy(n: blasint, alpha: *const c_float, x: *const c_float, incx: blasint, y: *mut c_float, incy: blasint) -> ();
    pub fn cblas_zaxpy(n: blasint, alpha: *const c_double, x: *const c_double, incx: blasint, y: *mut c_double, incy: blasint) -> ();
    pub fn cblas_scopy(n: blasint, x: *const c_float, incx: blasint, y: *mut c_float, incy: blasint) -> ();
    pub fn cblas_dcopy(n: blasint, x: *const c_double, incx: blasint, y: *mut c_double, incy: blasint) -> ();
    pub fn cblas_ccopy(n: blasint, x: *const c_float, incx: blasint, y: *mut c_float, incy: blasint) -> ();
    pub fn cblas_zcopy(n: blasint, x: *const c_double, incx: blasint, y: *mut c_double, incy: blasint) -> ();
    pub fn cblas_sswap(n: blasint, x: *mut c_float, incx: blasint, y: *mut c_float, incy: blasint) -> ();
    pub fn cblas_dswap(n: blasint, x: *mut c_double, incx: blasint, y: *mut c_double, incy: blasint) -> ();
    pub fn cblas_cswap(n: blasint, x: *mut c_float, incx: blasint, y: *mut c_float, incy: blasint) -> ();
    pub fn cblas_zswap(n: blasint, x: *mut c_double, incx: blasint, y: *mut c_double, incy: blasint) -> ();
    pub fn cblas_srot(N: blasint, X: *mut c_float, incX: blasint, Y: *mut c_float, incY: blasint, c: c_float, s: c_float) -> ();
    pub fn cblas_drot(N: blasint, X: *mut c_double, incX: blasint, Y: *mut c_double, incY: blasint, c: c_double, s: c_double) -> ();
    pub fn cblas_crot(N: blasint, X: *mut c_float, incX: blasint, Y: *mut c_float, incY: blasint, c: c_float, s: *const c_float) -> ();
    pub fn cblas_zrot(N: blasint, X: *mut c_double, incX: blasint, Y: *mut c_double, incY: blasint, c: c_double, s: *const c_double) -> ();
    pub fn cblas_csrot(N: blasint, X: *mut c_float, incX: blasint, Y: *mut c_float, incY: blasint, c: c_float, s: c_float) -> ();
    pub fn cblas_zdrot(N: blasint, X: *mut c_double, incX: blasint, Y: *mut c_double, incY: blasint, c: c_double, s: c_double) -> ();
    pub fn cblas_srotg(a: *mut c_float, b: *mut c_float, c: *mut c_float, s: *mut c_float) -> ();
    pub fn cblas_drotg(a: *mut c_double, b: *mut c_double, c: *mut c_double, s: *mut c_double) -> ();
    pub fn cblas_crotg(a: *mut c_float, b: *mut c_float, c: *mut c_float, s: *mut c_float) -> ();
    pub fn cblas_zrotg(a: *mut c_double, b: *mut c_double, c: *mut c_double, s: *mut c_double) -> ();
    pub fn cblas_srotm(N: blasint, X: *mut c_float, incX: blasint, Y: *mut c_float, incY: blasint, P: *const c_float) -> ();
    pub fn cblas_drotm(N: blasint, X: *mut c_double, incX: blasint, Y: *mut c_double, incY: blasint, P: *const c_double) -> ();
    pub fn cblas_srotmg(d1: *mut c_float, d2: *mut c_float, b1: *mut c_float, b2: c_float, P: *mut c_float) -> ();
    pub fn cblas_drotmg(d1: *mut c_double, d2: *mut c_double, b1: *mut c_double, b2: c_double, P: *mut c_double) -> ();
    pub fn cblas_sscal(N: blasint, alpha: c_float, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_dscal(N: blasint, alpha: c_double, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_cscal(N: blasint, alpha: *const c_float, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_zscal(N: blasint, alpha: *const c_double, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_csscal(N: blasint, alpha: c_float, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_zdscal(N: blasint, alpha: c_double, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_sgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: blasint, n: blasint, alpha: c_float, a: *const c_float, lda: blasint, x: *const c_float, incx: blasint, beta: c_float, y: *mut c_float, incy: blasint) -> ();
    pub fn cblas_dgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: blasint, n: blasint, alpha: c_double, a: *const c_double, lda: blasint, x: *const c_double, incx: blasint, beta: c_double, y: *mut c_double, incy: blasint) -> ();
    pub fn cblas_cgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: blasint, n: blasint, alpha: *const c_float, a: *const c_float, lda: blasint, x: *const c_float, incx: blasint, beta: *const c_float, y: *mut c_float, incy: blasint) -> ();
    pub fn cblas_zgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: blasint, n: blasint, alpha: *const c_double, a: *const c_double, lda: blasint, x: *const c_double, incx: blasint, beta: *const c_double, y: *mut c_double, incy: blasint) -> ();
    pub fn cblas_sger(order: CBLAS_ORDER, M: blasint, N: blasint, alpha: c_float, X: *const c_float, incX: blasint, Y: *const c_float, incY: blasint, A: *mut c_float, lda: blasint) -> ();
    pub fn cblas_dger(order: CBLAS_ORDER, M: blasint, N: blasint, alpha: c_double, X: *const c_double, incX: blasint, Y: *const c_double, incY: blasint, A: *mut c_double, lda: blasint) -> ();
    pub fn cblas_cgeru(order: CBLAS_ORDER, M: blasint, N: blasint, alpha: *const c_float, X: *const c_float, incX: blasint, Y: *const c_float, incY: blasint, A: *mut c_float, lda: blasint) -> ();
    pub fn cblas_cgerc(order: CBLAS_ORDER, M: blasint, N: blasint, alpha: *const c_float, X: *const c_float, incX: blasint, Y: *const c_float, incY: blasint, A: *mut c_float, lda: blasint) -> ();
    pub fn cblas_zgeru(order: CBLAS_ORDER, M: blasint, N: blasint, alpha: *const c_double, X: *const c_double, incX: blasint, Y: *const c_double, incY: blasint, A: *mut c_double, lda: blasint) -> ();
    pub fn cblas_zgerc(order: CBLAS_ORDER, M: blasint, N: blasint, alpha: *const c_double, X: *const c_double, incX: blasint, Y: *const c_double, incY: blasint, A: *mut c_double, lda: blasint) -> ();
    pub fn cblas_strsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, A: *const c_float, lda: blasint, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_dtrsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, A: *const c_double, lda: blasint, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_ctrsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, A: *const c_float, lda: blasint, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_ztrsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, A: *const c_double, lda: blasint, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_strmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, A: *const c_float, lda: blasint, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_dtrmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, A: *const c_double, lda: blasint, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_ctrmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, A: *const c_float, lda: blasint, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_ztrmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, A: *const c_double, lda: blasint, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_ssyr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_float, X: *const c_float, incX: blasint, A: *mut c_float, lda: blasint) -> ();
    pub fn cblas_dsyr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_double, X: *const c_double, incX: blasint, A: *mut c_double, lda: blasint) -> ();
    pub fn cblas_cher(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_float, X: *const c_float, incX: blasint, A: *mut c_float, lda: blasint) -> ();
    pub fn cblas_zher(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_double, X: *const c_double, incX: blasint, A: *mut c_double, lda: blasint) -> ();
    pub fn cblas_ssyr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_float, X: *const c_float, incX: blasint, Y: *const c_float, incY: blasint, A: *mut c_float, lda: blasint) -> ();
    pub fn cblas_dsyr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_double, X: *const c_double, incX: blasint, Y: *const c_double, incY: blasint, A: *mut c_double, lda: blasint) -> ();
    pub fn cblas_cher2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: *const c_float, X: *const c_float, incX: blasint, Y: *const c_float, incY: blasint, A: *mut c_float, lda: blasint) -> ();
    pub fn cblas_zher2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: *const c_double, X: *const c_double, incX: blasint, Y: *const c_double, incY: blasint, A: *mut c_double, lda: blasint) -> ();
    pub fn cblas_sgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: blasint, N: blasint, KL: blasint, KU: blasint, alpha: c_float, A: *const c_float, lda: blasint, X: *const c_float, incX: blasint, beta: c_float, Y: *mut c_float, incY: blasint) -> ();
    pub fn cblas_dgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: blasint, N: blasint, KL: blasint, KU: blasint, alpha: c_double, A: *const c_double, lda: blasint, X: *const c_double, incX: blasint, beta: c_double, Y: *mut c_double, incY: blasint) -> ();
    pub fn cblas_cgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: blasint, N: blasint, KL: blasint, KU: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, X: *const c_float, incX: blasint, beta: *const c_float, Y: *mut c_float, incY: blasint) -> ();
    pub fn cblas_zgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: blasint, N: blasint, KL: blasint, KU: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, X: *const c_double, incX: blasint, beta: *const c_double, Y: *mut c_double, incY: blasint) -> ();
    pub fn cblas_ssbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, K: blasint, alpha: c_float, A: *const c_float, lda: blasint, X: *const c_float, incX: blasint, beta: c_float, Y: *mut c_float, incY: blasint) -> ();
    pub fn cblas_dsbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, K: blasint, alpha: c_double, A: *const c_double, lda: blasint, X: *const c_double, incX: blasint, beta: c_double, Y: *mut c_double, incY: blasint) -> ();
    pub fn cblas_stbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, K: blasint, A: *const c_float, lda: blasint, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_dtbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, K: blasint, A: *const c_double, lda: blasint, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_ctbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, K: blasint, A: *const c_float, lda: blasint, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_ztbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, K: blasint, A: *const c_double, lda: blasint, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_stbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, K: blasint, A: *const c_float, lda: blasint, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_dtbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, K: blasint, A: *const c_double, lda: blasint, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_ctbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, K: blasint, A: *const c_float, lda: blasint, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_ztbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, K: blasint, A: *const c_double, lda: blasint, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_stpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, Ap: *const c_float, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_dtpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, Ap: *const c_double, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_ctpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, Ap: *const c_float, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_ztpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, Ap: *const c_double, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_stpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, Ap: *const c_float, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_dtpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, Ap: *const c_double, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_ctpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, Ap: *const c_float, X: *mut c_float, incX: blasint) -> ();
    pub fn cblas_ztpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: blasint, Ap: *const c_double, X: *mut c_double, incX: blasint) -> ();
    pub fn cblas_ssymv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_float, A: *const c_float, lda: blasint, X: *const c_float, incX: blasint, beta: c_float, Y: *mut c_float, incY: blasint) -> ();
    pub fn cblas_dsymv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_double, A: *const c_double, lda: blasint, X: *const c_double, incX: blasint, beta: c_double, Y: *mut c_double, incY: blasint) -> ();
    pub fn cblas_chemv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, X: *const c_float, incX: blasint, beta: *const c_float, Y: *mut c_float, incY: blasint) -> ();
    pub fn cblas_zhemv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, X: *const c_double, incX: blasint, beta: *const c_double, Y: *mut c_double, incY: blasint) -> ();
    pub fn cblas_sspmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_float, Ap: *const c_float, X: *const c_float, incX: blasint, beta: c_float, Y: *mut c_float, incY: blasint) -> ();
    pub fn cblas_dspmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_double, Ap: *const c_double, X: *const c_double, incX: blasint, beta: c_double, Y: *mut c_double, incY: blasint) -> ();
    pub fn cblas_sspr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_float, X: *const c_float, incX: blasint, Ap: *mut c_float) -> ();
    pub fn cblas_dspr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_double, X: *const c_double, incX: blasint, Ap: *mut c_double) -> ();
    pub fn cblas_chpr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_float, X: *const c_float, incX: blasint, A: *mut c_float) -> ();
    pub fn cblas_zhpr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_double, X: *const c_double, incX: blasint, A: *mut c_double) -> ();
    pub fn cblas_sspr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_float, X: *const c_float, incX: blasint, Y: *const c_float, incY: blasint, A: *mut c_float) -> ();
    pub fn cblas_dspr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: c_double, X: *const c_double, incX: blasint, Y: *const c_double, incY: blasint, A: *mut c_double) -> ();
    pub fn cblas_chpr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: *const c_float, X: *const c_float, incX: blasint, Y: *const c_float, incY: blasint, Ap: *mut c_float) -> ();
    pub fn cblas_zhpr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: *const c_double, X: *const c_double, incX: blasint, Y: *const c_double, incY: blasint, Ap: *mut c_double) -> ();
    pub fn cblas_chbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, K: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, X: *const c_float, incX: blasint, beta: *const c_float, Y: *mut c_float, incY: blasint) -> ();
    pub fn cblas_zhbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, K: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, X: *const c_double, incX: blasint, beta: *const c_double, Y: *mut c_double, incY: blasint) -> ();
    pub fn cblas_chpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: *const c_float, Ap: *const c_float, X: *const c_float, incX: blasint, beta: *const c_float, Y: *mut c_float, incY: blasint) -> ();
    pub fn cblas_zhpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: blasint, alpha: *const c_double, Ap: *const c_double, X: *const c_double, incX: blasint, beta: *const c_double, Y: *mut c_double, incY: blasint) -> ();
    pub fn cblas_sgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: blasint, N: blasint, K: blasint, alpha: c_float, A: *const c_float, lda: blasint, B: *const c_float, ldb: blasint, beta: c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_dgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: blasint, N: blasint, K: blasint, alpha: c_double, A: *const c_double, lda: blasint, B: *const c_double, ldb: blasint, beta: c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_cgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: blasint, N: blasint, K: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, B: *const c_float, ldb: blasint, beta: *const c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_cgemm3m(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: blasint, N: blasint, K: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, B: *const c_float, ldb: blasint, beta: *const c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_zgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: blasint, N: blasint, K: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, B: *const c_double, ldb: blasint, beta: *const c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_zgemm3m(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: blasint, N: blasint, K: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, B: *const c_double, ldb: blasint, beta: *const c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_ssymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: blasint, N: blasint, alpha: c_float, A: *const c_float, lda: blasint, B: *const c_float, ldb: blasint, beta: c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_dsymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: blasint, N: blasint, alpha: c_double, A: *const c_double, lda: blasint, B: *const c_double, ldb: blasint, beta: c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_csymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: blasint, N: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, B: *const c_float, ldb: blasint, beta: *const c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_zsymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: blasint, N: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, B: *const c_double, ldb: blasint, beta: *const c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_ssyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: c_float, A: *const c_float, lda: blasint, beta: c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_dsyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: c_double, A: *const c_double, lda: blasint, beta: c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_csyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, beta: *const c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_zsyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, beta: *const c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_ssyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: c_float, A: *const c_float, lda: blasint, B: *const c_float, ldb: blasint, beta: c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_dsyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: c_double, A: *const c_double, lda: blasint, B: *const c_double, ldb: blasint, beta: c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_csyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, B: *const c_float, ldb: blasint, beta: *const c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_zsyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, B: *const c_double, ldb: blasint, beta: *const c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_strmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: blasint, N: blasint, alpha: c_float, A: *const c_float, lda: blasint, B: *mut c_float, ldb: blasint) -> ();
    pub fn cblas_dtrmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: blasint, N: blasint, alpha: c_double, A: *const c_double, lda: blasint, B: *mut c_double, ldb: blasint) -> ();
    pub fn cblas_ctrmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: blasint, N: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, B: *mut c_float, ldb: blasint) -> ();
    pub fn cblas_ztrmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: blasint, N: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, B: *mut c_double, ldb: blasint) -> ();
    pub fn cblas_strsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: blasint, N: blasint, alpha: c_float, A: *const c_float, lda: blasint, B: *mut c_float, ldb: blasint) -> ();
    pub fn cblas_dtrsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: blasint, N: blasint, alpha: c_double, A: *const c_double, lda: blasint, B: *mut c_double, ldb: blasint) -> ();
    pub fn cblas_ctrsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: blasint, N: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, B: *mut c_float, ldb: blasint) -> ();
    pub fn cblas_ztrsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: blasint, N: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, B: *mut c_double, ldb: blasint) -> ();
    pub fn cblas_chemm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: blasint, N: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, B: *const c_float, ldb: blasint, beta: *const c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_zhemm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: blasint, N: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, B: *const c_double, ldb: blasint, beta: *const c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_cherk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: c_float, A: *const c_float, lda: blasint, beta: c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_zherk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: c_double, A: *const c_double, lda: blasint, beta: c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_cher2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: *const c_float, A: *const c_float, lda: blasint, B: *const c_float, ldb: blasint, beta: c_float, C: *mut c_float, ldc: blasint) -> ();
    pub fn cblas_zher2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: blasint, K: blasint, alpha: *const c_double, A: *const c_double, lda: blasint, B: *const c_double, ldb: blasint, beta: c_double, C: *mut c_double, ldc: blasint) -> ();
    pub fn cblas_xerbla(p: blasint, rout: *mut c_char, form: *mut c_char, ...) -> ();
    pub fn cblas_saxpby(n: blasint, alpha: c_float, x: *const c_float, incx: blasint, beta: c_float, y: *mut c_float, incy: blasint) -> ();
    pub fn cblas_daxpby(n: blasint, alpha: c_double, x: *const c_double, incx: blasint, beta: c_double, y: *mut c_double, incy: blasint) -> ();
    pub fn cblas_caxpby(n: blasint, alpha: *const c_float, x: *const c_float, incx: blasint, beta: *const c_float, y: *mut c_float, incy: blasint) -> ();
    pub fn cblas_zaxpby(n: blasint, alpha: *const c_double, x: *const c_double, incx: blasint, beta: *const c_double, y: *mut c_double, incy: blasint) -> ();
    pub fn cblas_somatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: blasint, ccols: blasint, calpha: c_float, a: *const c_float, clda: blasint, b: *mut c_float, cldb: blasint) -> ();
    pub fn cblas_domatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: blasint, ccols: blasint, calpha: c_double, a: *const c_double, clda: blasint, b: *mut c_double, cldb: blasint) -> ();
    pub fn cblas_comatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: blasint, ccols: blasint, calpha: *const c_float, a: *const c_float, clda: blasint, b: *mut c_float, cldb: blasint) -> ();
    pub fn cblas_zomatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: blasint, ccols: blasint, calpha: *const c_double, a: *const c_double, clda: blasint, b: *mut c_double, cldb: blasint) -> ();
    pub fn cblas_simatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: blasint, ccols: blasint, calpha: c_float, a: *mut c_float, clda: blasint, cldb: blasint) -> ();
    pub fn cblas_dimatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: blasint, ccols: blasint, calpha: c_double, a: *mut c_double, clda: blasint, cldb: blasint) -> ();
    pub fn cblas_cimatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: blasint, ccols: blasint, calpha: *const c_float, a: *mut c_float, clda: blasint, cldb: blasint) -> ();
    pub fn cblas_zimatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: blasint, ccols: blasint, calpha: *const c_double, a: *mut c_double, clda: blasint, cldb: blasint) -> ();
}

// Level 1
//
// http://www.netlib.org/blas/#_level_1
extern "C" {
    // Single
    pub fn srotg_(a: *mut c_float, b: *mut c_float, c: *mut c_float, s: *mut c_float);
    pub fn srotmg_(d1: *mut c_float, d2: *mut c_float, x1: *mut c_float, y1: *const c_float,
                   param: *mut c_float);
    pub fn srot_(n: *const c_int, x: *mut c_float, incx: *const c_int, y: *mut c_float,
                 incy: *const c_int, c: *const c_float, s: *const c_float);
    pub fn srotm_(n: *const c_int, x: *mut c_float, incx: *const c_int, y: *mut c_float,
                  incy: *const c_int, param: *const c_float);
    pub fn sswap_(n: *const c_int, x: *mut c_float, incx: *const c_int, y: *mut c_float,
                  incy: *const c_int);
    pub fn sscal_(n: *const c_int, a: *const c_float, x: *mut c_float, incx: *const c_int);
    pub fn scopy_(n: *const c_int, x: *const c_float, incx: *const c_int, y: *mut c_float,
                  incy: *const c_int);
    pub fn saxpy_(n: *const c_int, alpha: *const c_float, x: *const c_float, incx: *const c_int,
                  y: *mut c_float, incy: *const c_int);
    pub fn sdot_(n: *const c_int, x: *const c_float, incx: *const c_int, y: *const c_float,
                 incy: *const c_int) -> c_float;
    pub fn sdsdot_(n: *const c_int, sb: *const c_float, x: *const c_float, incx: *const c_int,
                   y: *const c_float, incy: *const c_int) -> c_float;
    pub fn snrm2_(n: *const c_int, x: *const c_float, incx: *const c_int) -> c_float;
    pub fn scnrm2_(n: *const c_int, x: *const float_complex, incx: *const c_int) -> c_float;
    pub fn sasum_(n: *const c_int, x: *const c_float, incx: *const c_int) -> c_float;
    pub fn isamax_(n: *const c_int, x: *const c_float, incx: *const c_int) -> c_int;

    // Double
    pub fn drotg_(a: *mut c_double, b: *mut c_double, c: *mut c_double, s: *mut c_double);
    pub fn drotmg_(d1: *mut c_double, d2: *mut c_double, x1: *mut c_double, y1: *const c_double,
                   param: *mut c_double);
    pub fn drot_(n: *const c_int, x: *mut c_double, incx: *const c_int, y: *mut c_double,
                 incy: *const c_int, c: *const c_double, s: *const c_double);
    pub fn drotm_(n: *const c_int, x: *mut c_double, incx: *const c_int, y: *mut c_double,
                  incy: *const c_int, param: *const c_double);
    pub fn dswap_(n: *const c_int, x: *mut c_double, incx: *const c_int, y: *mut c_double,
                  incy: *const c_int);
    pub fn dscal_(n: *const c_int, a: *const c_double, x: *mut c_double, incx: *const c_int);
    pub fn dcopy_(n: *const c_int, x: *const c_double, incx: *const c_int, y: *mut c_double,
                  incy: *const c_int);
    pub fn daxpy_(n: *const c_int, alpha: *const c_double, x: *const c_double, incx: *const c_int,
                  y: *mut c_double, incy: *const c_int);
    pub fn ddot_(n: *const c_int, x: *const c_double, incx: *const c_int, y: *const c_double,
                 incy: *const c_int) -> c_double;
    pub fn dsdot_(n: *const c_int, x: *const c_float, incx: *const c_int, y: *const c_float,
                  incy: *const c_int) -> c_double;
    pub fn dnrm2_(n: *const c_int, x: *const c_double, incx: *const c_int) -> c_double;
    pub fn dznrm2_(n: *const c_int, x: *const double_complex, incx: *const c_int) -> c_double;
    pub fn dasum_(n: *const c_int, x: *const c_double, incx: *const c_int) -> c_double;
    pub fn idamax_(n: *const c_int, x: *const c_double, incx: *const c_int) -> c_int;

    // Complex
    pub fn crotg_(a: *mut float_complex, b: *const float_complex, c: *mut c_float,
                  s: *mut float_complex);
    pub fn csrot_(n: *const c_int, x: *mut float_complex, incx: *const c_int,
                  y: *mut float_complex, incy: *const c_int, c: *const c_float, s: *const c_float);
    pub fn cswap_(n: *const c_int, x: *mut float_complex, incx: *const c_int,
                  y: *mut float_complex, incy: *const c_int);
    pub fn cscal_(n: *const c_int, a: *const float_complex, x: *mut float_complex,
                  incx: *const c_int);
    pub fn csscal_(n: *const c_int, a: *const c_float, x: *mut float_complex, incx: *const c_int);
    pub fn ccopy_(n: *const c_int, x: *const float_complex, incx: *const c_int,
                  y: *mut float_complex, incy: *const c_int);
    pub fn caxpy_(n: *const c_int, alpha: *const float_complex, x: *const float_complex,
                  incx: *const c_int, y: *mut float_complex, incy: *const c_int);
    pub fn cdotu_(pres: *mut float_complex, n: *const c_int, x: *const float_complex,
                  incx: *const c_int, y: *const float_complex, incy: *const c_int);
    pub fn cdotc_(pres: *mut float_complex, n: *const c_int, x: *const float_complex,
                  incx: *const c_int, y: *const float_complex, incy: *const c_int);
    pub fn scasum_(n: *const c_int, x: *const float_complex, incx: *const c_int) -> c_float;
    pub fn icamax_(n: *const c_int, x: *const float_complex, incx: *const c_int) -> c_int;

    // Double complex
    pub fn zrotg_(a: *mut double_complex, b: *const double_complex, c: *mut c_double,
                  s: *mut double_complex);
    pub fn zdrot_(n: *const c_int, x: *mut double_complex, incx: *const c_int,
                  y: *mut double_complex, incy: *const c_int, c: *const c_double,
                  s: *const c_double);
    pub fn zswap_(n: *const c_int, x: *mut double_complex, incx: *const c_int,
                  y: *mut double_complex, incy: *const c_int);
    pub fn zscal_(n: *const c_int, a: *const double_complex, x: *mut double_complex,
                  incx: *const c_int);
    pub fn zdscal_(n: *const c_int, a: *const c_double, x: *mut double_complex,
                   incx: *const c_int);
    pub fn zcopy_(n: *const c_int, x: *const double_complex, incx: *const c_int,
                  y: *mut double_complex, incy: *const c_int);
    pub fn zaxpy_(n: *const c_int, alpha: *const double_complex, x: *const double_complex,
                  incx: *const c_int, y: *mut double_complex, incy: *const c_int);
    pub fn zdotu_(pres: *mut double_complex, n: *const c_int, x: *const double_complex,
                  incx: *const c_int, y: *const double_complex, incy: *const c_int);
    pub fn zdotc_(pres: *mut double_complex, n: *const c_int, x: *const double_complex,
                  incx: *const c_int, y: *const double_complex, incy: *const c_int);
    pub fn dzasum_(n: *const c_int, x: *const double_complex, incx: *const c_int) -> c_double;
    pub fn izamax_(n: *const c_int, x: *const double_complex, incx: *const c_int) -> c_int;
}

// Level 2
//
// http://www.netlib.org/blas/#_level_2
extern "C" {
    // Single
    pub fn sgemv_(trans: *const c_char, m: *const c_int, n: *const c_int, alpha: *const c_float,
                  a: *const c_float, lda: *const c_int, x: *const c_float, incx: *const c_int,
                  beta: *const c_float, y: *mut c_float, incy: *const c_int);
    pub fn sgbmv_(trans: *const c_char, m: *const c_int, n: *const c_int, kl: *const c_int,
                  ku: *const c_int, alpha: *const c_float, a: *const c_float, lda: *const c_int,
                  x: *const c_float, incx: *const c_int, beta: *const c_float, y: *mut c_float,
                  incy: *const c_int);
    pub fn ssymv_(uplo: *const c_char, n: *const c_int, alpha: *const c_float, a: *const c_float,
                  lda: *const c_int, x: *const c_float, incx: *const c_int, beta: *const c_float,
                  y: *mut c_float, incy: *const c_int);
    pub fn ssbmv_(uplo: *const c_char, n: *const c_int, k: *const c_int, alpha: *const c_float,
                  a: *const c_float, lda: *const c_int, x: *const c_float, incx: *const c_int,
                  beta: *const c_float, y: *mut c_float, incy: *const c_int);
    pub fn sspmv_(uplo: *const c_char, n: *const c_int, alpha: *const c_float, ap: *const c_float,
                  x: *const c_float, incx: *const c_int, beta: *const c_float, y: *mut c_float,
                  incy: *const c_int);
    pub fn strmv_(uplo: *const c_char, transa: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const c_float, lda: *const c_int, b: *mut c_float, incx: *const c_int);
    pub fn stbmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int, a: *const c_float, lda: *const c_int, x: *mut c_float,
                  incx: *const c_int);
    pub fn stpmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const c_float, x: *mut c_float, incx: *const c_int);
    pub fn strsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const c_float, lda: *const c_int, x: *mut c_float, incx: *const c_int);
    pub fn stbsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int, a: *const c_float, lda: *const c_int, x: *mut c_float,
                  incx: *const c_int);
    pub fn stpsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const c_float, x: *mut c_float, incx: *const c_int);
    pub fn sger_(m: *const c_int, n: *const c_int, alpha: *const c_float, x: *const c_float,
                 incx: *const c_int, y: *const c_float, incy: *const c_int, a: *mut c_float,
                 lda: *const c_int);
    pub fn ssyr_(uplo: *const c_char, n: *const c_int, alpha: *const c_float, x: *const c_float,
                 incx: *const c_int, a: *mut c_float, lda: *const c_int);
    pub fn sspr_(uplo: *const c_char, n: *const c_int, alpha: *const c_float, x: *const c_float,
                 incx: *const c_int, ap: *mut c_float);
    pub fn ssyr2_(uplo: *const c_char, n: *const c_int, alpha: *const c_float, x: *const c_float,
                  incx: *const c_int, y: *const c_float, incy: *const c_int, a: *mut c_float,
                  lda: *const c_int);
    pub fn sspr2_(uplo: *const c_char, n: *const c_int, alpha: *const c_float, x: *const c_float,
                  incx: *const c_int, y: *const c_float, incy: *const c_int, ap: *mut c_float);

    // Double
    pub fn dgemv_(trans: *const c_char, m: *const c_int, n: *const c_int, alpha: *const c_double,
                  a: *const c_double, lda: *const c_int, x: *const c_double, incx: *const c_int,
                  beta: *const c_double, y: *mut c_double, incy: *const c_int);
    pub fn dgbmv_(trans: *const c_char, m: *const c_int, n: *const c_int, kl: *const c_int,
                  ku: *const c_int, alpha: *const c_double, a: *const c_double, lda: *const c_int,
                  x: *const c_double, incx: *const c_int, beta: *const c_double, y: *mut c_double,
                  incy: *const c_int);
    pub fn dsymv_(uplo: *const c_char, n: *const c_int, alpha: *const c_double, a: *const c_double,
                  lda: *const c_int, x: *const c_double, incx: *const c_int, beta: *const c_double,
                  y: *mut c_double, incy: *const c_int);
    pub fn dsbmv_(uplo: *const c_char, n: *const c_int, k: *const c_int, alpha: *const c_double,
                  a: *const c_double, lda: *const c_int, x: *const c_double, incx: *const c_int,
                  beta: *const c_double, y: *mut c_double, incy: *const c_int);
    pub fn dspmv_(uplo: *const c_char, n: *const c_int, alpha: *const c_double, ap: *const c_double,
                  x: *const c_double, incx: *const c_int, beta: *const c_double,
                  y: *mut c_double, incy: *const c_int);
    pub fn dtrmv_(uplo: *const c_char, transa: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const c_double, lda: *const c_int, b: *mut c_double, incx: *const c_int);
    pub fn dtbmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int, a: *const c_double, lda: *const c_int, x: *mut c_double,
                  incx: *const c_int);
    pub fn dtpmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const c_double, x: *mut c_double, incx: *const c_int);
    pub fn dtrsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const c_double, lda: *const c_int, x: *mut c_double, incx: *const c_int);
    pub fn dtbsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int, a: *const c_double, lda: *const c_int, x: *mut c_double,
                  incx: *const c_int);
    pub fn dtpsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const c_double, x: *mut c_double, incx: *const c_int);
    pub fn dger_(m: *const c_int, n: *const c_int, alpha: *const c_double, x: *const c_double,
                 incx: *const c_int, y: *const c_double, incy: *const c_int, a: *mut c_double,
                 lda: *const c_int);
    pub fn dsyr_(uplo: *const c_char, n: *const c_int, alpha: *const c_double, x: *const c_double,
                 incx: *const c_int, a: *mut c_double, lda: *const c_int);
    pub fn dspr_(uplo: *const c_char, n: *const c_int, alpha: *const c_double, x: *const c_double,
                 incx: *const c_int, ap: *mut c_double);
    pub fn dsyr2_(uplo: *const c_char, n: *const c_int, alpha: *const c_double, x: *const c_double,
                  incx: *const c_int, y: *const c_double, incy: *const c_int, a: *mut c_double,
                  lda: *const c_int);
    pub fn dspr2_(uplo: *const c_char, n: *const c_int, alpha: *const c_double, x: *const c_double,
                  incx: *const c_int, y: *const c_double, incy: *const c_int, ap: *mut c_double);

    // Complex
    pub fn cgemv_(trans: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const float_complex, a: *const float_complex, lda: *const c_int,
                  x: *const float_complex, incx: *const c_int, beta: *const float_complex,
                  y: *mut float_complex, incy: *const c_int);
    pub fn cgbmv_(trans: *const c_char, m: *const c_int, n: *const c_int, kl: *const c_int,
                  ku: *const c_int, alpha: *const float_complex, a: *const float_complex,
                  lda: *const c_int, x: *const float_complex, incx: *const c_int,
                  beta: *const float_complex, y: *mut float_complex, incy: *const c_int);
    pub fn chemv_(uplo: *const c_char, n: *const c_int, alpha: *const float_complex,
                  a: *const float_complex, lda: *const c_int, x: *const float_complex,
                  incx: *const c_int, beta: *const float_complex, y: *mut float_complex,
                  incy: *const c_int);
    pub fn chbmv_(uplo: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const float_complex, a: *const float_complex, lda: *const c_int,
                  x: *const float_complex, incx: *const c_int, beta: *const float_complex,
                  y: *mut float_complex, incy: *const c_int);
    pub fn chpmv_(uplo: *const c_char, n: *const c_int, alpha: *const float_complex,
                  ap: *const float_complex, x: *const float_complex, incx: *const c_int,
                  beta: *const float_complex, y: *mut float_complex, incy: *const c_int);
    pub fn ctrmv_(uplo: *const c_char, transa: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const float_complex, lda: *const c_int, b: *mut float_complex,
                  incx: *const c_int);
    pub fn ctbmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int, a: *const float_complex, lda: *const c_int,
                  x: *mut float_complex, incx: *const c_int);
    pub fn ctpmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const float_complex, x: *mut float_complex, incx: *const c_int);
    pub fn ctrsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const float_complex, lda: *const c_int, x: *mut float_complex,
                  incx: *const c_int);
    pub fn ctbsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int, a: *const float_complex, lda: *const c_int,
                  x: *mut float_complex, incx: *const c_int);
    pub fn ctpsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const float_complex, x: *mut float_complex, incx: *const c_int);
    pub fn cgeru_(m: *const c_int, n: *const c_int, alpha: *const float_complex,
                  x: *const float_complex, incx: *const c_int, y: *const float_complex,
                  incy: *const c_int, a: *mut float_complex, lda: *const c_int);
    pub fn cgerc_(m: *const c_int, n: *const c_int, alpha: *const float_complex,
                  x: *const float_complex, incx: *const c_int, y: *const float_complex,
                  incy: *const c_int, a: *mut float_complex, lda: *const c_int);
    pub fn cher_(uplo: *const c_char, n: *const c_int, alpha: *const c_float,
                 x: *const float_complex, incx: *const c_int, a: *mut float_complex,
                 lda: *const c_int);
    pub fn chpr_(uplo: *const c_char, n: *const c_int, alpha: *const c_float,
                 x: *const float_complex, incx: *const c_int, ap: *mut float_complex);
    pub fn chpr2_(uplo: *const c_char, n: *const c_int, alpha: *const float_complex,
                  x: *const float_complex, incx: *const c_int, y: *const float_complex,
                  incy: *const c_int, ap: *mut float_complex);
    pub fn cher2_(uplo: *const c_char, n: *const c_int, alpha: *const float_complex, x: *const
                  float_complex, incx: *const c_int, y: *const float_complex, incy: *const c_int,
                  a: *mut float_complex, lda: *const c_int);

    // Double complex
    pub fn zgemv_(trans: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const double_complex, a: *const double_complex, lda: *const c_int,
                  x: *const double_complex, incx: *const c_int, beta: *const double_complex,
                  y: *mut double_complex, incy: *const c_int);
    pub fn zgbmv_(trans: *const c_char, m: *const c_int, n: *const c_int, kl: *const c_int,
                  ku: *const c_int, alpha: *const double_complex, a: *const double_complex,
                  lda: *const c_int, x: *const double_complex, incx: *const c_int,
                  beta: *const double_complex, y: *mut double_complex, incy: *const c_int);
    pub fn zhemv_(uplo: *const c_char, n: *const c_int, alpha: *const double_complex,
                  a: *const double_complex, lda: *const c_int, x: *const double_complex,
                  incx: *const c_int, beta: *const double_complex, y: *mut double_complex,
                  incy: *const c_int);
    pub fn zhbmv_(uplo: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const double_complex, a: *const double_complex, lda: *const c_int,
                  x: *const double_complex, incx: *const c_int, beta: *const double_complex,
                  y: *mut double_complex, incy: *const c_int);
    pub fn zhpmv_(uplo: *const c_char, n: *const c_int, alpha: *const double_complex,
                  ap: *const double_complex, x: *const double_complex, incx: *const c_int,
                  beta: *const double_complex, y: *mut double_complex, incy: *const c_int);
    pub fn ztrmv_(uplo: *const c_char, transa: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const double_complex, lda: *const c_int, b: *mut double_complex,
                  incx: *const c_int);
    pub fn ztbmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int,
                  a: *const double_complex, lda: *const c_int, x: *mut double_complex,
                  incx: *const c_int);
    pub fn ztpmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const double_complex, x: *mut double_complex, incx: *const c_int);
    pub fn ztrsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const double_complex, lda: *const c_int, x: *mut double_complex,
                  incx: *const c_int);
    pub fn ztbsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int, a: *const double_complex, lda: *const c_int,
                  x: *mut double_complex, incx: *const c_int);
    pub fn ztpsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const double_complex, x: *mut double_complex, incx: *const c_int);
    pub fn zgeru_(m: *const c_int, n: *const c_int, alpha: *const double_complex,
                  x: *const double_complex, incx: *const c_int, y: *const double_complex,
                  incy: *const c_int, a: *mut double_complex, lda: *const c_int);
    pub fn zgerc_(m: *const c_int, n: *const c_int, alpha: *const double_complex,
                  x: *const double_complex, incx: *const c_int, y: *const double_complex,
                  incy: *const c_int, a: *mut double_complex, lda: *const c_int);
    pub fn zher_(uplo: *const c_char, n: *const c_int, alpha: *const c_double,
                 x: *const double_complex, incx: *const c_int, a: *mut double_complex,
                 lda: *const c_int);
    pub fn zhpr_(uplo: *const c_char, n: *const c_int, alpha: *const c_double,
                 x: *const double_complex, incx: *const c_int, ap: *mut double_complex);
    pub fn zher2_(uplo: *const c_char, n: *const c_int, alpha: *const double_complex,
                  x: *const double_complex, incx: *const c_int, y: *const double_complex,
                  incy: *const c_int, a: *mut double_complex, lda: *const c_int);
    pub fn zhpr2_(uplo: *const c_char, n: *const c_int, alpha: *const double_complex,
                  x: *const double_complex, incx: *const c_int, y: *const double_complex,
                  incy: *const c_int, ap: *mut double_complex);
}

// Level 3
//
// http://www.netlib.org/blas/#_level_3
extern "C" {
    // Single
    pub fn sgemm_(transa: *const c_char, transb: *const c_char, m: *const c_int, n: *const c_int,
                  k: *const c_int, alpha: *const c_float, a: *const c_float, lda: *const c_int,
                  b: *const c_float, ldb: *const c_int,
                  beta: *const c_float, c: *mut c_float, ldc: *const c_int);
    pub fn ssymm_(side: *const c_char, uplo: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const c_float, a: *const c_float, lda: *const c_int, b: *const c_float,
                  ldb: *const c_int, beta: *const c_float, c: *mut c_float, ldc: *const c_int);
    pub fn ssyrk_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const c_float, a: *const c_float, lda: *const c_int, beta: *const c_float,
                  c: *mut c_float, ldc: *const c_int);
    pub fn ssyr2k_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                   alpha: *const c_float, a: *const c_float, lda: *const c_int, b: *const c_float,
                   ldb: *const c_int, beta: *const c_float, c: *mut c_float, ldc: *const c_int);
    pub fn strmm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int, alpha: *const c_float,
                  a: *const c_float, lda: *const c_int, b: *mut c_float, ldb: *const c_int);
    pub fn strsm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int, alpha: *const c_float,
                  a: *const c_float, lda: *const c_int, b: *mut c_float, ldb: *const c_int);

    // Double
    pub fn dgemm_(transa: *const c_char, transb: *const c_char, m: *const c_int, n: *const c_int,
                  k: *const c_int, alpha: *const c_double, a: *const c_double, lda: *const c_int,
                  b: *const c_double, ldb: *const c_int, beta: *const c_double, c: *mut c_double,
                  ldc: *const c_int);
    pub fn dsymm_(side: *const c_char, uplo: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const c_double, a: *const c_double, lda: *const c_int,
                  b: *const c_double, ldb: *const c_int, beta: *const c_double, c: *mut c_double,
                  ldc: *const c_int);
    pub fn dsyrk_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const c_double, a: *const c_double, lda: *const c_int,
                  beta: *const c_double, c: *mut c_double, ldc: *const c_int);
    pub fn dsyr2k_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                   alpha: *const c_double, a: *const c_double, lda: *const c_int,
                   b: *const c_double, ldb: *const c_int, beta: *const c_double, c: *mut c_double,
                   ldc: *const c_int);
    pub fn dtrmm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int, alpha: *const c_double,
                  a: *const c_double, lda: *const c_int, b: *mut c_double, ldb: *const c_int);
    pub fn dtrsm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int, alpha: *const c_double,
                  a: *const c_double, lda: *const c_int, b: *mut c_double, ldb: *const c_int);

    // Complex
    pub fn cgemm_(transa: *const c_char, transb: *const c_char, m: *const c_int, n: *const c_int,
                  k: *const c_int, alpha: *const float_complex, a: *const float_complex,
                  lda: *const c_int, b: *const float_complex, ldb: *const c_int,
                  beta: *const float_complex, c: *mut float_complex, ldc: *const c_int);
    pub fn csymm_(side: *const c_char, uplo: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const float_complex, a: *const float_complex, lda: *const c_int,
                  b: *const float_complex, ldb: *const c_int, beta: *const float_complex,
                  c: *mut float_complex, ldc: *const c_int);
    pub fn chemm_(side: *const c_char, uplo: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const float_complex, a: *const float_complex, lda: *const c_int,
                  b: *const float_complex, ldb: *const c_int, beta: *const float_complex,
                  c: *mut float_complex, ldc: *const c_int);
    pub fn csyrk_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const float_complex, a: *const float_complex, lda: *const c_int,
                  beta: *const float_complex, c: *mut float_complex, ldc: *const c_int);
    pub fn cherk_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const c_float, a: *const float_complex, lda: *const c_int,
                  beta: *const c_float, c: *mut float_complex, ldc: *const c_int);
    pub fn csyr2k_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                   alpha: *const float_complex, a: *const float_complex, lda: *const c_int,
                   b: *const float_complex, ldb: *const c_int, beta: *const float_complex,
                   c: *mut float_complex, ldc: *const c_int);
    pub fn cher2k_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                   alpha: *const float_complex, a: *const float_complex, lda: *const c_int,
                   b: *const float_complex, ldb: *const c_int, beta: *const c_float,
                   c: *mut float_complex, ldc: *const c_int);
    pub fn ctrmm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const float_complex, a: *const float_complex, lda: *const c_int,
                  b: *mut float_complex, ldb: *const c_int);
    pub fn ctrsm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const float_complex, a: *const float_complex, lda: *const c_int,
                  b: *mut float_complex, ldb: *const c_int);

    // Double complex
    pub fn zgemm_(transa: *const c_char, transb: *const c_char, m: *const c_int, n: *const c_int,
                  k: *const c_int, alpha: *const double_complex, a: *const double_complex,
                  lda: *const c_int, b: *const double_complex, ldb: *const c_int,
                  beta: *const double_complex, c: *mut double_complex, ldc: *const c_int);
    pub fn zsymm_(side: *const c_char, uplo: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const double_complex, a: *const double_complex, lda: *const c_int,
                  b: *const double_complex, ldb: *const c_int, beta: *const double_complex,
                  c: *mut double_complex, ldc: *const c_int);
    pub fn zhemm_(side: *const c_char, uplo: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const double_complex, a: *const double_complex, lda: *const c_int,
                  b: *const double_complex, ldb: *const c_int, beta: *const double_complex,
                  c: *mut double_complex, ldc: *const c_int);
    pub fn zsyrk_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const double_complex, a: *const double_complex, lda: *const c_int,
                  beta: *const double_complex, c: *mut double_complex, ldc: *const c_int);
    pub fn zherk_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const c_double, a: *const double_complex, lda: *const c_int,
                  beta: *const c_double, c: *mut double_complex, ldc: *const c_int);
    pub fn zsyr2k_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                   alpha: *const double_complex, a: *const double_complex, lda: *const c_int,
                   b: *const double_complex, ldb: *const c_int, beta: *const double_complex,
                   c: *mut double_complex, ldc: *const c_int);
    pub fn zher2k_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                   alpha: *const double_complex, a: *const double_complex, lda: *const c_int,
                   b: *const double_complex, ldb: *const c_int, beta: *const c_double,
                   c: *mut double_complex, ldc: *const c_int);
    pub fn ztrmm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const double_complex, a: *const double_complex, lda: *const c_int,
                  b: *mut double_complex, ldb: *const c_int);
    pub fn ztrsm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const double_complex, a: *const double_complex, lda: *const c_int,
                  b: *mut double_complex, ldb: *const c_int);
}
