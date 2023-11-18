#[cxx::bridge(namespace = "snappy")]
mod ffi {
    extern "C++" {
        include!("snappy-cxx-rs/snappy/snappy.h");

        unsafe fn Compress(
            input: *const c_char,
            input_length: usize,
            compressed: *mut CxxString,
        ) -> usize;

        unsafe fn Uncompress(
            input: *const c_char,
            input_length: usize,
            uncompressed: *mut CxxString,
        ) -> bool;
    }
}

pub fn compress(input: &[u8]) -> Vec<u8> {
    cxx::let_cxx_string!(output = "");

    unsafe {
        ffi::Compress(
            input.as_ptr() as *const _,
            input.len(),
            output.as_mut().get_unchecked_mut() as *mut _,
        );
    }
    output.as_bytes().to_vec()
}

pub fn uncompress(input: &[u8]) -> Option<Vec<u8>> {
    cxx::let_cxx_string!(output = "");

    unsafe {
        if ffi::Uncompress(
            input.as_ptr() as *const _,
            input.len(),
            output.as_mut().get_unchecked_mut() as *mut _,
        ) {
            Some(output.as_bytes().to_vec())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_simple() {
        let input = b"Hello world!";
        let compressed = compress(input);
        let uncompressed = uncompress(&compressed).unwrap();
        assert_eq!(input, &uncompressed[..]);
    }
}
