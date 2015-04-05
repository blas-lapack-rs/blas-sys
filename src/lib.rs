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
pub struct complex_float {
    pub real: c_float,
    pub imag: c_float,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct complex_double {
    pub real: c_double,
    pub imag: c_double,
}

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
    pub fn cblas_sdsdot(n: c_int, alpha: c_float, x: *const c_float, incx: c_int, y: *const c_float, incy: c_int) -> c_float;
    pub fn cblas_dsdot(n: c_int, x: *const c_float, incx: c_int, y: *const c_float, incy: c_int) -> c_double;
    pub fn cblas_sdot(n: c_int, x: *const c_float, incx: c_int, y: *const c_float, incy: c_int) -> c_float;
    pub fn cblas_ddot(n: c_int, x: *const c_double, incx: c_int, y: *const c_double, incy: c_int) -> c_double;
    pub fn cblas_cdotu(n: c_int, x: *const c_float, incx: c_int, y: *const c_float, incy: c_int) -> complex_float;
    pub fn cblas_cdotc(n: c_int, x: *const c_float, incx: c_int, y: *const c_float, incy: c_int) -> complex_float;
    pub fn cblas_zdotu(n: c_int, x: *const c_double, incx: c_int, y: *const c_double, incy: c_int) -> complex_double;
    pub fn cblas_zdotc(n: c_int, x: *const c_double, incx: c_int, y: *const c_double, incy: c_int) -> complex_double;
    pub fn cblas_cdotu_sub(n: c_int, x: *const c_float, incx: c_int, y: *const c_float, incy: c_int, ret: *mut complex_float) -> ();
    pub fn cblas_cdotc_sub(n: c_int, x: *const c_float, incx: c_int, y: *const c_float, incy: c_int, ret: *mut complex_float) -> ();
    pub fn cblas_zdotu_sub(n: c_int, x: *const c_double, incx: c_int, y: *const c_double, incy: c_int, ret: *mut complex_double) -> ();
    pub fn cblas_zdotc_sub(n: c_int, x: *const c_double, incx: c_int, y: *const c_double, incy: c_int, ret: *mut complex_double) -> ();
    pub fn cblas_sasum(n: c_int, x: *const c_float, incx: c_int) -> c_float;
    pub fn cblas_dasum(n: c_int, x: *const c_double, incx: c_int) -> c_double;
    pub fn cblas_scasum(n: c_int, x: *const c_float, incx: c_int) -> c_float;
    pub fn cblas_dzasum(n: c_int, x: *const c_double, incx: c_int) -> c_double;
    pub fn cblas_snrm2(N: c_int, X: *const c_float, incX: c_int) -> c_float;
    pub fn cblas_dnrm2(N: c_int, X: *const c_double, incX: c_int) -> c_double;
    pub fn cblas_scnrm2(N: c_int, X: *const c_float, incX: c_int) -> c_float;
    pub fn cblas_dznrm2(N: c_int, X: *const c_double, incX: c_int) -> c_double;
    pub fn cblas_isamax(n: c_int, x: *const c_float, incx: c_int) -> size_t;
    pub fn cblas_idamax(n: c_int, x: *const c_double, incx: c_int) -> size_t;
    pub fn cblas_icamax(n: c_int, x: *const c_float, incx: c_int) -> size_t;
    pub fn cblas_izamax(n: c_int, x: *const c_double, incx: c_int) -> size_t;
    pub fn cblas_saxpy(n: c_int, alpha: c_float, x: *const c_float, incx: c_int, y: *mut c_float, incy: c_int) -> ();
    pub fn cblas_daxpy(n: c_int, alpha: c_double, x: *const c_double, incx: c_int, y: *mut c_double, incy: c_int) -> ();
    pub fn cblas_caxpy(n: c_int, alpha: *const c_float, x: *const c_float, incx: c_int, y: *mut c_float, incy: c_int) -> ();
    pub fn cblas_zaxpy(n: c_int, alpha: *const c_double, x: *const c_double, incx: c_int, y: *mut c_double, incy: c_int) -> ();
    pub fn cblas_scopy(n: c_int, x: *const c_float, incx: c_int, y: *mut c_float, incy: c_int) -> ();
    pub fn cblas_dcopy(n: c_int, x: *const c_double, incx: c_int, y: *mut c_double, incy: c_int) -> ();
    pub fn cblas_ccopy(n: c_int, x: *const c_float, incx: c_int, y: *mut c_float, incy: c_int) -> ();
    pub fn cblas_zcopy(n: c_int, x: *const c_double, incx: c_int, y: *mut c_double, incy: c_int) -> ();
    pub fn cblas_sswap(n: c_int, x: *mut c_float, incx: c_int, y: *mut c_float, incy: c_int) -> ();
    pub fn cblas_dswap(n: c_int, x: *mut c_double, incx: c_int, y: *mut c_double, incy: c_int) -> ();
    pub fn cblas_cswap(n: c_int, x: *mut c_float, incx: c_int, y: *mut c_float, incy: c_int) -> ();
    pub fn cblas_zswap(n: c_int, x: *mut c_double, incx: c_int, y: *mut c_double, incy: c_int) -> ();
    pub fn cblas_srot(N: c_int, X: *mut c_float, incX: c_int, Y: *mut c_float, incY: c_int, c: c_float, s: c_float) -> ();
    pub fn cblas_drot(N: c_int, X: *mut c_double, incX: c_int, Y: *mut c_double, incY: c_int, c: c_double, s: c_double) -> ();
    pub fn cblas_crot(N: c_int, X: *mut c_float, incX: c_int, Y: *mut c_float, incY: c_int, c: c_float, s: *const c_float) -> ();
    pub fn cblas_zrot(N: c_int, X: *mut c_double, incX: c_int, Y: *mut c_double, incY: c_int, c: c_double, s: *const c_double) -> ();
    pub fn cblas_csrot(N: c_int, X: *mut c_float, incX: c_int, Y: *mut c_float, incY: c_int, c: c_float, s: c_float) -> ();
    pub fn cblas_zdrot(N: c_int, X: *mut c_double, incX: c_int, Y: *mut c_double, incY: c_int, c: c_double, s: c_double) -> ();
    pub fn cblas_srotg(a: *mut c_float, b: *mut c_float, c: *mut c_float, s: *mut c_float) -> ();
    pub fn cblas_drotg(a: *mut c_double, b: *mut c_double, c: *mut c_double, s: *mut c_double) -> ();
    pub fn cblas_crotg(a: *mut c_float, b: *mut c_float, c: *mut c_float, s: *mut c_float) -> ();
    pub fn cblas_zrotg(a: *mut c_double, b: *mut c_double, c: *mut c_double, s: *mut c_double) -> ();
    pub fn cblas_srotm(N: c_int, X: *mut c_float, incX: c_int, Y: *mut c_float, incY: c_int, P: *const c_float) -> ();
    pub fn cblas_drotm(N: c_int, X: *mut c_double, incX: c_int, Y: *mut c_double, incY: c_int, P: *const c_double) -> ();
    pub fn cblas_srotmg(d1: *mut c_float, d2: *mut c_float, b1: *mut c_float, b2: c_float, P: *mut c_float) -> ();
    pub fn cblas_drotmg(d1: *mut c_double, d2: *mut c_double, b1: *mut c_double, b2: c_double, P: *mut c_double) -> ();
    pub fn cblas_sscal(N: c_int, alpha: c_float, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_dscal(N: c_int, alpha: c_double, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_cscal(N: c_int, alpha: *const c_float, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_zscal(N: c_int, alpha: *const c_double, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_csscal(N: c_int, alpha: c_float, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_zdscal(N: c_int, alpha: c_double, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_sgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: c_int, n: c_int, alpha: c_float, a: *const c_float, lda: c_int, x: *const c_float, incx: c_int, beta: c_float, y: *mut c_float, incy: c_int) -> ();
    pub fn cblas_dgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: c_int, n: c_int, alpha: c_double, a: *const c_double, lda: c_int, x: *const c_double, incx: c_int, beta: c_double, y: *mut c_double, incy: c_int) -> ();
    pub fn cblas_cgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: c_int, n: c_int, alpha: *const c_float, a: *const c_float, lda: c_int, x: *const c_float, incx: c_int, beta: *const c_float, y: *mut c_float, incy: c_int) -> ();
    pub fn cblas_zgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: c_int, n: c_int, alpha: *const c_double, a: *const c_double, lda: c_int, x: *const c_double, incx: c_int, beta: *const c_double, y: *mut c_double, incy: c_int) -> ();
    pub fn cblas_sger(order: CBLAS_ORDER, M: c_int, N: c_int, alpha: c_float, X: *const c_float, incX: c_int, Y: *const c_float, incY: c_int, A: *mut c_float, lda: c_int) -> ();
    pub fn cblas_dger(order: CBLAS_ORDER, M: c_int, N: c_int, alpha: c_double, X: *const c_double, incX: c_int, Y: *const c_double, incY: c_int, A: *mut c_double, lda: c_int) -> ();
    pub fn cblas_cgeru(order: CBLAS_ORDER, M: c_int, N: c_int, alpha: *const c_float, X: *const c_float, incX: c_int, Y: *const c_float, incY: c_int, A: *mut c_float, lda: c_int) -> ();
    pub fn cblas_cgerc(order: CBLAS_ORDER, M: c_int, N: c_int, alpha: *const c_float, X: *const c_float, incX: c_int, Y: *const c_float, incY: c_int, A: *mut c_float, lda: c_int) -> ();
    pub fn cblas_zgeru(order: CBLAS_ORDER, M: c_int, N: c_int, alpha: *const c_double, X: *const c_double, incX: c_int, Y: *const c_double, incY: c_int, A: *mut c_double, lda: c_int) -> ();
    pub fn cblas_zgerc(order: CBLAS_ORDER, M: c_int, N: c_int, alpha: *const c_double, X: *const c_double, incX: c_int, Y: *const c_double, incY: c_int, A: *mut c_double, lda: c_int) -> ();
    pub fn cblas_strsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, A: *const c_float, lda: c_int, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_dtrsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, A: *const c_double, lda: c_int, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_ctrsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, A: *const c_float, lda: c_int, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_ztrsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, A: *const c_double, lda: c_int, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_strmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, A: *const c_float, lda: c_int, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_dtrmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, A: *const c_double, lda: c_int, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_ctrmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, A: *const c_float, lda: c_int, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_ztrmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, A: *const c_double, lda: c_int, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_ssyr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_float, X: *const c_float, incX: c_int, A: *mut c_float, lda: c_int) -> ();
    pub fn cblas_dsyr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_double, X: *const c_double, incX: c_int, A: *mut c_double, lda: c_int) -> ();
    pub fn cblas_cher(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_float, X: *const c_float, incX: c_int, A: *mut c_float, lda: c_int) -> ();
    pub fn cblas_zher(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_double, X: *const c_double, incX: c_int, A: *mut c_double, lda: c_int) -> ();
    pub fn cblas_ssyr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_float, X: *const c_float, incX: c_int, Y: *const c_float, incY: c_int, A: *mut c_float, lda: c_int) -> ();
    pub fn cblas_dsyr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_double, X: *const c_double, incX: c_int, Y: *const c_double, incY: c_int, A: *mut c_double, lda: c_int) -> ();
    pub fn cblas_cher2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: *const c_float, X: *const c_float, incX: c_int, Y: *const c_float, incY: c_int, A: *mut c_float, lda: c_int) -> ();
    pub fn cblas_zher2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: *const c_double, X: *const c_double, incX: c_int, Y: *const c_double, incY: c_int, A: *mut c_double, lda: c_int) -> ();
    pub fn cblas_sgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int, KL: c_int, KU: c_int, alpha: c_float, A: *const c_float, lda: c_int, X: *const c_float, incX: c_int, beta: c_float, Y: *mut c_float, incY: c_int) -> ();
    pub fn cblas_dgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int, KL: c_int, KU: c_int, alpha: c_double, A: *const c_double, lda: c_int, X: *const c_double, incX: c_int, beta: c_double, Y: *mut c_double, incY: c_int) -> ();
    pub fn cblas_cgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int, KL: c_int, KU: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, X: *const c_float, incX: c_int, beta: *const c_float, Y: *mut c_float, incY: c_int) -> ();
    pub fn cblas_zgbmv(order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int, KL: c_int, KU: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, X: *const c_double, incX: c_int, beta: *const c_double, Y: *mut c_double, incY: c_int) -> ();
    pub fn cblas_ssbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, K: c_int, alpha: c_float, A: *const c_float, lda: c_int, X: *const c_float, incX: c_int, beta: c_float, Y: *mut c_float, incY: c_int) -> ();
    pub fn cblas_dsbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, K: c_int, alpha: c_double, A: *const c_double, lda: c_int, X: *const c_double, incX: c_int, beta: c_double, Y: *mut c_double, incY: c_int) -> ();
    pub fn cblas_stbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, K: c_int, A: *const c_float, lda: c_int, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_dtbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, K: c_int, A: *const c_double, lda: c_int, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_ctbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, K: c_int, A: *const c_float, lda: c_int, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_ztbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, K: c_int, A: *const c_double, lda: c_int, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_stbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, K: c_int, A: *const c_float, lda: c_int, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_dtbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, K: c_int, A: *const c_double, lda: c_int, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_ctbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, K: c_int, A: *const c_float, lda: c_int, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_ztbsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, K: c_int, A: *const c_double, lda: c_int, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_stpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, Ap: *const c_float, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_dtpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, Ap: *const c_double, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_ctpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, Ap: *const c_float, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_ztpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, Ap: *const c_double, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_stpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, Ap: *const c_float, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_dtpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, Ap: *const c_double, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_ctpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, Ap: *const c_float, X: *mut c_float, incX: c_int) -> ();
    pub fn cblas_ztpsv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, N: c_int, Ap: *const c_double, X: *mut c_double, incX: c_int) -> ();
    pub fn cblas_ssymv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_float, A: *const c_float, lda: c_int, X: *const c_float, incX: c_int, beta: c_float, Y: *mut c_float, incY: c_int) -> ();
    pub fn cblas_dsymv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_double, A: *const c_double, lda: c_int, X: *const c_double, incX: c_int, beta: c_double, Y: *mut c_double, incY: c_int) -> ();
    pub fn cblas_chemv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, X: *const c_float, incX: c_int, beta: *const c_float, Y: *mut c_float, incY: c_int) -> ();
    pub fn cblas_zhemv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, X: *const c_double, incX: c_int, beta: *const c_double, Y: *mut c_double, incY: c_int) -> ();
    pub fn cblas_sspmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_float, Ap: *const c_float, X: *const c_float, incX: c_int, beta: c_float, Y: *mut c_float, incY: c_int) -> ();
    pub fn cblas_dspmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_double, Ap: *const c_double, X: *const c_double, incX: c_int, beta: c_double, Y: *mut c_double, incY: c_int) -> ();
    pub fn cblas_sspr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_float, X: *const c_float, incX: c_int, Ap: *mut c_float) -> ();
    pub fn cblas_dspr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_double, X: *const c_double, incX: c_int, Ap: *mut c_double) -> ();
    pub fn cblas_chpr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_float, X: *const c_float, incX: c_int, A: *mut c_float) -> ();
    pub fn cblas_zhpr(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_double, X: *const c_double, incX: c_int, A: *mut c_double) -> ();
    pub fn cblas_sspr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_float, X: *const c_float, incX: c_int, Y: *const c_float, incY: c_int, A: *mut c_float) -> ();
    pub fn cblas_dspr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: c_double, X: *const c_double, incX: c_int, Y: *const c_double, incY: c_int, A: *mut c_double) -> ();
    pub fn cblas_chpr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: *const c_float, X: *const c_float, incX: c_int, Y: *const c_float, incY: c_int, Ap: *mut c_float) -> ();
    pub fn cblas_zhpr2(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: *const c_double, X: *const c_double, incX: c_int, Y: *const c_double, incY: c_int, Ap: *mut c_double) -> ();
    pub fn cblas_chbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, K: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, X: *const c_float, incX: c_int, beta: *const c_float, Y: *mut c_float, incY: c_int) -> ();
    pub fn cblas_zhbmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, K: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, X: *const c_double, incX: c_int, beta: *const c_double, Y: *mut c_double, incY: c_int) -> ();
    pub fn cblas_chpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: *const c_float, Ap: *const c_float, X: *const c_float, incX: c_int, beta: *const c_float, Y: *mut c_float, incY: c_int) -> ();
    pub fn cblas_zhpmv(order: CBLAS_ORDER, Uplo: CBLAS_UPLO, N: c_int, alpha: *const c_double, Ap: *const c_double, X: *const c_double, incX: c_int, beta: *const c_double, Y: *mut c_double, incY: c_int) -> ();
    pub fn cblas_sgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: c_int, N: c_int, K: c_int, alpha: c_float, A: *const c_float, lda: c_int, B: *const c_float, ldb: c_int, beta: c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_dgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: c_int, N: c_int, K: c_int, alpha: c_double, A: *const c_double, lda: c_int, B: *const c_double, ldb: c_int, beta: c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_cgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: c_int, N: c_int, K: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, B: *const c_float, ldb: c_int, beta: *const c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_cgemm3m(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: c_int, N: c_int, K: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, B: *const c_float, ldb: c_int, beta: *const c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_zgemm(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: c_int, N: c_int, K: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, B: *const c_double, ldb: c_int, beta: *const c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_zgemm3m(Order: CBLAS_ORDER, TransA: CBLAS_TRANSPOSE, TransB: CBLAS_TRANSPOSE, M: c_int, N: c_int, K: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, B: *const c_double, ldb: c_int, beta: *const c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_ssymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: c_int, N: c_int, alpha: c_float, A: *const c_float, lda: c_int, B: *const c_float, ldb: c_int, beta: c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_dsymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: c_int, N: c_int, alpha: c_double, A: *const c_double, lda: c_int, B: *const c_double, ldb: c_int, beta: c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_csymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: c_int, N: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, B: *const c_float, ldb: c_int, beta: *const c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_zsymm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: c_int, N: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, B: *const c_double, ldb: c_int, beta: *const c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_ssyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: c_float, A: *const c_float, lda: c_int, beta: c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_dsyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: c_double, A: *const c_double, lda: c_int, beta: c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_csyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, beta: *const c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_zsyrk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, beta: *const c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_ssyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: c_float, A: *const c_float, lda: c_int, B: *const c_float, ldb: c_int, beta: c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_dsyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: c_double, A: *const c_double, lda: c_int, B: *const c_double, ldb: c_int, beta: c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_csyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, B: *const c_float, ldb: c_int, beta: *const c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_zsyr2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, B: *const c_double, ldb: c_int, beta: *const c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_strmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: c_int, N: c_int, alpha: c_float, A: *const c_float, lda: c_int, B: *mut c_float, ldb: c_int) -> ();
    pub fn cblas_dtrmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: c_int, N: c_int, alpha: c_double, A: *const c_double, lda: c_int, B: *mut c_double, ldb: c_int) -> ();
    pub fn cblas_ctrmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: c_int, N: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, B: *mut c_float, ldb: c_int) -> ();
    pub fn cblas_ztrmm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: c_int, N: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, B: *mut c_double, ldb: c_int) -> ();
    pub fn cblas_strsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: c_int, N: c_int, alpha: c_float, A: *const c_float, lda: c_int, B: *mut c_float, ldb: c_int) -> ();
    pub fn cblas_dtrsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: c_int, N: c_int, alpha: c_double, A: *const c_double, lda: c_int, B: *mut c_double, ldb: c_int) -> ();
    pub fn cblas_ctrsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: c_int, N: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, B: *mut c_float, ldb: c_int) -> ();
    pub fn cblas_ztrsm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG, M: c_int, N: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, B: *mut c_double, ldb: c_int) -> ();
    pub fn cblas_chemm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: c_int, N: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, B: *const c_float, ldb: c_int, beta: *const c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_zhemm(Order: CBLAS_ORDER, Side: CBLAS_SIDE, Uplo: CBLAS_UPLO, M: c_int, N: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, B: *const c_double, ldb: c_int, beta: *const c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_cherk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: c_float, A: *const c_float, lda: c_int, beta: c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_zherk(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: c_double, A: *const c_double, lda: c_int, beta: c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_cher2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: *const c_float, A: *const c_float, lda: c_int, B: *const c_float, ldb: c_int, beta: c_float, C: *mut c_float, ldc: c_int) -> ();
    pub fn cblas_zher2k(Order: CBLAS_ORDER, Uplo: CBLAS_UPLO, Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int, alpha: *const c_double, A: *const c_double, lda: c_int, B: *const c_double, ldb: c_int, beta: c_double, C: *mut c_double, ldc: c_int) -> ();
    pub fn cblas_xerbla(p: c_int, rout: *mut c_char, form: *mut c_char, ...) -> ();
    pub fn cblas_saxpby(n: c_int, alpha: c_float, x: *const c_float, incx: c_int, beta: c_float, y: *mut c_float, incy: c_int) -> ();
    pub fn cblas_daxpby(n: c_int, alpha: c_double, x: *const c_double, incx: c_int, beta: c_double, y: *mut c_double, incy: c_int) -> ();
    pub fn cblas_caxpby(n: c_int, alpha: *const c_float, x: *const c_float, incx: c_int, beta: *const c_float, y: *mut c_float, incy: c_int) -> ();
    pub fn cblas_zaxpby(n: c_int, alpha: *const c_double, x: *const c_double, incx: c_int, beta: *const c_double, y: *mut c_double, incy: c_int) -> ();
    pub fn cblas_somatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: c_int, ccols: c_int, calpha: c_float, a: *const c_float, clda: c_int, b: *mut c_float, cldb: c_int) -> ();
    pub fn cblas_domatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: c_int, ccols: c_int, calpha: c_double, a: *const c_double, clda: c_int, b: *mut c_double, cldb: c_int) -> ();
    pub fn cblas_comatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: c_int, ccols: c_int, calpha: *const c_float, a: *const c_float, clda: c_int, b: *mut c_float, cldb: c_int) -> ();
    pub fn cblas_zomatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: c_int, ccols: c_int, calpha: *const c_double, a: *const c_double, clda: c_int, b: *mut c_double, cldb: c_int) -> ();
    pub fn cblas_simatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: c_int, ccols: c_int, calpha: c_float, a: *mut c_float, clda: c_int, cldb: c_int) -> ();
    pub fn cblas_dimatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: c_int, ccols: c_int, calpha: c_double, a: *mut c_double, clda: c_int, cldb: c_int) -> ();
    pub fn cblas_cimatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: c_int, ccols: c_int, calpha: *const c_float, a: *mut c_float, clda: c_int, cldb: c_int) -> ();
    pub fn cblas_zimatcopy(CORDER: CBLAS_ORDER, CTRANS: CBLAS_TRANSPOSE, crows: c_int, ccols: c_int, calpha: *const c_double, a: *mut c_double, clda: c_int, cldb: c_int) -> ();
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
    pub fn scnrm2_(n: *const c_int, x: *const complex_float, incx: *const c_int) -> c_float;
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
    pub fn dznrm2_(n: *const c_int, x: *const complex_double, incx: *const c_int) -> c_double;
    pub fn dasum_(n: *const c_int, x: *const c_double, incx: *const c_int) -> c_double;
    pub fn idamax_(n: *const c_int, x: *const c_double, incx: *const c_int) -> c_int;

    // Complex
    pub fn crotg_(a: *mut complex_float, b: *const complex_float, c: *mut c_float,
                  s: *mut complex_float);
    pub fn csrot_(n: *const c_int, x: *mut complex_float, incx: *const c_int,
                  y: *mut complex_float, incy: *const c_int, c: *const c_float, s: *const c_float);
    pub fn cswap_(n: *const c_int, x: *mut complex_float, incx: *const c_int,
                  y: *mut complex_float, incy: *const c_int);
    pub fn cscal_(n: *const c_int, a: *const complex_float, x: *mut complex_float,
                  incx: *const c_int);
    pub fn csscal_(n: *const c_int, a: *const c_float, x: *mut complex_float, incx: *const c_int);
    pub fn ccopy_(n: *const c_int, x: *const complex_float, incx: *const c_int,
                  y: *mut complex_float, incy: *const c_int);
    pub fn caxpy_(n: *const c_int, alpha: *const complex_float, x: *const complex_float,
                  incx: *const c_int, y: *mut complex_float, incy: *const c_int);
    pub fn cdotu_(pres: *mut complex_float, n: *const c_int, x: *const complex_float,
                  incx: *const c_int, y: *const complex_float, incy: *const c_int);
    pub fn cdotc_(pres: *mut complex_float, n: *const c_int, x: *const complex_float,
                  incx: *const c_int, y: *const complex_float, incy: *const c_int);
    pub fn scasum_(n: *const c_int, x: *const complex_float, incx: *const c_int) -> c_float;
    pub fn icamax_(n: *const c_int, x: *const complex_float, incx: *const c_int) -> c_int;

    // Double complex
    pub fn zrotg_(a: *mut complex_double, b: *const complex_double, c: *mut c_double,
                  s: *mut complex_double);
    pub fn zdrot_(n: *const c_int, x: *mut complex_double, incx: *const c_int,
                  y: *mut complex_double, incy: *const c_int, c: *const c_double,
                  s: *const c_double);
    pub fn zswap_(n: *const c_int, x: *mut complex_double, incx: *const c_int,
                  y: *mut complex_double, incy: *const c_int);
    pub fn zscal_(n: *const c_int, a: *const complex_double, x: *mut complex_double,
                  incx: *const c_int);
    pub fn zdscal_(n: *const c_int, a: *const c_double, x: *mut complex_double,
                   incx: *const c_int);
    pub fn zcopy_(n: *const c_int, x: *const complex_double, incx: *const c_int,
                  y: *mut complex_double, incy: *const c_int);
    pub fn zaxpy_(n: *const c_int, alpha: *const complex_double, x: *const complex_double,
                  incx: *const c_int, y: *mut complex_double, incy: *const c_int);
    pub fn zdotu_(pres: *mut complex_double, n: *const c_int, x: *const complex_double,
                  incx: *const c_int, y: *const complex_double, incy: *const c_int);
    pub fn zdotc_(pres: *mut complex_double, n: *const c_int, x: *const complex_double,
                  incx: *const c_int, y: *const complex_double, incy: *const c_int);
    pub fn dzasum_(n: *const c_int, x: *const complex_double, incx: *const c_int) -> c_double;
    pub fn izamax_(n: *const c_int, x: *const complex_double, incx: *const c_int) -> c_int;
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
                  alpha: *const complex_float, a: *const complex_float, lda: *const c_int,
                  x: *const complex_float, incx: *const c_int, beta: *const complex_float,
                  y: *mut complex_float, incy: *const c_int);
    pub fn cgbmv_(trans: *const c_char, m: *const c_int, n: *const c_int, kl: *const c_int,
                  ku: *const c_int, alpha: *const complex_float, a: *const complex_float,
                  lda: *const c_int, x: *const complex_float, incx: *const c_int,
                  beta: *const complex_float, y: *mut complex_float, incy: *const c_int);
    pub fn chemv_(uplo: *const c_char, n: *const c_int, alpha: *const complex_float,
                  a: *const complex_float, lda: *const c_int, x: *const complex_float,
                  incx: *const c_int, beta: *const complex_float, y: *mut complex_float,
                  incy: *const c_int);
    pub fn chbmv_(uplo: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const complex_float, a: *const complex_float, lda: *const c_int,
                  x: *const complex_float, incx: *const c_int, beta: *const complex_float,
                  y: *mut complex_float, incy: *const c_int);
    pub fn chpmv_(uplo: *const c_char, n: *const c_int, alpha: *const complex_float,
                  ap: *const complex_float, x: *const complex_float, incx: *const c_int,
                  beta: *const complex_float, y: *mut complex_float, incy: *const c_int);
    pub fn ctrmv_(uplo: *const c_char, transa: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const complex_float, lda: *const c_int, b: *mut complex_float,
                  incx: *const c_int);
    pub fn ctbmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int, a: *const complex_float, lda: *const c_int,
                  x: *mut complex_float, incx: *const c_int);
    pub fn ctpmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const complex_float, x: *mut complex_float, incx: *const c_int);
    pub fn ctrsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const complex_float, lda: *const c_int, x: *mut complex_float,
                  incx: *const c_int);
    pub fn ctbsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int, a: *const complex_float, lda: *const c_int,
                  x: *mut complex_float, incx: *const c_int);
    pub fn ctpsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const complex_float, x: *mut complex_float, incx: *const c_int);
    pub fn cgeru_(m: *const c_int, n: *const c_int, alpha: *const complex_float,
                  x: *const complex_float, incx: *const c_int, y: *const complex_float,
                  incy: *const c_int, a: *mut complex_float, lda: *const c_int);
    pub fn cgerc_(m: *const c_int, n: *const c_int, alpha: *const complex_float,
                  x: *const complex_float, incx: *const c_int, y: *const complex_float,
                  incy: *const c_int, a: *mut complex_float, lda: *const c_int);
    pub fn cher_(uplo: *const c_char, n: *const c_int, alpha: *const c_float,
                 x: *const complex_float, incx: *const c_int, a: *mut complex_float,
                 lda: *const c_int);
    pub fn chpr_(uplo: *const c_char, n: *const c_int, alpha: *const c_float,
                 x: *const complex_float, incx: *const c_int, ap: *mut complex_float);
    pub fn chpr2_(uplo: *const c_char, n: *const c_int, alpha: *const complex_float,
                  x: *const complex_float, incx: *const c_int, y: *const complex_float,
                  incy: *const c_int, ap: *mut complex_float);
    pub fn cher2_(uplo: *const c_char, n: *const c_int, alpha: *const complex_float, x: *const
                  complex_float, incx: *const c_int, y: *const complex_float, incy: *const c_int,
                  a: *mut complex_float, lda: *const c_int);

    // Double complex
    pub fn zgemv_(trans: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const complex_double, a: *const complex_double, lda: *const c_int,
                  x: *const complex_double, incx: *const c_int, beta: *const complex_double,
                  y: *mut complex_double, incy: *const c_int);
    pub fn zgbmv_(trans: *const c_char, m: *const c_int, n: *const c_int, kl: *const c_int,
                  ku: *const c_int, alpha: *const complex_double, a: *const complex_double,
                  lda: *const c_int, x: *const complex_double, incx: *const c_int,
                  beta: *const complex_double, y: *mut complex_double, incy: *const c_int);
    pub fn zhemv_(uplo: *const c_char, n: *const c_int, alpha: *const complex_double,
                  a: *const complex_double, lda: *const c_int, x: *const complex_double,
                  incx: *const c_int, beta: *const complex_double, y: *mut complex_double,
                  incy: *const c_int);
    pub fn zhbmv_(uplo: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const complex_double, a: *const complex_double, lda: *const c_int,
                  x: *const complex_double, incx: *const c_int, beta: *const complex_double,
                  y: *mut complex_double, incy: *const c_int);
    pub fn zhpmv_(uplo: *const c_char, n: *const c_int, alpha: *const complex_double,
                  ap: *const complex_double, x: *const complex_double, incx: *const c_int,
                  beta: *const complex_double, y: *mut complex_double, incy: *const c_int);
    pub fn ztrmv_(uplo: *const c_char, transa: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const complex_double, lda: *const c_int, b: *mut complex_double,
                  incx: *const c_int);
    pub fn ztbmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int,
                  a: *const complex_double, lda: *const c_int, x: *mut complex_double,
                  incx: *const c_int);
    pub fn ztpmv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const complex_double, x: *mut complex_double, incx: *const c_int);
    pub fn ztrsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  a: *const complex_double, lda: *const c_int, x: *mut complex_double,
                  incx: *const c_int);
    pub fn ztbsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  k: *const c_int, a: *const complex_double, lda: *const c_int,
                  x: *mut complex_double, incx: *const c_int);
    pub fn ztpsv_(uplo: *const c_char, trans: *const c_char, diag: *const c_char, n: *const c_int,
                  ap: *const complex_double, x: *mut complex_double, incx: *const c_int);
    pub fn zgeru_(m: *const c_int, n: *const c_int, alpha: *const complex_double,
                  x: *const complex_double, incx: *const c_int, y: *const complex_double,
                  incy: *const c_int, a: *mut complex_double, lda: *const c_int);
    pub fn zgerc_(m: *const c_int, n: *const c_int, alpha: *const complex_double,
                  x: *const complex_double, incx: *const c_int, y: *const complex_double,
                  incy: *const c_int, a: *mut complex_double, lda: *const c_int);
    pub fn zher_(uplo: *const c_char, n: *const c_int, alpha: *const c_double,
                 x: *const complex_double, incx: *const c_int, a: *mut complex_double,
                 lda: *const c_int);
    pub fn zhpr_(uplo: *const c_char, n: *const c_int, alpha: *const c_double,
                 x: *const complex_double, incx: *const c_int, ap: *mut complex_double);
    pub fn zher2_(uplo: *const c_char, n: *const c_int, alpha: *const complex_double,
                  x: *const complex_double, incx: *const c_int, y: *const complex_double,
                  incy: *const c_int, a: *mut complex_double, lda: *const c_int);
    pub fn zhpr2_(uplo: *const c_char, n: *const c_int, alpha: *const complex_double,
                  x: *const complex_double, incx: *const c_int, y: *const complex_double,
                  incy: *const c_int, ap: *mut complex_double);
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
                  k: *const c_int, alpha: *const complex_float, a: *const complex_float,
                  lda: *const c_int, b: *const complex_float, ldb: *const c_int,
                  beta: *const complex_float, c: *mut complex_float, ldc: *const c_int);
    pub fn csymm_(side: *const c_char, uplo: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const complex_float, a: *const complex_float, lda: *const c_int,
                  b: *const complex_float, ldb: *const c_int, beta: *const complex_float,
                  c: *mut complex_float, ldc: *const c_int);
    pub fn chemm_(side: *const c_char, uplo: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const complex_float, a: *const complex_float, lda: *const c_int,
                  b: *const complex_float, ldb: *const c_int, beta: *const complex_float,
                  c: *mut complex_float, ldc: *const c_int);
    pub fn csyrk_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const complex_float, a: *const complex_float, lda: *const c_int,
                  beta: *const complex_float, c: *mut complex_float, ldc: *const c_int);
    pub fn cherk_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const c_float, a: *const complex_float, lda: *const c_int,
                  beta: *const c_float, c: *mut complex_float, ldc: *const c_int);
    pub fn csyr2k_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                   alpha: *const complex_float, a: *const complex_float, lda: *const c_int,
                   b: *const complex_float, ldb: *const c_int, beta: *const complex_float,
                   c: *mut complex_float, ldc: *const c_int);
    pub fn cher2k_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                   alpha: *const complex_float, a: *const complex_float, lda: *const c_int,
                   b: *const complex_float, ldb: *const c_int, beta: *const c_float,
                   c: *mut complex_float, ldc: *const c_int);
    pub fn ctrmm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const complex_float, a: *const complex_float, lda: *const c_int,
                  b: *mut complex_float, ldb: *const c_int);
    pub fn ctrsm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const complex_float, a: *const complex_float, lda: *const c_int,
                  b: *mut complex_float, ldb: *const c_int);

    // Double complex
    pub fn zgemm_(transa: *const c_char, transb: *const c_char, m: *const c_int, n: *const c_int,
                  k: *const c_int, alpha: *const complex_double, a: *const complex_double,
                  lda: *const c_int, b: *const complex_double, ldb: *const c_int,
                  beta: *const complex_double, c: *mut complex_double, ldc: *const c_int);
    pub fn zsymm_(side: *const c_char, uplo: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const complex_double, a: *const complex_double, lda: *const c_int,
                  b: *const complex_double, ldb: *const c_int, beta: *const complex_double,
                  c: *mut complex_double, ldc: *const c_int);
    pub fn zhemm_(side: *const c_char, uplo: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const complex_double, a: *const complex_double, lda: *const c_int,
                  b: *const complex_double, ldb: *const c_int, beta: *const complex_double,
                  c: *mut complex_double, ldc: *const c_int);
    pub fn zsyrk_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const complex_double, a: *const complex_double, lda: *const c_int,
                  beta: *const complex_double, c: *mut complex_double, ldc: *const c_int);
    pub fn zherk_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                  alpha: *const c_double, a: *const complex_double, lda: *const c_int,
                  beta: *const c_double, c: *mut complex_double, ldc: *const c_int);
    pub fn zsyr2k_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                   alpha: *const complex_double, a: *const complex_double, lda: *const c_int,
                   b: *const complex_double, ldb: *const c_int, beta: *const complex_double,
                   c: *mut complex_double, ldc: *const c_int);
    pub fn zher2k_(uplo: *const c_char, trans: *const c_char, n: *const c_int, k: *const c_int,
                   alpha: *const complex_double, a: *const complex_double, lda: *const c_int,
                   b: *const complex_double, ldb: *const c_int, beta: *const c_double,
                   c: *mut complex_double, ldc: *const c_int);
    pub fn ztrmm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const complex_double, a: *const complex_double, lda: *const c_int,
                  b: *mut complex_double, ldb: *const c_int);
    pub fn ztrsm_(side: *const c_char, uplo: *const c_char, transa: *const c_char,
                  diag: *const c_char, m: *const c_int, n: *const c_int,
                  alpha: *const complex_double, a: *const complex_double, lda: *const c_int,
                  b: *mut complex_double, ldb: *const c_int);
}
