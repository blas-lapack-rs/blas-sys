//! Bindings to the [Basic Linear Algebra Subprograms][1].
//!
//! [1]: http://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms

#![allow(bad_style)]

extern crate libc;

#[cfg(feature = "openblas")]
extern crate openblas_blas_provider as raw;

#[cfg(feature = "netlib")]
extern crate netlib_blas_provider as raw;

use libc::{c_uint, c_int, c_char, c_double, c_float, size_t};

#[cfg(feature = "openblas")]
pub type int = c_int;

#[cfg(feature = "netlib")]
pub type int = c_int;

pub type complex_float = [c_float; 2];
pub type complex_double = [c_double; 2];

#[cfg(feature = "openblas")]
pub type CBLAS_INDEX = size_t;

#[cfg(feature = "netlib")]
pub type CBLAS_INDEX = c_int;

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

// C interface. Level 1
//
// http://www.netlib.org/blas/#_level_1
extern "C" {
    // Single
    pub fn cblas_srotg(a: *mut c_float, b: *mut c_float, c: *mut c_float, s: *mut c_float);
    pub fn cblas_srotmg(d1: *mut c_float, d2: *mut c_float, b1: *mut c_float, b2: c_float, p: *mut c_float);
    pub fn cblas_srot(n: int, x: *mut c_float, incx: int, y: *mut c_float, incy: int, c: c_float, s: c_float);
    pub fn cblas_srotm(n: int, x: *mut c_float, incx: int, y: *mut c_float, incy: int, p: *const c_float);
    pub fn cblas_sswap(n: int, x: *mut c_float, incx: int, y: *mut c_float, incy: int);
    pub fn cblas_sscal(n: int, alpha: c_float, x: *mut c_float, incx: int);
    pub fn cblas_scopy(n: int, x: *const c_float, incx: int, y: *mut c_float, incy: int);
    pub fn cblas_saxpy(n: int, alpha: c_float, x: *const c_float, incx: int, y: *mut c_float, incy: int);
    pub fn cblas_sdot(n: int, x: *const c_float, incx: int, y: *const c_float, incy: int) -> c_float;
    pub fn cblas_sdsdot(n: int, alpha: c_float, x: *const c_float, incx: int, y: *const c_float, incy: int) -> c_float;
    pub fn cblas_snrm2(n: int, x: *const c_float, incx: int) -> c_float;
    pub fn cblas_scnrm2(n: int, x: *const c_float, incx: int) -> c_float;
    pub fn cblas_sasum(n: int, x: *const c_float, incx: int) -> c_float;
    pub fn cblas_isamax(n: int, x: *const c_float, incx: int) -> CBLAS_INDEX;

    // Double
    pub fn cblas_drotg(a: *mut c_double, b: *mut c_double, c: *mut c_double, s: *mut c_double);
    pub fn cblas_drotmg(d1: *mut c_double, d2: *mut c_double, b1: *mut c_double, b2: c_double, p: *mut c_double);
    pub fn cblas_drot(n: int, x: *mut c_double, incx: int, y: *mut c_double, incy: int, c: c_double, s: c_double);
    pub fn cblas_drotm(n: int, x: *mut c_double, incx: int, y: *mut c_double, incy: int, p: *const c_double);
    pub fn cblas_dswap(n: int, x: *mut c_double, incx: int, y: *mut c_double, incy: int);
    pub fn cblas_dscal(n: int, alpha: c_double, x: *mut c_double, incx: int);
    pub fn cblas_dcopy(n: int, x: *const c_double, incx: int, y: *mut c_double, incy: int);
    pub fn cblas_daxpy(n: int, alpha: c_double, x: *const c_double, incx: int, y: *mut c_double, incy: int);
    pub fn cblas_ddot(n: int, x: *const c_double, incx: int, y: *const c_double, incy: int) -> c_double;
    pub fn cblas_dsdot(n: int, x: *const c_float, incx: int, y: *const c_float, incy: int) -> c_double;
    pub fn cblas_dnrm2(n: int, x: *const c_double, incx: int) -> c_double;
    pub fn cblas_dznrm2(n: int, x: *const c_double, incx: int) -> c_double;
    pub fn cblas_dasum(n: int, x: *const c_double, incx: int) -> c_double;
    pub fn cblas_idamax(n: int, x: *const c_double, incx: int) -> CBLAS_INDEX;

    // Complex
    pub fn cblas_crotg(a: *mut c_float, b: *mut c_float, c: *mut c_float, s: *mut c_float);
    pub fn cblas_csrot(n: int, x: *mut c_float, incx: int, y: *mut c_float, incy: int, c: c_float, s: c_float);
    pub fn cblas_cswap(n: int, x: *mut c_float, incx: int, y: *mut c_float, incy: int);
    pub fn cblas_cscal(n: int, alpha: *const c_float, x: *mut c_float, incx: int);
    pub fn cblas_csscal(n: int, alpha: c_float, x: *mut c_float, incx: int);
    pub fn cblas_ccopy(n: int, x: *const c_float, incx: int, y: *mut c_float, incy: int);
    pub fn cblas_caxpy(n: int, alpha: *const c_float, x: *const c_float, incx: int, y: *mut c_float, incy: int);
    pub fn cblas_cdotu(n: int, x: *const c_float, incx: int, y: *const c_float, incy: int) -> complex_float;
    pub fn cblas_cdotc(n: int, x: *const c_float, incx: int, y: *const c_float, incy: int) -> complex_float;
    pub fn cblas_scasum(n: int, x: *const c_float, incx: int) -> c_float;
    pub fn cblas_icamax(n: int, x: *const c_float, incx: int) -> CBLAS_INDEX;

    // Double complex
    pub fn cblas_zrotg(a: *mut c_double, b: *mut c_double, c: *mut c_double, s: *mut c_double);
    pub fn cblas_zdrot(n: int, x: *mut c_double, incx: int, y: *mut c_double, incy: int, c: c_double, s: c_double);
    pub fn cblas_zswap(n: int, x: *mut c_double, incx: int, y: *mut c_double, incy: int);
    pub fn cblas_zscal(n: int, alpha: *const c_double, x: *mut c_double, incx: int);
    pub fn cblas_zdscal(n: int, alpha: c_double, x: *mut c_double, incx: int);
    pub fn cblas_zcopy(n: int, x: *const c_double, incx: int, y: *mut c_double, incy: int);
    pub fn cblas_zaxpy(n: int, alpha: *const c_double, x: *const c_double, incx: int, y: *mut c_double, incy: int);
    pub fn cblas_zdotu(n: int, x: *const c_double, incx: int, y: *const c_double, incy: int) -> complex_double;
    pub fn cblas_zdotc(n: int, x: *const c_double, incx: int, y: *const c_double, incy: int) -> complex_double;
    pub fn cblas_dzasum(n: int, x: *const c_double, incx: int) -> c_double;
    pub fn cblas_izamax(n: int, x: *const c_double, incx: int) -> CBLAS_INDEX;
}

