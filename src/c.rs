#![allow(bad_style)]

use libc::{c_char, c_double, c_float, c_int, c_void};

pub type CBLAS_INDEX = c_int;

#[repr(C)]
pub enum CBLAS_LAYOUT {
    CblasRowMajor = 101,
    CblasColMajor = 102,
}
pub use self::CBLAS_LAYOUT::*;

#[repr(C)]
pub enum CBLAS_TRANSPOSE {
    CblasNoTrans = 111,
    CblasTrans = 112,
    CblasConjTrans = 113,
}
pub use self::CBLAS_TRANSPOSE::*;

#[repr(C)]
pub enum CBLAS_UPLO {
    CblasUpper = 121,
    CblasLower = 122,
}
pub use self::CBLAS_UPLO::*;

#[repr(C)]
pub enum CBLAS_DIAG {
    CblasNonUnit = 131,
    CblasUnit = 132,
}
pub use self::CBLAS_DIAG::*;

#[repr(C)]
pub enum CBLAS_SIDE {
    CblasLeft = 141,
    CblasRight = 142,
}
pub use self::CBLAS_SIDE::*;

pub type CBLAS_ORDER = CBLAS_LAYOUT;

// Prototypes for level 1 BLAS functions (complex are recast as routines)
extern "C" {
    pub fn cblas_dcabs1(z: *const c_void) -> c_double;
    pub fn cblas_scabs1(c: *const c_void) -> c_float;

    pub fn cblas_sdsdot(n: c_int, alpha: c_float, x: *const c_float,
                        incx: c_int, y: *const c_float, incy: c_int) -> c_float;
    pub fn cblas_dsdot(n: c_int, x: *const c_float, incx: c_int, y: *const c_float,
                       incy: c_int) -> c_double;
    pub fn cblas_sdot(n: c_int, x: *const c_float, incx: c_int,
                      y: *const c_float, incy: c_int) -> c_float;
    pub fn cblas_ddot(n: c_int, x: *const c_double, incx: c_int,
                      y: *const c_double, incy: c_int) -> c_double;
}

// Functions having prefixes Z and C only
extern "C" {
    pub fn cblas_cdotu_sub(n: c_int, x: *const c_void, incx: c_int,
                           y: *const c_void, incy: c_int, dotu: *mut c_void);
    pub fn cblas_cdotc_sub(n: c_int, x: *const c_void, incx: c_int,
                           y: *const c_void, incy: c_int, dotc: *mut c_void);

    pub fn cblas_zdotu_sub(n: c_int, x: *const c_void, incx: c_int,
                           y: *const c_void, incy: c_int, dotu: *mut c_void);
    pub fn cblas_zdotc_sub(n: c_int, x: *const c_void, incx: c_int,
                           y: *const c_void, incy: c_int, dotc: *mut c_void);
}

// Functions having prefixes S D SC DZ
extern "C" {
    pub fn cblas_snrm2(n: c_int, x: *const c_float, incx: c_int) -> c_float;
    pub fn cblas_sasum(n: c_int, x: *const c_float, incx: c_int) -> c_float;

    pub fn cblas_dnrm2(n: c_int, x: *const c_double, incx: c_int) -> c_double;
    pub fn cblas_dasum(n: c_int, x: *const c_double, incx: c_int) -> c_double;

    pub fn cblas_scnrm2(n: c_int, x: *const c_void, incx: c_int) -> c_float;
    pub fn cblas_scasum(n: c_int, x: *const c_void, incx: c_int) -> c_float;

    pub fn cblas_dznrm2(n: c_int, x: *const c_void, incx: c_int) -> c_double;
    pub fn cblas_dzasum(n: c_int, x: *const c_void, incx: c_int) -> c_double;
}

