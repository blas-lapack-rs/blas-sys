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

    pub fn cblas_sdsdot(N: c_int, alpha: c_float, X: *const c_float,
                        incX: c_int, Y: *const c_float, incY: c_int) -> c_float;
    pub fn cblas_dsdot(N: c_int, X: *const c_float, incX: c_int, Y: *const c_float,
                       incY: c_int) -> c_double;
    pub fn cblas_sdot(N: c_int, X: *const c_float, incX: c_int,
                      Y: *const c_float, incY: c_int) -> c_float;
    pub fn cblas_ddot(N: c_int, X: *const c_double, incX: c_int,
                      Y: *const c_double, incY: c_int) -> c_double;
}

// Functions having prefixes Z and C only
extern "C" {
    pub fn cblas_cdotu_sub(N: c_int, X: *const c_void, incX: c_int,
                           Y: *const c_void, incY: c_int, dotu: *mut c_void);
    pub fn cblas_cdotc_sub(N: c_int, X: *const c_void, incX: c_int,
                           Y: *const c_void, incY: c_int, dotc: *mut c_void);

    pub fn cblas_zdotu_sub(N: c_int, X: *const c_void, incX: c_int,
                           Y: *const c_void, incY: c_int, dotu: *mut c_void);
    pub fn cblas_zdotc_sub(N: c_int, X: *const c_void, incX: c_int,
                           Y: *const c_void, incY: c_int, dotc: *mut c_void);
}

// Functions having prefixes S D SC DZ
extern "C" {
    pub fn cblas_snrm2(N: c_int, X: *const c_float, incX: c_int) -> c_float;
    pub fn cblas_sasum(N: c_int, X: *const c_float, incX: c_int) -> c_float;

    pub fn cblas_dnrm2(N: c_int, X: *const c_double, incX: c_int) -> c_double;
    pub fn cblas_dasum(N: c_int, X: *const c_double, incX: c_int) -> c_double;

    pub fn cblas_scnrm2(N: c_int, X: *const c_void, incX: c_int) -> c_float;
    pub fn cblas_scasum(N: c_int, X: *const c_void, incX: c_int) -> c_float;

    pub fn cblas_dznrm2(N: c_int, X: *const c_void, incX: c_int) -> c_double;
    pub fn cblas_dzasum(N: c_int, X: *const c_void, incX: c_int) -> c_double;
}

// Functions having standard 4 prefixes (S D C Z)
extern "C" {
    pub fn cblas_isamax(N: c_int, X: *const c_float, incX: c_int) -> CBLAS_INDEX;
    pub fn cblas_idamax(N: c_int, X: *const c_double, incX: c_int) -> CBLAS_INDEX;
    pub fn cblas_icamax(N: c_int, X: *const c_void, incX: c_int) -> CBLAS_INDEX;
    pub fn cblas_izamax(N: c_int, X: *const c_void, incX: c_int) -> CBLAS_INDEX;
}

// Prototypes for level 1 BLAS routines
//
// Routines with standard 4 prefixes (s, d, c, z)
extern "C" {
    pub fn cblas_sswap(N: c_int, X: *mut c_float, incX: c_int,
                       Y: *mut c_float, incY: c_int);
    pub fn cblas_scopy(N: c_int, X: *const c_float, incX: c_int,
                       Y: *mut c_float, incY: c_int);
    pub fn cblas_saxpy(N: c_int, alpha: c_float, X: *const c_float,
                       incX: c_int, Y: *mut c_float, incY: c_int);

    pub fn cblas_dswap(N: c_int, X: *mut c_double, incX: c_int,
                       Y: *mut c_double, incY: c_int);
    pub fn cblas_dcopy(N: c_int, X: *const c_double, incX: c_int,
                       Y: *mut c_double, incY: c_int);
    pub fn cblas_daxpy(N: c_int, alpha: c_double, X: *const c_double,
                       incX: c_int, Y: *mut c_double, incY: c_int);

    pub fn cblas_cswap(N: c_int, X: *mut c_void, incX: c_int,
                       Y: *mut c_void, incY: c_int);
    pub fn cblas_ccopy(N: c_int, X: *const c_void, incX: c_int,
                       Y: *mut c_void, incY: c_int);
    pub fn cblas_caxpy(N: c_int, alpha: *const c_void, X: *const c_void,
                       incX: c_int, Y: *mut c_void, incY: c_int);

    pub fn cblas_zswap(N: c_int, X: *mut c_void, incX: c_int,
                       Y: *mut c_void, incY: c_int);
    pub fn cblas_zcopy(N: c_int, X: *const c_void, incX: c_int,
                       Y: *mut c_void, incY: c_int);
    pub fn cblas_zaxpy(N: c_int, alpha: *const c_void, X: *const c_void,
                       incX: c_int, Y: *mut c_void, incY: c_int);
}

