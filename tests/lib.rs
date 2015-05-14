extern crate libblas_sys as raw;

#[test]
fn link() {
    let trans = b'N';
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
        raw::dgemv_(&trans as *const _ as *const _, &m as *const _, &n as *const _,
                    &alpha as *const _, a.as_ptr(), &lda as *const _, x.as_ptr(),
                    &incx as *const _, &beta as *const _, y.as_mut_ptr(), &incy as *const _);
    }
}