// Functions having standard 4 prefixes (S D C Z)
extern "C" {
    pub fn cblas_isamax(n: c_int, x: *const c_float, incx: c_int) -> CBLAS_INDEX;
    pub fn cblas_idamax(n: c_int, x: *const c_double, incx: c_int) -> CBLAS_INDEX;
    pub fn cblas_icamax(n: c_int, x: *const c_void, incx: c_int) -> CBLAS_INDEX;
    pub fn cblas_izamax(n: c_int, x: *const c_void, incx: c_int) -> CBLAS_INDEX;
}

// Prototypes for level 1 BLAS routines
//
// Routines with standard 4 prefixes (s, d, c, z)
extern "C" {
    pub fn cblas_sswap(n: c_int, x: *mut c_float, incx: c_int,
                       y: *mut c_float, incy: c_int);
    pub fn cblas_scopy(n: c_int, x: *const c_float, incx: c_int,
                       y: *mut c_float, incy: c_int);
    pub fn cblas_saxpy(n: c_int, alpha: c_float, x: *const c_float,
                       incx: c_int, y: *mut c_float, incy: c_int);

    pub fn cblas_dswap(n: c_int, x: *mut c_double, incx: c_int,
                       y: *mut c_double, incy: c_int);
    pub fn cblas_dcopy(n: c_int, x: *const c_double, incx: c_int,
                       y: *mut c_double, incy: c_int);
    pub fn cblas_daxpy(n: c_int, alpha: c_double, x: *const c_double,
                       incx: c_int, y: *mut c_double, incy: c_int);

    pub fn cblas_cswap(n: c_int, x: *mut c_void, incx: c_int,
                       y: *mut c_void, incy: c_int);
    pub fn cblas_ccopy(n: c_int, x: *const c_void, incx: c_int,
                       y: *mut c_void, incy: c_int);
    pub fn cblas_caxpy(n: c_int, alpha: *const c_void, x: *const c_void,
                       incx: c_int, y: *mut c_void, incy: c_int);

    pub fn cblas_zswap(n: c_int, x: *mut c_void, incx: c_int,
                       y: *mut c_void, incy: c_int);
    pub fn cblas_zcopy(n: c_int, x: *const c_void, incx: c_int,
                       y: *mut c_void, incy: c_int);
    pub fn cblas_zaxpy(n: c_int, alpha: *const c_void, x: *const c_void,
                       incx: c_int, y: *mut c_void, incy: c_int);
}

// Routines with S and D prefix only
extern "C" {
    pub fn cblas_srotg(a: *mut c_float, b: *mut c_float, c: *mut c_float, s: *mut c_float);
    pub fn cblas_srotmg(d1: *mut c_float, d2: *mut c_float, b1: *mut c_float, b2: c_float, p: *mut c_float);
    pub fn cblas_srot(n: c_int, x: *mut c_float, incx: c_int,
                      y: *mut c_float, incy: c_int, c: c_float, s: c_float);
    pub fn cblas_srotm(n: c_int, x: *mut c_float, incx: c_int,
                      y: *mut c_float, incy: c_int, p: *const c_float);

    pub fn cblas_drotg(a: *mut c_double, b: *mut c_double, c: *mut c_double, s: *mut c_double);
    pub fn cblas_drotmg(d1: *mut c_double, d2: *mut c_double, b1: *mut c_double, b2: c_double, p: *mut c_double);
    pub fn cblas_drot(n: c_int, x: *mut c_double, incx: c_int,
                      y: *mut c_double, incy: c_int, c: c_double, s: c_double);
    pub fn cblas_drotm(n: c_int, x: *mut c_double, incx: c_int,
                      y: *mut c_double, incy: c_int, p: *const c_double);
}

// Routines with S D C Z CS and ZD prefixes
extern "C" {
    pub fn cblas_sscal(n: c_int, alpha: c_float, x: *mut c_float, incx: c_int);
    pub fn cblas_dscal(n: c_int, alpha: c_double, x: *mut c_double, incx: c_int);
    pub fn cblas_cscal(n: c_int, alpha: *const c_void, x: *mut c_void, incx: c_int);
    pub fn cblas_zscal(n: c_int, alpha: *const c_void, x: *mut c_void, incx: c_int);
    pub fn cblas_csscal(n: c_int, alpha: c_float, x: *mut c_void, incx: c_int);
    pub fn cblas_zdscal(n: c_int, alpha: c_double, x: *mut c_void, incx: c_int);
}