// Routines with S and D prefix only
extern "C" {
    pub fn cblas_srotg(a: *mut c_float, b: *mut c_float, c: *mut c_float, s: *mut c_float);
    pub fn cblas_srotmg(d1: *mut c_float, d2: *mut c_float, b1: *mut c_float, b2: c_float, P: *mut c_float);
    pub fn cblas_srot(N: c_int, X: *mut c_float, incX: c_int,
                      Y: *mut c_float, incY: c_int, c: c_float, s: c_float);
    pub fn cblas_srotm(N: c_int, X: *mut c_float, incX: c_int,
                      Y: *mut c_float, incY: c_int, P: *const c_float);

    pub fn cblas_drotg(a: *mut c_double, b: *mut c_double, c: *mut c_double, s: *mut c_double);
    pub fn cblas_drotmg(d1: *mut c_double, d2: *mut c_double, b1: *mut c_double, b2: c_double, P: *mut c_double);
    pub fn cblas_drot(N: c_int, X: *mut c_double, incX: c_int,
                      Y: *mut c_double, incY: c_int, c: c_double, s: c_double);
    pub fn cblas_drotm(N: c_int, X: *mut c_double, incX: c_int,
                      Y: *mut c_double, incY: c_int, P: *const c_double);
}

// Routines with S D C Z CS and ZD prefixes
extern "C" {
    pub fn cblas_sscal(N: c_int, alpha: c_float, X: *mut c_float, incX: c_int);
    pub fn cblas_dscal(N: c_int, alpha: c_double, X: *mut c_double, incX: c_int);
    pub fn cblas_cscal(N: c_int, alpha: *const c_void, X: *mut c_void, incX: c_int);
    pub fn cblas_zscal(N: c_int, alpha: *const c_void, X: *mut c_void, incX: c_int);
    pub fn cblas_csscal(N: c_int, alpha: c_float, X: *mut c_void, incX: c_int);
    pub fn cblas_zdscal(N: c_int, alpha: c_double, X: *mut c_void, incX: c_int);
}

