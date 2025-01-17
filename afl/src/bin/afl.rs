#[macro_use]
extern crate afl;

fn check_basic_simd_res(
    std_res: Result<&str, std::str::Utf8Error>,
    simd_res: Result<(), simdutf8::basic::Utf8Error>,
) {
    assert_eq!(std_res.is_ok(), simd_res.is_ok());
}

fn check_compat_simd_res(
    std_res: Result<&str, std::str::Utf8Error>,
    simd_res: Result<(), simdutf8::compat::Utf8Error>,
) {
    match (simd_res, std_res) {
        (Ok(_), Ok(_)) => {}
        (Ok(_), Err(_)) => {
            panic!("simd: Ok, std: Err")
        }
        (Err(_), Ok(_)) => {
            panic!("simd: Err, std: Ok")
        }
        (Err(simd_err), Err(std_err)) => {
            assert_eq!(simd_err.valid_up_to(), std_err.valid_up_to());
            assert_eq!(simd_err.error_len(), std_err.error_len());
        }
    }
}

fn main() {
    fuzz!(|data: &[u8]| {
        unsafe {
            let std_res = std::str::from_utf8(data);
            let basic_simd_res = simdutf8::basic::imp::x86::avx2::validate_utf8(data);
            let compat_simd_res = simdutf8::compat::imp::x86::avx2::validate_utf8(data);
            check_basic_simd_res(std_res, basic_simd_res);
            check_compat_simd_res(std_res, compat_simd_res);
            let basic_simd_res = simdutf8::basic::imp::x86::sse42::validate_utf8(data);
            let compat_simd_res = simdutf8::compat::imp::x86::sse42::validate_utf8(data);
            check_basic_simd_res(std_res, basic_simd_res);
            check_compat_simd_res(std_res, compat_simd_res);
        }
    });
}