// Prototypes for level 2 BLAS
//
// Routines with standard 4 prefixes (S, D, C, Z)
extern "C" {
    pub fn cblas_sgemv(layout: CBLAS_LAYOUT,
                       transa: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       alpha: c_float, a: *const c_float, lda: c_int,
                       x: *const c_float, incx: c_int, beta: c_float,
                       y: *mut c_float, incy: c_int);
    pub fn cblas_sgbmv(layout: CBLAS_LAYOUT,
                       transa: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       kl: c_int, ku: c_int, alpha: c_float,
                       a: *const c_float, lda: c_int, x: *const c_float,
                       incx: c_int, beta: c_float, y: *mut c_float, incy: c_int);
    pub fn cblas_strmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, a: *const c_float, lda: c_int,
                       x: *mut c_float, incx: c_int);
    pub fn cblas_stbmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, k: c_int, a: *const c_float, lda: c_int,
                       x: *mut c_float, incx: c_int);
    pub fn cblas_stpmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, ap: *const c_float, x: *mut c_float, incx: c_int);
    pub fn cblas_strsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, a: *const c_float, lda: c_int, x: *mut c_float,
                       incx: c_int);
    pub fn cblas_stbsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, k: c_int, a: *const c_float, lda: c_int,
                       x: *mut c_float, incx: c_int);
    pub fn cblas_stpsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, ap: *const c_float, x: *mut c_float, incx: c_int);

    pub fn cblas_dgemv(layout: CBLAS_LAYOUT,
                       transa: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       alpha: c_double, a: *const c_double, lda: c_int,
                       x: *const c_double, incx: c_int, beta: c_double,
                       y: *mut c_double, incy: c_int);
    pub fn cblas_dgbmv(layout: CBLAS_LAYOUT,
                       transa: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       kl: c_int, ku: c_int, alpha: c_double,
                       a: *const c_double, lda: c_int, x: *const c_double,
                       incx: c_int, beta: c_double, y: *mut c_double, incy: c_int);
    pub fn cblas_dtrmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, a: *const c_double, lda: c_int,
                       x: *mut c_double, incx: c_int);
    pub fn cblas_dtbmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, k: c_int, a: *const c_double, lda: c_int,
                       x: *mut c_double, incx: c_int);
    pub fn cblas_dtpmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, ap: *const c_double, x: *mut c_double, incx: c_int);
    pub fn cblas_dtrsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, a: *const c_double, lda: c_int, x: *mut c_double,
                       incx: c_int);
    pub fn cblas_dtbsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, k: c_int, a: *const c_double, lda: c_int,
                       x: *mut c_double, incx: c_int);
    pub fn cblas_dtpsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, ap: *const c_double, x: *mut c_double, incx: c_int);

    pub fn cblas_cgemv(layout: CBLAS_LAYOUT,
                       transa: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       x: *const c_void, incx: c_int, beta: *const c_void,
                       y: *mut c_void, incy: c_int);
    pub fn cblas_cgbmv(layout: CBLAS_LAYOUT,
                       transa: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       kl: c_int, ku: c_int, alpha: *const c_void,
                       a: *const c_void, lda: c_int, x: *const c_void,
                       incx: c_int, beta: *const c_void, y: *mut c_void, incy: c_int);
    pub fn cblas_ctrmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, a: *const c_void, lda: c_int,
                       x: *mut c_void, incx: c_int);
    pub fn cblas_ctbmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, k: c_int, a: *const c_void, lda: c_int,
                       x: *mut c_void, incx: c_int);
    pub fn cblas_ctpmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, ap: *const c_void, x: *mut c_void, incx: c_int);
    pub fn cblas_ctrsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, a: *const c_void, lda: c_int, x: *mut c_void,
                       incx: c_int);
    pub fn cblas_ctbsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, k: c_int, a: *const c_void, lda: c_int,
                       x: *mut c_void, incx: c_int);
    pub fn cblas_ctpsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, ap: *const c_void, x: *mut c_void, incx: c_int);

    pub fn cblas_zgemv(layout: CBLAS_LAYOUT,
                       transa: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       x: *const c_void, incx: c_int, beta: *const c_void,
                       y: *mut c_void, incy: c_int);
    pub fn cblas_zgbmv(layout: CBLAS_LAYOUT,
                       transa: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       kl: c_int, ku: c_int, alpha: *const c_void,
                       a: *const c_void, lda: c_int, x: *const c_void,
                       incx: c_int, beta: *const c_void, y: *mut c_void, incy: c_int);
    pub fn cblas_ztrmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, a: *const c_void, lda: c_int,
                       x: *mut c_void, incx: c_int);
    pub fn cblas_ztbmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, k: c_int, a: *const c_void, lda: c_int,
                       x: *mut c_void, incx: c_int);
    pub fn cblas_ztpmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, ap: *const c_void, x: *mut c_void, incx: c_int);
    pub fn cblas_ztrsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, a: *const c_void, lda: c_int, x: *mut c_void,
                       incx: c_int);
    pub fn cblas_ztbsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, k: c_int, a: *const c_void, lda: c_int,
                       x: *mut c_void, incx: c_int);
    pub fn cblas_ztpsv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       transa: CBLAS_TRANSPOSE, diag: CBLAS_DIAG,
                       n: c_int, ap: *const c_void, x: *mut c_void, incx: c_int);
}