// Prototypes for level 2 BLAS
//
// Routines with standard 4 prefixes (S, D, C, Z)
extern "C" {
    pub fn cblas_sgemv(layout: CBLAS_LAYOUT,
                       TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       alpha: c_float, A: *const c_float, lda: c_int,
                       X: *const c_float, incX: c_int, beta: c_float,
                       Y: *mut c_float, incY: c_int);
    pub fn cblas_sgbmv(layout: CBLAS_LAYOUT,
                       TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       KL: c_int, KU: c_int, alpha: c_float,
                       A: *const c_float, lda: c_int, X: *const c_float,
                       incX: c_int, beta: c_float, Y: *mut c_float, incY: c_int);
    pub fn cblas_strmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, A: *const c_float, lda: c_int,
                       X: *mut c_float, incX: c_int);
    pub fn cblas_stbmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, K: c_int, A: *const c_float, lda: c_int,
                       X: *mut c_float, incX: c_int);
    pub fn cblas_stpmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, Ap: *const c_float, X: *mut c_float, incX: c_int);
    pub fn cblas_strsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, A: *const c_float, lda: c_int, X: *mut c_float,
                       incX: c_int);
    pub fn cblas_stbsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, K: c_int, A: *const c_float, lda: c_int,
                       X: *mut c_float, incX: c_int);
    pub fn cblas_stpsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, Ap: *const c_float, X: *mut c_float, incX: c_int);

    pub fn cblas_dgemv(layout: CBLAS_LAYOUT,
                       TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       alpha: c_double, A: *const c_double, lda: c_int,
                       X: *const c_double, incX: c_int, beta: c_double,
                       Y: *mut c_double, incY: c_int);
    pub fn cblas_dgbmv(layout: CBLAS_LAYOUT,
                       TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       KL: c_int, KU: c_int, alpha: c_double,
                       A: *const c_double, lda: c_int, X: *const c_double,
                       incX: c_int, beta: c_double, Y: *mut c_double, incY: c_int);
    pub fn cblas_dtrmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, A: *const c_double, lda: c_int,
                       X: *mut c_double, incX: c_int);
    pub fn cblas_dtbmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, K: c_int, A: *const c_double, lda: c_int,
                       X: *mut c_double, incX: c_int);
    pub fn cblas_dtpmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, Ap: *const c_double, X: *mut c_double, incX: c_int);
    pub fn cblas_dtrsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, A: *const c_double, lda: c_int, X: *mut c_double,
                       incX: c_int);
    pub fn cblas_dtbsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, K: c_int, A: *const c_double, lda: c_int,
                       X: *mut c_double, incX: c_int);
    pub fn cblas_dtpsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, Ap: *const c_double, X: *mut c_double, incX: c_int);

    pub fn cblas_cgemv(layout: CBLAS_LAYOUT,
                       TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       X: *const c_void, incX: c_int, beta: *const c_void,
                       Y: *mut c_void, incY: c_int);
    pub fn cblas_cgbmv(layout: CBLAS_LAYOUT,
                       TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       KL: c_int, KU: c_int, alpha: *const c_void,
                       A: *const c_void, lda: c_int, X: *const c_void,
                       incX: c_int, beta: *const c_void, Y: *mut c_void, incY: c_int);
    pub fn cblas_ctrmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, A: *const c_void, lda: c_int,
                       X: *mut c_void, incX: c_int);
    pub fn cblas_ctbmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, K: c_int, A: *const c_void, lda: c_int,
                       X: *mut c_void, incX: c_int);
    pub fn cblas_ctpmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, Ap: *const c_void, X: *mut c_void, incX: c_int);
    pub fn cblas_ctrsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, A: *const c_void, lda: c_int, X: *mut c_void,
                       incX: c_int);
    pub fn cblas_ctbsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, K: c_int, A: *const c_void, lda: c_int,
                       X: *mut c_void, incX: c_int);
    pub fn cblas_ctpsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, Ap: *const c_void, X: *mut c_void, incX: c_int);

    pub fn cblas_zgemv(layout: CBLAS_LAYOUT,
                       TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       X: *const c_void, incX: c_int, beta: *const c_void,
                       Y: *mut c_void, incY: c_int);
    pub fn cblas_zgbmv(layout: CBLAS_LAYOUT,
                       TransA: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       KL: c_int, KU: c_int, alpha: *const c_void,
                       A: *const c_void, lda: c_int, X: *const c_void,
                       incX: c_int, beta: *const c_void, Y: *mut c_void, incY: c_int);
    pub fn cblas_ztrmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, A: *const c_void, lda: c_int,
                       X: *mut c_void, incX: c_int);
    pub fn cblas_ztbmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, K: c_int, A: *const c_void, lda: c_int,
                       X: *mut c_void, incX: c_int);
    pub fn cblas_ztpmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, Ap: *const c_void, X: *mut c_void, incX: c_int);
    pub fn cblas_ztrsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, A: *const c_void, lda: c_int, X: *mut c_void,
                       incX: c_int);
    pub fn cblas_ztbsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, K: c_int, A: *const c_void, lda: c_int,
                       X: *mut c_void, incX: c_int);
    pub fn cblas_ztpsv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       TransA: CBLAS_TRANSPOSE, Diag: CBLAS_DIAG,
                       N: c_int, Ap: *const c_void, X: *mut c_void, incX: c_int);
}

