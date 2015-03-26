extern crate libblas_sys as raw;

#[test]
fn linking() {
    let m = 1;
    let n = 1;
    let alpha = 0.0;
    let a = vec![0.0];
    let lda = 1;
    let x = vec![0.0];
    let incx = 1;
    let beta = 0.0;
    let mut y = vec![0.0];
    let incy = 1;

    unsafe {
        raw::cblas_dgemv(raw::CblasRowMajor, raw::CblasNoTrans, m, n, alpha, a.as_ptr(), lda, x.as_ptr(),
                         incx, beta, y.as_mut_ptr(), incy);
    }
}