// Routines with S and D prefixes only
extern "C" {
    pub fn cblas_ssymv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: c_float, a: *const c_float,
                       lda: c_int, x: *const c_float, incx: c_int,
                       beta: c_float, y: *mut c_float, incy: c_int);
    pub fn cblas_ssbmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, k: c_int, alpha: c_float, a: *const c_float,
                       lda: c_int, x: *const c_float, incx: c_int,
                       beta: c_float, y: *mut c_float, incy: c_int);
    pub fn cblas_sspmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: c_float, ap: *const c_float,
                       x: *const c_float, incx: c_int,
                       beta: c_float, y: *mut c_float, incy: c_int);
    pub fn cblas_sger(layout: CBLAS_LAYOUT, m: c_int, n: c_int,
                      alpha: c_float, x: *const c_float, incx: c_int,
                      y: *const c_float, incy: c_int, a: *mut c_float, lda: c_int);
    pub fn cblas_ssyr(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                      n: c_int, alpha: c_float, x: *const c_float,
                      incx: c_int, a: *mut c_float, lda: c_int);
    pub fn cblas_sspr(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                      n: c_int, alpha: c_float, x: *const c_float,
                      incx: c_int, ap: *mut c_float);
    pub fn cblas_ssyr2(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: c_float, x: *const c_float,
                       incx: c_int, y: *const c_float, incy: c_int, a: *mut c_float,
                       lda: c_int);
    pub fn cblas_sspr2(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: c_float, x: *const c_float,
                       incx: c_int, y: *const c_float, incy: c_int, a: *mut c_float);

    pub fn cblas_dsymv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: c_double, a: *const c_double,
                       lda: c_int, x: *const c_double, incx: c_int,
                       beta: c_double, y: *mut c_double, incy: c_int);
    pub fn cblas_dsbmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, k: c_int, alpha: c_double, a: *const c_double,
                       lda: c_int, x: *const c_double, incx: c_int,
                       beta: c_double, y: *mut c_double, incy: c_int);
    pub fn cblas_dspmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: c_double, ap: *const c_double,
                       x: *const c_double, incx: c_int,
                       beta: c_double, y: *mut c_double, incy: c_int);
    pub fn cblas_dger(layout: CBLAS_LAYOUT, m: c_int, n: c_int,
                      alpha: c_double, x: *const c_double, incx: c_int,
                      y: *const c_double, incy: c_int, a: *mut c_double, lda: c_int);
    pub fn cblas_dsyr(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                      n: c_int, alpha: c_double, x: *const c_double,
                      incx: c_int, a: *mut c_double, lda: c_int);
    pub fn cblas_dspr(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                      n: c_int, alpha: c_double, x: *const c_double,
                      incx: c_int, ap: *mut c_double);
    pub fn cblas_dsyr2(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: c_double, x: *const c_double,
                       incx: c_int, y: *const c_double, incy: c_int, a: *mut c_double,
                       lda: c_int);
    pub fn cblas_dspr2(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: c_double, x: *const c_double,
                       incx: c_int, y: *const c_double, incy: c_int, a: *mut c_double);
}