// Routines with S and D prefixes only
extern "C" {
    pub fn cblas_ssymv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: c_float, A: *const c_float,
                       lda: c_int, X: *const c_float, incX: c_int,
                       beta: c_float, Y: *mut c_float, incY: c_int);
    pub fn cblas_ssbmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, K: c_int, alpha: c_float, A: *const c_float,
                       lda: c_int, X: *const c_float, incX: c_int,
                       beta: c_float, Y: *mut c_float, incY: c_int);
    pub fn cblas_sspmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: c_float, Ap: *const c_float,
                       X: *const c_float, incX: c_int,
                       beta: c_float, Y: *mut c_float, incY: c_int);
    pub fn cblas_sger(layout: CBLAS_LAYOUT, M: c_int, N: c_int,
                      alpha: c_float, X: *const c_float, incX: c_int,
                      Y: *const c_float, incY: c_int, A: *mut c_float, lda: c_int);
    pub fn cblas_ssyr(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                      N: c_int, alpha: c_float, X: *const c_float,
                      incX: c_int, A: *mut c_float, lda: c_int);
    pub fn cblas_sspr(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                      N: c_int, alpha: c_float, X: *const c_float,
                      incX: c_int, Ap: *mut c_float);
    pub fn cblas_ssyr2(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: c_float, X: *const c_float,
                       incX: c_int, Y: *const c_float, incY: c_int, A: *mut c_float,
                       lda: c_int);
    pub fn cblas_sspr2(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: c_float, X: *const c_float,
                       incX: c_int, Y: *const c_float, incY: c_int, A: *mut c_float);

    pub fn cblas_dsymv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: c_double, A: *const c_double,
                       lda: c_int, X: *const c_double, incX: c_int,
                       beta: c_double, Y: *mut c_double, incY: c_int);
    pub fn cblas_dsbmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, K: c_int, alpha: c_double, A: *const c_double,
                       lda: c_int, X: *const c_double, incX: c_int,
                       beta: c_double, Y: *mut c_double, incY: c_int);
    pub fn cblas_dspmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: c_double, Ap: *const c_double,
                       X: *const c_double, incX: c_int,
                       beta: c_double, Y: *mut c_double, incY: c_int);
    pub fn cblas_dger(layout: CBLAS_LAYOUT, M: c_int, N: c_int,
                      alpha: c_double, X: *const c_double, incX: c_int,
                      Y: *const c_double, incY: c_int, A: *mut c_double, lda: c_int);
    pub fn cblas_dsyr(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                      N: c_int, alpha: c_double, X: *const c_double,
                      incX: c_int, A: *mut c_double, lda: c_int);
    pub fn cblas_dspr(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                      N: c_int, alpha: c_double, X: *const c_double,
                      incX: c_int, Ap: *mut c_double);
    pub fn cblas_dsyr2(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: c_double, X: *const c_double,
                       incX: c_int, Y: *const c_double, incY: c_int, A: *mut c_double,
                       lda: c_int);
    pub fn cblas_dspr2(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: c_double, X: *const c_double,
                       incX: c_int, Y: *const c_double, incY: c_int, A: *mut c_double);
}

// Routines with C and Z prefixes only
extern "C" {
    pub fn cblas_chemv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: *const c_void, A: *const c_void,
                       lda: c_int, X: *const c_void, incX: c_int,
                       beta: *const c_void, Y: *mut c_void, incY: c_int);
    pub fn cblas_chbmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, K: c_int, alpha: *const c_void, A: *const c_void,
                       lda: c_int, X: *const c_void, incX: c_int,
                       beta: *const c_void, Y: *mut c_void, incY: c_int);
    pub fn cblas_chpmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: *const c_void, Ap: *const c_void,
                       X: *const c_void, incX: c_int,
                       beta: *const c_void, Y: *mut c_void, incY: c_int);
    pub fn cblas_cgeru(layout: CBLAS_LAYOUT, M: c_int, N: c_int,
                       alpha: *const c_void, X: *const c_void, incX: c_int,
                       Y: *const c_void, incY: c_int, A: *mut c_void, lda: c_int);
    pub fn cblas_cgerc(layout: CBLAS_LAYOUT, M: c_int, N: c_int,
                       alpha: *const c_void, X: *const c_void, incX: c_int,
                       Y: *const c_void, incY: c_int, A: *mut c_void, lda: c_int);
    pub fn cblas_cher(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                      N: c_int, alpha: c_float, X: *const c_void, incX: c_int,
                      A: *mut c_void, lda: c_int);
    pub fn cblas_chpr(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                      N: c_int, alpha: c_float, X: *const c_void,
                      incX: c_int, A: *mut c_void);
    pub fn cblas_cher2(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO, N: c_int,
                       alpha: *const c_void, X: *const c_void, incX: c_int,
                       Y: *const c_void, incY: c_int, A: *mut c_void, lda: c_int);
    pub fn cblas_chpr2(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO, N: c_int,
                       alpha: *const c_void, X: *const c_void, incX: c_int,
                       Y: *const c_void, incY: c_int, Ap: *mut c_void);

    pub fn cblas_zhemv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: *const c_void, A: *const c_void,
                       lda: c_int, X: *const c_void, incX: c_int,
                       beta: *const c_void, Y: *mut c_void, incY: c_int);
    pub fn cblas_zhbmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, K: c_int, alpha: *const c_void, A: *const c_void,
                       lda: c_int, X: *const c_void, incX: c_int,
                       beta: *const c_void, Y: *mut c_void, incY: c_int);
    pub fn cblas_zhpmv(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: *const c_void, Ap: *const c_void,
                       X: *const c_void, incX: c_int,
                       beta: *const c_void, Y: *mut c_void, incY: c_int);
    pub fn cblas_zgeru(layout: CBLAS_LAYOUT, M: c_int, N: c_int,
                       alpha: *const c_void, X: *const c_void, incX: c_int,
                       Y: *const c_void, incY: c_int, A: *mut c_void, lda: c_int);
    pub fn cblas_zgerc(layout: CBLAS_LAYOUT, M: c_int, N: c_int,
                       alpha: *const c_void, X: *const c_void, incX: c_int,
                       Y: *const c_void, incY: c_int, A: *mut c_void, lda: c_int);
    pub fn cblas_zher(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: c_double, X: *const c_void, incX: c_int,
                       A: *mut c_void, lda: c_int);
    pub fn cblas_zhpr(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       N: c_int, alpha: c_double, X: *const c_void,
                       incX: c_int, A: *mut c_void);
    pub fn cblas_zher2(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO, N: c_int,
                       alpha: *const c_void, X: *const c_void, incX: c_int,
                       Y: *const c_void, incY: c_int, A: *mut c_void, lda: c_int);
    pub fn cblas_zhpr2(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO, N: c_int,
                       alpha: *const c_void, X: *const c_void, incX: c_int,
                       Y: *const c_void, incY: c_int, Ap: *mut c_void);
}