// C interface. Level 2
//
// http://www.netlib.org/blas/#_level_2
extern "C" {
    // Single
    pub fn cblas_sgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: int, n: int, alpha: c_float, a: *const c_float, lda: int, x: *const c_float, incx: int, beta: c_float, y: *mut c_float, incy: int);
    pub fn cblas_sgbmv(order: CBLAS_ORDER, transa: CBLAS_TRANSPOSE, m: int, n: int, kl: int, ku: int, alpha: c_float, a: *const c_float, lda: int, x: *const c_float, incx: int, beta: c_float, y: *mut c_float, incy: int);
    pub fn cblas_ssymv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_float, a: *const c_float, lda: int, x: *const c_float, incx: int, beta: c_float, y: *mut c_float, incy: int);
    pub fn cblas_ssbmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, k: int, alpha: c_float, a: *const c_float, lda: int, x: *const c_float, incx: int, beta: c_float, y: *mut c_float, incy: int);
    pub fn cblas_sspmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_float, ap: *const c_float, x: *const c_float, incx: int, beta: c_float, y: *mut c_float, incy: int);
    pub fn cblas_strmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, a: *const c_float, lda: int, x: *mut c_float, incx: int);
    pub fn cblas_stbmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, k: int, a: *const c_float, lda: int, x: *mut c_float, incx: int);
    pub fn cblas_stpmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, ap: *const c_float, x: *mut c_float, incx: int);
    pub fn cblas_strsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, a: *const c_float, lda: int, x: *mut c_float, incx: int);
    pub fn cblas_stbsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, k: int, a: *const c_float, lda: int, x: *mut c_float, incx: int);
    pub fn cblas_stpsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, ap: *const c_float, x: *mut c_float, incx: int);
    pub fn cblas_sger(order: CBLAS_ORDER, m: int, n: int, alpha: c_float, x: *const c_float, incx: int, y: *const c_float, incy: int, a: *mut c_float, lda: int);
    pub fn cblas_ssyr(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_float, x: *const c_float, incx: int, a: *mut c_float, lda: int);
    pub fn cblas_sspr(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_float, x: *const c_float, incx: int, ap: *mut c_float);
    pub fn cblas_ssyr2(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_float, x: *const c_float, incx: int, y: *const c_float, incy: int, a: *mut c_float, lda: int);
    pub fn cblas_sspr2(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_float, x: *const c_float, incx: int, y: *const c_float, incy: int, a: *mut c_float);

    // Double
    pub fn cblas_dgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: int, n: int, alpha: c_double, a: *const c_double, lda: int, x: *const c_double, incx: int, beta: c_double, y: *mut c_double, incy: int);
    pub fn cblas_dgbmv(order: CBLAS_ORDER, transa: CBLAS_TRANSPOSE, m: int, n: int, kl: int, ku: int, alpha: c_double, a: *const c_double, lda: int, x: *const c_double, incx: int, beta: c_double, y: *mut c_double, incy: int);
    pub fn cblas_dsymv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_double, a: *const c_double, lda: int, x: *const c_double, incx: int, beta: c_double, y: *mut c_double, incy: int);
    pub fn cblas_dsbmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, k: int, alpha: c_double, a: *const c_double, lda: int, x: *const c_double, incx: int, beta: c_double, y: *mut c_double, incy: int);
    pub fn cblas_dspmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_double, ap: *const c_double, x: *const c_double, incx: int, beta: c_double, y: *mut c_double, incy: int);
    pub fn cblas_dtrmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, a: *const c_double, lda: int, x: *mut c_double, incx: int);
    pub fn cblas_dtbmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, k: int, a: *const c_double, lda: int, x: *mut c_double, incx: int);
    pub fn cblas_dtpmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, ap: *const c_double, x: *mut c_double, incx: int);
    pub fn cblas_dtrsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, a: *const c_double, lda: int, x: *mut c_double, incx: int);
    pub fn cblas_dtbsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, k: int, a: *const c_double, lda: int, x: *mut c_double, incx: int);
    pub fn cblas_dtpsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, ap: *const c_double, x: *mut c_double, incx: int);
    pub fn cblas_dger(order: CBLAS_ORDER, m: int, n: int, alpha: c_double, x: *const c_double, incx: int, y: *const c_double, incy: int, a: *mut c_double, lda: int);
    pub fn cblas_dsyr(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_double, x: *const c_double, incx: int, a: *mut c_double, lda: int);
    pub fn cblas_dspr(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_double, x: *const c_double, incx: int, ap: *mut c_double);
    pub fn cblas_dsyr2(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_double, x: *const c_double, incx: int, y: *const c_double, incy: int, a: *mut c_double, lda: int);
    pub fn cblas_dspr2(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_double, x: *const c_double, incx: int, y: *const c_double, incy: int, a: *mut c_double);

    // Complex
    pub fn cblas_cgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: int, n: int, alpha: *const c_float, a: *const c_float, lda: int, x: *const c_float, incx: int, beta: *const c_float, y: *mut c_float, incy: int);
    pub fn cblas_cgbmv(order: CBLAS_ORDER, transa: CBLAS_TRANSPOSE, m: int, n: int, kl: int, ku: int, alpha: *const c_float, a: *const c_float, lda: int, x: *const c_float, incx: int, beta: *const c_float, y: *mut c_float, incy: int);
    pub fn cblas_chemv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: *const c_float, a: *const c_float, lda: int, x: *const c_float, incx: int, beta: *const c_float, y: *mut c_float, incy: int);
    pub fn cblas_chbmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, k: int, alpha: *const c_float, a: *const c_float, lda: int, x: *const c_float, incx: int, beta: *const c_float, y: *mut c_float, incy: int);
    pub fn cblas_chpmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: *const c_float, ap: *const c_float, x: *const c_float, incx: int, beta: *const c_float, y: *mut c_float, incy: int);
    pub fn cblas_ctrmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, a: *const c_float, lda: int, x: *mut c_float, incx: int);
    pub fn cblas_ctbmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, k: int, a: *const c_float, lda: int, x: *mut c_float, incx: int);
    pub fn cblas_ctpmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, ap: *const c_float, x: *mut c_float, incx: int);
    pub fn cblas_ctrsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, a: *const c_float, lda: int, x: *mut c_float, incx: int);
    pub fn cblas_ctbsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, k: int, a: *const c_float, lda: int, x: *mut c_float, incx: int);
    pub fn cblas_ctpsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, ap: *const c_float, x: *mut c_float, incx: int);
    pub fn cblas_cgeru(order: CBLAS_ORDER, m: int, n: int, alpha: *const c_float, x: *const c_float, incx: int, y: *const c_float, incy: int, a: *mut c_float, lda: int);
    pub fn cblas_cgerc(order: CBLAS_ORDER, m: int, n: int, alpha: *const c_float, x: *const c_float, incx: int, y: *const c_float, incy: int, a: *mut c_float, lda: int);
    pub fn cblas_cher(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_float, x: *const c_float, incx: int, a: *mut c_float, lda: int);
    pub fn cblas_chpr(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_float, x: *const c_float, incx: int, a: *mut c_float);
    pub fn cblas_cher2(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: *const c_float, x: *const c_float, incx: int, y: *const c_float, incy: int, a: *mut c_float, lda: int);
    pub fn cblas_chpr2(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: *const c_float, x: *const c_float, incx: int, y: *const c_float, incy: int, ap: *mut c_float);

    // Double complex
    pub fn cblas_zgemv(order: CBLAS_ORDER, trans: CBLAS_TRANSPOSE, m: int, n: int, alpha: *const c_double, a: *const c_double, lda: int, x: *const c_double, incx: int, beta: *const c_double, y: *mut c_double, incy: int);
    pub fn cblas_zgbmv(order: CBLAS_ORDER, transa: CBLAS_TRANSPOSE, m: int, n: int, kl: int, ku: int, alpha: *const c_double, a: *const c_double, lda: int, x: *const c_double, incx: int, beta: *const c_double, y: *mut c_double, incy: int);
    pub fn cblas_zhemv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: *const c_double, a: *const c_double, lda: int, x: *const c_double, incx: int, beta: *const c_double, y: *mut c_double, incy: int);
    pub fn cblas_zhbmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, k: int, alpha: *const c_double, a: *const c_double, lda: int, x: *const c_double, incx: int, beta: *const c_double, y: *mut c_double, incy: int);
    pub fn cblas_zhpmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: *const c_double, ap: *const c_double, x: *const c_double, incx: int, beta: *const c_double, y: *mut c_double, incy: int);
    pub fn cblas_ztrmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, a: *const c_double, lda: int, x: *mut c_double, incx: int);
    pub fn cblas_ztbmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, k: int, a: *const c_double, lda: int, x: *mut c_double, incx: int);
    pub fn cblas_ztpmv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, ap: *const c_double, x: *mut c_double, incx: int);
    pub fn cblas_ztrsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, a: *const c_double, lda: int, x: *mut c_double, incx: int);
    pub fn cblas_ztbsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, k: int, a: *const c_double, lda: int, x: *mut c_double, incx: int);
    pub fn cblas_ztpsv(order: CBLAS_ORDER, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, n: int, ap: *const c_double, x: *mut c_double, incx: int);
    pub fn cblas_zgeru(order: CBLAS_ORDER, m: int, n: int, alpha: *const c_double, x: *const c_double, incx: int, y: *const c_double, incy: int, a: *mut c_double, lda: int);
    pub fn cblas_zgerc(order: CBLAS_ORDER, m: int, n: int, alpha: *const c_double, x: *const c_double, incx: int, y: *const c_double, incy: int, a: *mut c_double, lda: int);
    pub fn cblas_zher(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_double, x: *const c_double, incx: int, a: *mut c_double, lda: int);
    pub fn cblas_zhpr(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: c_double, x: *const c_double, incx: int, a: *mut c_double);
    pub fn cblas_zher2(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: *const c_double, x: *const c_double, incx: int, y: *const c_double, incy: int, a: *mut c_double, lda: int);
    pub fn cblas_zhpr2(order: CBLAS_ORDER, uplo: CBLAS_UPLO, n: int, alpha: *const c_double, x: *const c_double, incx: int, y: *const c_double, incy: int, ap: *mut c_double);
}