// Routines with C and Z prefixes only
extern "C" {
    pub fn cblas_chemv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: *const c_void, a: *const c_void,
                       lda: c_int, x: *const c_void, incx: c_int,
                       beta: *const c_void, y: *mut c_void, incy: c_int);
    pub fn cblas_chbmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, k: c_int, alpha: *const c_void, a: *const c_void,
                       lda: c_int, x: *const c_void, incx: c_int,
                       beta: *const c_void, y: *mut c_void, incy: c_int);
    pub fn cblas_chpmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: *const c_void, ap: *const c_void,
                       x: *const c_void, incx: c_int,
                       beta: *const c_void, y: *mut c_void, incy: c_int);
    pub fn cblas_cgeru(layout: CBLAS_LAYOUT, m: c_int, n: c_int,
                       alpha: *const c_void, x: *const c_void, incx: c_int,
                       y: *const c_void, incy: c_int, a: *mut c_void, lda: c_int);
    pub fn cblas_cgerc(layout: CBLAS_LAYOUT, m: c_int, n: c_int,
                       alpha: *const c_void, x: *const c_void, incx: c_int,
                       y: *const c_void, incy: c_int, a: *mut c_void, lda: c_int);
    pub fn cblas_cher(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                      n: c_int, alpha: c_float, x: *const c_void, incx: c_int,
                      a: *mut c_void, lda: c_int);
    pub fn cblas_chpr(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                      n: c_int, alpha: c_float, x: *const c_void,
                      incx: c_int, a: *mut c_void);
    pub fn cblas_cher2(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO, n: c_int,
                       alpha: *const c_void, x: *const c_void, incx: c_int,
                       y: *const c_void, incy: c_int, a: *mut c_void, lda: c_int);
    pub fn cblas_chpr2(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO, n: c_int,
                       alpha: *const c_void, x: *const c_void, incx: c_int,
                       y: *const c_void, incy: c_int, ap: *mut c_void);

    pub fn cblas_zhemv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: *const c_void, a: *const c_void,
                       lda: c_int, x: *const c_void, incx: c_int,
                       beta: *const c_void, y: *mut c_void, incy: c_int);
    pub fn cblas_zhbmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, k: c_int, alpha: *const c_void, a: *const c_void,
                       lda: c_int, x: *const c_void, incx: c_int,
                       beta: *const c_void, y: *mut c_void, incy: c_int);
    pub fn cblas_zhpmv(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: *const c_void, ap: *const c_void,
                       x: *const c_void, incx: c_int,
                       beta: *const c_void, y: *mut c_void, incy: c_int);
    pub fn cblas_zgeru(layout: CBLAS_LAYOUT, m: c_int, n: c_int,
                       alpha: *const c_void, x: *const c_void, incx: c_int,
                       y: *const c_void, incy: c_int, a: *mut c_void, lda: c_int);
    pub fn cblas_zgerc(layout: CBLAS_LAYOUT, m: c_int, n: c_int,
                       alpha: *const c_void, x: *const c_void, incx: c_int,
                       y: *const c_void, incy: c_int, a: *mut c_void, lda: c_int);
    pub fn cblas_zher(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: c_double, x: *const c_void, incx: c_int,
                       a: *mut c_void, lda: c_int);
    pub fn cblas_zhpr(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       n: c_int, alpha: c_double, x: *const c_void,
                       incx: c_int, a: *mut c_void);
    pub fn cblas_zher2(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO, n: c_int,
                       alpha: *const c_void, x: *const c_void, incx: c_int,
                       y: *const c_void, incy: c_int, a: *mut c_void, lda: c_int);
    pub fn cblas_zhpr2(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO, n: c_int,
                       alpha: *const c_void, x: *const c_void, incx: c_int,
                       y: *const c_void, incy: c_int, ap: *mut c_void);
}