// Prototypes for level 3 BLAS
//
// Routines with standard 4 prefixes (S, D, C, Z)
extern "C" {
    pub fn cblas_sgemm(layout: CBLAS_LAYOUT, TransA: CBLAS_TRANSPOSE,
                       TransB: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       K: c_int, alpha: c_float, A: *const c_float,
                       lda: c_int, B: *const c_float, ldb: c_int,
                       beta: c_float, C: *mut c_float, ldc: c_int);
    pub fn cblas_ssymm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, M: c_int, N: c_int,
                       alpha: c_float, A: *const c_float, lda: c_int,
                       B: *const c_float, ldb: c_int, beta: c_float,
                       C: *mut c_float, ldc: c_int);
    pub fn cblas_ssyrk(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                       alpha: c_float, A: *const c_float, lda: c_int,
                       beta: c_float, C: *mut c_float, ldc: c_int);
    pub fn cblas_ssyr2k(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                        Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                        alpha: c_float, A: *const c_float, lda: c_int,
                        B: *const c_float, ldb: c_int, beta: c_float,
                        C: *mut c_float, ldc: c_int);
    pub fn cblas_strmm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE,
                       Diag: CBLAS_DIAG, M: c_int, N: c_int,
                       alpha: c_float, A: *const c_float, lda: c_int,
                       B: *mut c_float, ldb: c_int);
    pub fn cblas_strsm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE,
                       Diag: CBLAS_DIAG, M: c_int, N: c_int,
                       alpha: c_float, A: *const c_float, lda: c_int,
                       B: *mut c_float, ldb: c_int);

    pub fn cblas_dgemm(layout: CBLAS_LAYOUT, TransA: CBLAS_TRANSPOSE,
                       TransB: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       K: c_int, alpha: c_double, A: *const c_double,
                       lda: c_int, B: *const c_double, ldb: c_int,
                       beta: c_double, C: *mut c_double, ldc: c_int);
    pub fn cblas_dsymm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, M: c_int, N: c_int,
                       alpha: c_double, A: *const c_double, lda: c_int,
                       B: *const c_double, ldb: c_int, beta: c_double,
                       C: *mut c_double, ldc: c_int);
    pub fn cblas_dsyrk(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                       alpha: c_double, A: *const c_double, lda: c_int,
                       beta: c_double, C: *mut c_double, ldc: c_int);
    pub fn cblas_dsyr2k(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                        Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                        alpha: c_double, A: *const c_double, lda: c_int,
                        B: *const c_double, ldb: c_int, beta: c_double,
                        C: *mut c_double, ldc: c_int);
    pub fn cblas_dtrmm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE,
                       Diag: CBLAS_DIAG, M: c_int, N: c_int,
                       alpha: c_double, A: *const c_double, lda: c_int,
                       B: *mut c_double, ldb: c_int);
    pub fn cblas_dtrsm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE,
                       Diag: CBLAS_DIAG, M: c_int, N: c_int,
                       alpha: c_double, A: *const c_double, lda: c_int,
                       B: *mut c_double, ldb: c_int);

    pub fn cblas_cgemm(layout: CBLAS_LAYOUT, TransA: CBLAS_TRANSPOSE,
                       TransB: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       K: c_int, alpha: *const c_void, A: *const c_void,
                       lda: c_int, B: *const c_void, ldb: c_int,
                       beta: *const c_void, C: *mut c_void, ldc: c_int);
    pub fn cblas_csymm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, M: c_int, N: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       B: *const c_void, ldb: c_int, beta: *const c_void,
                       C: *mut c_void, ldc: c_int);
    pub fn cblas_csyrk(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       beta: *const c_void, C: *mut c_void, ldc: c_int);
    pub fn cblas_csyr2k(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                        Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                        alpha: *const c_void, A: *const c_void, lda: c_int,
                        B: *const c_void, ldb: c_int, beta: *const c_void,
                        C: *mut c_void, ldc: c_int);
    pub fn cblas_ctrmm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE,
                       Diag: CBLAS_DIAG, M: c_int, N: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       B: *mut c_void, ldb: c_int);
    pub fn cblas_ctrsm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE,
                       Diag: CBLAS_DIAG, M: c_int, N: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       B: *mut c_void, ldb: c_int);

    pub fn cblas_zgemm(layout: CBLAS_LAYOUT, TransA: CBLAS_TRANSPOSE,
                       TransB: CBLAS_TRANSPOSE, M: c_int, N: c_int,
                       K: c_int, alpha: *const c_void, A: *const c_void,
                       lda: c_int, B: *const c_void, ldb: c_int,
                       beta: *const c_void, C: *mut c_void, ldc: c_int);
    pub fn cblas_zsymm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, M: c_int, N: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       B: *const c_void, ldb: c_int, beta: *const c_void,
                       C: *mut c_void, ldc: c_int);
    pub fn cblas_zsyrk(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       beta: *const c_void, C: *mut c_void, ldc: c_int);
    pub fn cblas_zsyr2k(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                        Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                        alpha: *const c_void, A: *const c_void, lda: c_int,
                        B: *const c_void, ldb: c_int, beta: *const c_void,
                        C: *mut c_void, ldc: c_int);
    pub fn cblas_ztrmm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE,
                       Diag: CBLAS_DIAG, M: c_int, N: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       B: *mut c_void, ldb: c_int);
    pub fn cblas_ztrsm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, TransA: CBLAS_TRANSPOSE,
                       Diag: CBLAS_DIAG, M: c_int, N: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       B: *mut c_void, ldb: c_int);
}

