use libc::c_int;

#[allow(bad_style)]
pub type CBLAS_INDEX = c_int;

#[allow(bad_style)]
#[repr(C)]
pub enum CBLAS_LAYOUT {
    CblasRowMajor = 101,
    CblasColMajor = 102,
}
pub use self::CBLAS_LAYOUT::*;

#[allow(bad_style)]
#[repr(C)]
pub enum CBLAS_TRANSPOSE {
    CblasNoTrans = 111,
    CblasTrans = 112,
    CblasConjTrans = 113,
}
pub use self::CBLAS_TRANSPOSE::*;

#[allow(bad_style)]
#[repr(C)]
pub enum CBLAS_UPLO {
    CblasUpper = 121,
    CblasLower = 122,
}
pub use self::CBLAS_UPLO::*;

#[allow(bad_style)]
#[repr(C)]
pub enum CBLAS_DIAG {
    CblasNonUnit = 131,
    CblasUnit = 132,
}
pub use self::CBLAS_DIAG::*;

#[allow(bad_style)]
#[repr(C)]
pub enum CBLAS_SIDE {
    CblasLeft = 141,
    CblasRight = 142,
}
pub use self::CBLAS_SIDE::*;

#[allow(bad_style)]
pub type CBLAS_ORDER = CBLAS_LAYOUT;