// Prototypes for level 3 BLAS
//
// Routines with standard 4 prefixes (S, D, C, Z)
extern "C" {
    pub fn cblas_sgemm(layout: CBLAS_LAYOUT, transa: CBLAS_TRANSPOSE,
                       transb: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       k: c_int, alpha: c_float, a: *const c_float,
                       lda: c_int, b: *const c_float, ldb: c_int,
                       beta: c_float, c: *mut c_float, ldc: c_int);
    pub fn cblas_ssymm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, m: c_int, n: c_int,
                       alpha: c_float, a: *const c_float, lda: c_int,
                       b: *const c_float, ldb: c_int, beta: c_float,
                       c: *mut c_float, ldc: c_int);
    pub fn cblas_ssyrk(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                       alpha: c_float, a: *const c_float, lda: c_int,
                       beta: c_float, c: *mut c_float, ldc: c_int);
    pub fn cblas_ssyr2k(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                        trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                        alpha: c_float, a: *const c_float, lda: c_int,
                        b: *const c_float, ldb: c_int, beta: c_float,
                        c: *mut c_float, ldc: c_int);
    pub fn cblas_strmm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE,
                       diag: CBLAS_DIAG, m: c_int, n: c_int,
                       alpha: c_float, a: *const c_float, lda: c_int,
                       b: *mut c_float, ldb: c_int);
    pub fn cblas_strsm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE,
                       diag: CBLAS_DIAG, m: c_int, n: c_int,
                       alpha: c_float, a: *const c_float, lda: c_int,
                       b: *mut c_float, ldb: c_int);

    pub fn cblas_dgemm(layout: CBLAS_LAYOUT, transa: CBLAS_TRANSPOSE,
                       transb: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       k: c_int, alpha: c_double, a: *const c_double,
                       lda: c_int, b: *const c_double, ldb: c_int,
                       beta: c_double, c: *mut c_double, ldc: c_int);
    pub fn cblas_dsymm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, m: c_int, n: c_int,
                       alpha: c_double, a: *const c_double, lda: c_int,
                       b: *const c_double, ldb: c_int, beta: c_double,
                       c: *mut c_double, ldc: c_int);
    pub fn cblas_dsyrk(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                       alpha: c_double, a: *const c_double, lda: c_int,
                       beta: c_double, c: *mut c_double, ldc: c_int);
    pub fn cblas_dsyr2k(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                        trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                        alpha: c_double, a: *const c_double, lda: c_int,
                        b: *const c_double, ldb: c_int, beta: c_double,
                        c: *mut c_double, ldc: c_int);
    pub fn cblas_dtrmm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE,
                       diag: CBLAS_DIAG, m: c_int, n: c_int,
                       alpha: c_double, a: *const c_double, lda: c_int,
                       b: *mut c_double, ldb: c_int);
    pub fn cblas_dtrsm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE,
                       diag: CBLAS_DIAG, m: c_int, n: c_int,
                       alpha: c_double, a: *const c_double, lda: c_int,
                       b: *mut c_double, ldb: c_int);

    pub fn cblas_cgemm(layout: CBLAS_LAYOUT, transa: CBLAS_TRANSPOSE,
                       transb: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       k: c_int, alpha: *const c_void, a: *const c_void,
                       lda: c_int, b: *const c_void, ldb: c_int,
                       beta: *const c_void, c: *mut c_void, ldc: c_int);
    pub fn cblas_csymm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, m: c_int, n: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       b: *const c_void, ldb: c_int, beta: *const c_void,
                       c: *mut c_void, ldc: c_int);
    pub fn cblas_csyrk(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       beta: *const c_void, c: *mut c_void, ldc: c_int);
    pub fn cblas_csyr2k(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                        trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                        alpha: *const c_void, a: *const c_void, lda: c_int,
                        b: *const c_void, ldb: c_int, beta: *const c_void,
                        c: *mut c_void, ldc: c_int);
    pub fn cblas_ctrmm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE,
                       diag: CBLAS_DIAG, m: c_int, n: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       b: *mut c_void, ldb: c_int);
    pub fn cblas_ctrsm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE,
                       diag: CBLAS_DIAG, m: c_int, n: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       b: *mut c_void, ldb: c_int);

    pub fn cblas_zgemm(layout: CBLAS_LAYOUT, transa: CBLAS_TRANSPOSE,
                       transb: CBLAS_TRANSPOSE, m: c_int, n: c_int,
                       k: c_int, alpha: *const c_void, a: *const c_void,
                       lda: c_int, b: *const c_void, ldb: c_int,
                       beta: *const c_void, c: *mut c_void, ldc: c_int);
    pub fn cblas_zsymm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, m: c_int, n: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       b: *const c_void, ldb: c_int, beta: *const c_void,
                       c: *mut c_void, ldc: c_int);
    pub fn cblas_zsyrk(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       beta: *const c_void, c: *mut c_void, ldc: c_int);
    pub fn cblas_zsyr2k(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                        trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                        alpha: *const c_void, a: *const c_void, lda: c_int,
                        b: *const c_void, ldb: c_int, beta: *const c_void,
                        c: *mut c_void, ldc: c_int);
    pub fn cblas_ztrmm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE,
                       diag: CBLAS_DIAG, m: c_int, n: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       b: *mut c_void, ldb: c_int);
    pub fn cblas_ztrsm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, transa: CBLAS_TRANSPOSE,
                       diag: CBLAS_DIAG, m: c_int, n: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       b: *mut c_void, ldb: c_int);
}

