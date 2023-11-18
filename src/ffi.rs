use std::ffi::c_char;
use std::ffi::c_uint;

extern "C" {
    pub(crate) fn snappy_compress(
        input: *const c_char,
        input_length: usize,
        compressed: *mut c_char,
        compressed_length: *mut usize,
    ) -> c_uint;

    pub(crate) fn snappy_uncompress(
        compressed: *const c_char,
        compressed_length: usize,
        uncompressed: *mut c_char,
        uncompressed_length: *mut usize,
    ) -> c_uint;

    pub(crate) fn snappy_max_compressed_length(source_length: usize) -> usize;

    pub(crate) fn snappy_uncompressed_length(
        compressed: *const c_char,
        compressed_length: usize,
        result: *mut usize,
    ) -> c_uint;

    pub(crate) fn snappy_validate_compressed_buffer(
        compressed: *const c_char,
        compressed_length: usize,
    ) -> c_uint;
}
