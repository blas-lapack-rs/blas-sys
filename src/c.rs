use libc::c_int;

#[allow(bad_style)]
pub type CBLAS_INDEX = c_int;

#[allow(bad_style)]
#[repr(C)]
pub enum CBLAS_LAYOUT {
    CblasRowMajor = 101,
    CblasColMajor = 102,
}

#[allow(bad_style)]
#[repr(C)]
pub enum CBLAS_TRANSPOSE {
    CblasNoTrans = 111,
    CblasTrans = 112,
    CblasConjTrans = 113,
}

#[allow(bad_style)]
#[repr(C)]
pub enum CBLAS_UPLO {
    CblasUpper = 121,
    CblasLower = 122,
}

#[allow(bad_style)]
#[repr(C)]
pub enum CBLAS_DIAG {
    CblasNonUnit = 131,
    CblasUnit = 132,
}

#[allow(bad_style)]
#[repr(C)]
pub enum CBLAS_SIDE {
    CblasLeft = 141,
    CblasRight = 142,
}

#[allow(bad_style)]
pub type CBLAS_ORDER = CBLAS_LAYOUT;