// Routines with prefixes C and Z only
extern "C" {
    pub fn cblas_chemm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, m: c_int, n: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       b: *const c_void, ldb: c_int, beta: *const c_void,
                       c: *mut c_void, ldc: c_int);
    pub fn cblas_cherk(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                       alpha: c_float, a: *const c_void, lda: c_int,
                       beta: c_float, c: *mut c_void, ldc: c_int);
    pub fn cblas_cher2k(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                        trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                        alpha: *const c_void, a: *const c_void, lda: c_int,
                        b: *const c_void, ldb: c_int, beta: c_float,
                        c: *mut c_void, ldc: c_int);

    pub fn cblas_zhemm(layout: CBLAS_LAYOUT, side: CBLAS_SIDE,
                       uplo: CBLAS_UPLO, m: c_int, n: c_int,
                       alpha: *const c_void, a: *const c_void, lda: c_int,
                       b: *const c_void, ldb: c_int, beta: *const c_void,
                       c: *mut c_void, ldc: c_int);
    pub fn cblas_zherk(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                       trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                       alpha: c_double, a: *const c_void, lda: c_int,
                       beta: c_double, c: *mut c_void, ldc: c_int);
    pub fn cblas_zher2k(layout: CBLAS_LAYOUT, uplo: CBLAS_UPLO,
                        trans: CBLAS_TRANSPOSE, n: c_int, k: c_int,
                        alpha: *const c_void, a: *const c_void, lda: c_int,
                        b: *const c_void, ldb: c_int, beta: c_double,
                        c: *mut c_void, ldc: c_int);
}

extern "C" {
    pub fn cblas_xerbla(p: c_int, rout: *const c_char, form: *const c_char, ...);
}
