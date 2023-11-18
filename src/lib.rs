use std::ffi::c_char;

pub use ffi::compress_source_to_sink;
pub use ffi::uncompress_source_to_sink;

pub struct RustSliceSource<'a> {
    data: &'a [u8],
}

impl<'a> RustSliceSource<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub(crate) fn available(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn peek(&mut self, len: &mut usize) -> *const c_char {
        *len = self.data.len();
        self.data.as_ptr() as *const _
    }

    pub(crate) fn skip(&mut self, n: usize) {
        self.data = &self.data[n..];
    }
}

pub struct RustVecSink {
    data: Vec<u8>,
}

impl RustVecSink {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.data
    }

    pub(crate) unsafe fn append(&mut self, bytes: *const c_char, len: usize) {
        let data = unsafe { std::slice::from_raw_parts(bytes as *const _, len) };
        self.data.extend_from_slice(data);
    }
}

#[cxx::bridge(namespace = "snappy")]
mod ffi {
    extern "Rust" {
        type RustSliceSource<'a>;

        fn available(&self) -> usize;
        fn peek(&mut self, len: &mut usize) -> *const c_char;
        fn skip(&mut self, n: usize);
    }

    extern "Rust" {
        type RustVecSink;

        unsafe fn append(&mut self, bytes: *const c_char, len: usize);
    }

    unsafe extern "C++" {
        include!("snappy-cxx-rs/src/adapter.h");

        fn compress_source_to_sink<'a>(
            uncompressed: Pin<&mut RustSliceSource<'a>>,
            compressed: Pin<&mut RustVecSink>,
        ) -> usize;

        fn uncompress_source_to_sink<'a>(
            compressed: Pin<&mut RustSliceSource<'a>>,
            uncompressed: Pin<&mut RustVecSink>,
        ) -> bool;
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use super::*;

    #[test]
    fn test_roundtrip_source_sink() {
        let input_data = b"Hello world!";

        let mut input = RustSliceSource::new(&input_data[..]);
        let mut output = RustVecSink::new();
        compress_source_to_sink(Pin::new(&mut input), Pin::new(&mut output));

        let compressed = output.into_inner();
        assert_ne!(input_data, compressed.as_slice());

        let mut input = RustSliceSource::new(&compressed[..]);
        let mut output = RustVecSink::new();
        uncompress_source_to_sink(Pin::new(&mut input), Pin::new(&mut output));

        let uncompressed = output.into_inner();
        assert_eq!(input_data, uncompressed.as_slice());
    }
}