// Routines with prefixes C and Z only
extern "C" {
    pub fn cblas_chemm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, M: c_int, N: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       B: *const c_void, ldb: c_int, beta: *const c_void,
                       C: *mut c_void, ldc: c_int);
    pub fn cblas_cherk(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                       alpha: c_float, A: *const c_void, lda: c_int,
                       beta: c_float, C: *mut c_void, ldc: c_int);
    pub fn cblas_cher2k(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                        Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                        alpha: *const c_void, A: *const c_void, lda: c_int,
                        B: *const c_void, ldb: c_int, beta: c_float,
                        C: *mut c_void, ldc: c_int);

    pub fn cblas_zhemm(layout: CBLAS_LAYOUT, Side: CBLAS_SIDE,
                       Uplo: CBLAS_UPLO, M: c_int, N: c_int,
                       alpha: *const c_void, A: *const c_void, lda: c_int,
                       B: *const c_void, ldb: c_int, beta: *const c_void,
                       C: *mut c_void, ldc: c_int);
    pub fn cblas_zherk(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                       Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                       alpha: c_double, A: *const c_void, lda: c_int,
                       beta: c_double, C: *mut c_void, ldc: c_int);
    pub fn cblas_zher2k(layout: CBLAS_LAYOUT, Uplo: CBLAS_UPLO,
                        Trans: CBLAS_TRANSPOSE, N: c_int, K: c_int,
                        alpha: *const c_void, A: *const c_void, lda: c_int,
                        B: *const c_void, ldb: c_int, beta: c_double,
                        C: *mut c_void, ldc: c_int);
}

extern "C" {
    pub fn cblas_xerbla(p: c_int, rout: *const c_char, form: *const c_char, ...);
}