// C interface. Level 3
//
// http://www.netlib.org/blas/#_level_3
extern "C" {
    // Single
    pub fn cblas_sgemm(order: CBLAS_ORDER, transa: CBLAS_TRANSPOSE, transb: CBLAS_TRANSPOSE, m: int, n: int, k: int, alpha: c_float, a: *const c_float, lda: int, b: *const c_float, ldb: int, beta: c_float, c: *mut c_float, ldc: int);
    pub fn cblas_ssymm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, m: int, n: int, alpha: c_float, a: *const c_float, lda: int, b: *const c_float, ldb: int, beta: c_float, c: *mut c_float, ldc: int);
    pub fn cblas_ssyrk(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: c_float, a: *const c_float, lda: int, beta: c_float, c: *mut c_float, ldc: int);
    pub fn cblas_ssyr2k(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: c_float, a: *const c_float, lda: int, b: *const c_float, ldb: int, beta: c_float, c: *mut c_float, ldc: int);
    pub fn cblas_strmm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, m: int, n: int, alpha: c_float, a: *const c_float, lda: int, b: *mut c_float, ldb: int);
    pub fn cblas_strsm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, m: int, n: int, alpha: c_float, a: *const c_float, lda: int, b: *mut c_float, ldb: int);

    // Double
    pub fn cblas_dgemm(order: CBLAS_ORDER, transa: CBLAS_TRANSPOSE, transb: CBLAS_TRANSPOSE, m: int, n: int, k: int, alpha: c_double, a: *const c_double, lda: int, b: *const c_double, ldb: int, beta: c_double, c: *mut c_double, ldc: int);
    pub fn cblas_dsymm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, m: int, n: int, alpha: c_double, a: *const c_double, lda: int, b: *const c_double, ldb: int, beta: c_double, c: *mut c_double, ldc: int);
    pub fn cblas_dsyrk(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: c_double, a: *const c_double, lda: int, beta: c_double, c: *mut c_double, ldc: int);
    pub fn cblas_dsyr2k(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: c_double, a: *const c_double, lda: int, b: *const c_double, ldb: int, beta: c_double, c: *mut c_double, ldc: int);
    pub fn cblas_dtrmm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, m: int, n: int, alpha: c_double, a: *const c_double, lda: int, b: *mut c_double, ldb: int);
    pub fn cblas_dtrsm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, m: int, n: int, alpha: c_double, a: *const c_double, lda: int, b: *mut c_double, ldb: int);

    // Complex
    pub fn cblas_cgemm(order: CBLAS_ORDER, transa: CBLAS_TRANSPOSE, transb: CBLAS_TRANSPOSE, m: int, n: int, k: int, alpha: *const c_float, a: *const c_float, lda: int, b: *const c_float, ldb: int, beta: *const c_float, c: *mut c_float, ldc: int);
    pub fn cblas_csymm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, m: int, n: int, alpha: *const c_float, a: *const c_float, lda: int, b: *const c_float, ldb: int, beta: *const c_float, c: *mut c_float, ldc: int);
    pub fn cblas_chemm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, m: int, n: int, alpha: *const c_float, a: *const c_float, lda: int, b: *const c_float, ldb: int, beta: *const c_float, c: *mut c_float, ldc: int);
    pub fn cblas_csyrk(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: *const c_float, a: *const c_float, lda: int, beta: *const c_float, c: *mut c_float, ldc: int);
    pub fn cblas_cherk(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: c_float, a: *const c_float, lda: int, beta: c_float, c: *mut c_float, ldc: int);
    pub fn cblas_csyr2k(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: *const c_float, a: *const c_float, lda: int, b: *const c_float, ldb: int, beta: *const c_float, c: *mut c_float, ldc: int);
    pub fn cblas_cher2k(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: *const c_float, a: *const c_float, lda: int, b: *const c_float, ldb: int, beta: c_float, c: *mut c_float, ldc: int);
    pub fn cblas_ctrmm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, m: int, n: int, alpha: *const c_float, a: *const c_float, lda: int, b: *mut c_float, ldb: int);
    pub fn cblas_ctrsm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, m: int, n: int, alpha: *const c_float, a: *const c_float, lda: int, b: *mut c_float, ldb: int);

    // Double complex
    pub fn cblas_zgemm(order: CBLAS_ORDER, transa: CBLAS_TRANSPOSE, transb: CBLAS_TRANSPOSE, m: int, n: int, k: int, alpha: *const c_double, a: *const c_double, lda: int, b: *const c_double, ldb: int, beta: *const c_double, c: *mut c_double, ldc: int);
    pub fn cblas_zsymm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, m: int, n: int, alpha: *const c_double, a: *const c_double, lda: int, b: *const c_double, ldb: int, beta: *const c_double, c: *mut c_double, ldc: int);
    pub fn cblas_zhemm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, m: int, n: int, alpha: *const c_double, a: *const c_double, lda: int, b: *const c_double, ldb: int, beta: *const c_double, c: *mut c_double, ldc: int);
    pub fn cblas_zsyrk(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: *const c_double, a: *const c_double, lda: int, beta: *const c_double, c: *mut c_double, ldc: int);
    pub fn cblas_zherk(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: c_double, a: *const c_double, lda: int, beta: c_double, c: *mut c_double, ldc: int);
    pub fn cblas_zsyr2k(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: *const c_double, a: *const c_double, lda: int, b: *const c_double, ldb: int, beta: *const c_double, c: *mut c_double, ldc: int);
    pub fn cblas_zher2k(order: CBLAS_ORDER, uplo: CBLAS_UPLO, trans: CBLAS_TRANSPOSE, n: int, k: int, alpha: *const c_double, a: *const c_double, lda: int, b: *const c_double, ldb: int, beta: c_double, c: *mut c_double, ldc: int);
    pub fn cblas_ztrmm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, m: int, n: int, alpha: *const c_double, a: *const c_double, lda: int, b: *mut c_double, ldb: int);
    pub fn cblas_ztrsm(order: CBLAS_ORDER, side: CBLAS_SIDE, uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG, m: int, n: int, alpha: *const c_double, a: *const c_double, lda: int, b: *mut c_double, ldb: int);
}

// C interface. Unclassified
extern "C" {
    pub fn cblas_xerbla(p: int, rout: *mut c_char, form: *mut c_char, ...);

    pub fn cblas_crot(n: int, x: *mut c_float, incx: int, y: *mut c_float, incy: int, c: c_float, s: *const c_float);
    pub fn cblas_zrot(n: int, x: *mut c_double, incx: int, y: *mut c_double, incy: int, c: c_double, s: *const c_double);

    pub fn cblas_cdotu_sub(n: int, x: *const c_float, incx: int, y: *const c_float, incy: int, ret: *mut complex_float);
    pub fn cblas_zdotu_sub(n: int, x: *const c_double, incx: int, y: *const c_double, incy: int, ret: *mut complex_double);

    pub fn cblas_cdotc_sub(n: int, x: *const c_float, incx: int, y: *const c_float, incy: int, ret: *mut complex_float);
    pub fn cblas_zdotc_sub(n: int, x: *const c_double, incx: int, y: *const c_double, incy: int, ret: *mut complex_double);

    pub fn cblas_saxpby(n: int, alpha: c_float, x: *const c_float, incx: int, beta: c_float, y: *mut c_float, incy: int);
    pub fn cblas_daxpby(n: int, alpha: c_double, x: *const c_double, incx: int, beta: c_double, y: *mut c_double, incy: int);
    pub fn cblas_caxpby(n: int, alpha: *const c_float, x: *const c_float, incx: int, beta: *const c_float, y: *mut c_float, incy: int);
    pub fn cblas_zaxpby(n: int, alpha: *const c_double, x: *const c_double, incx: int, beta: *const c_double, y: *mut c_double, incy: int);

    pub fn cblas_cgemm3m(order: CBLAS_ORDER, transa: CBLAS_TRANSPOSE, transb: CBLAS_TRANSPOSE, m: int, n: int, k: int, alpha: *const c_float, a: *const c_float, lda: int, b: *const c_float, ldb: int, beta: *const c_float, c: *mut c_float, ldc: int);
    pub fn cblas_zgemm3m(order: CBLAS_ORDER, transa: CBLAS_TRANSPOSE, transb: CBLAS_TRANSPOSE, m: int, n: int, k: int, alpha: *const c_double, a: *const c_double, lda: int, b: *const c_double, ldb: int, beta: *const c_double, c: *mut c_double, ldc: int);

    pub fn cblas_somatcopy(corder: CBLAS_ORDER, ctrans: CBLAS_TRANSPOSE, crows: int, ccols: int, calpha: c_float, a: *const c_float, clda: int, b: *mut c_float, cldb: int);
    pub fn cblas_domatcopy(corder: CBLAS_ORDER, ctrans: CBLAS_TRANSPOSE, crows: int, ccols: int, calpha: c_double, a: *const c_double, clda: int, b: *mut c_double, cldb: int);
    pub fn cblas_comatcopy(corder: CBLAS_ORDER, ctrans: CBLAS_TRANSPOSE, crows: int, ccols: int, calpha: *const c_float, a: *const c_float, clda: int, b: *mut c_float, cldb: int);
    pub fn cblas_zomatcopy(corder: CBLAS_ORDER, ctrans: CBLAS_TRANSPOSE, crows: int, ccols: int, calpha: *const c_double, a: *const c_double, clda: int, b: *mut c_double, cldb: int);

    pub fn cblas_simatcopy(corder: CBLAS_ORDER, ctrans: CBLAS_TRANSPOSE, crows: int, ccols: int, calpha: c_float, a: *mut c_float, clda: int, cldb: int);
    pub fn cblas_dimatcopy(corder: CBLAS_ORDER, ctrans: CBLAS_TRANSPOSE, crows: int, ccols: int, calpha: c_double, a: *mut c_double, clda: int, cldb: int);
    pub fn cblas_cimatcopy(corder: CBLAS_ORDER, ctrans: CBLAS_TRANSPOSE, crows: int, ccols: int, calpha: *const c_float, a: *mut c_float, clda: int, cldb: int);
    pub fn cblas_zimatcopy(corder: CBLAS_ORDER, ctrans: CBLAS_TRANSPOSE, crows: int, ccols: int, calpha: *const c_double, a: *mut c_double, clda: int, cldb: int);
}

// Fortran interface. Level 1
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

// Fortran interface. Level 2
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

// Fortran interface. Level 3
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
