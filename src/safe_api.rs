use super::ffi::*;
use std::ffi::c_char;
use std::ffi::c_uint;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnappyError {
    #[error("invalid input")]
    InvalidInput,
    #[error("buffer too small")]
    BufferTooSmall,
    #[error("unknown error code: {0}")]
    Unknown(c_uint),
}

fn snappy_to_result(status: c_uint) -> Result<(), SnappyError> {
    match status {
        0 => Ok(()),
        1 => Err(SnappyError::InvalidInput),
        2 => Err(SnappyError::BufferTooSmall),
        _ => Err(SnappyError::Unknown(status)),
    }
}

pub fn compress(input: &[u8]) -> Result<Vec<u8>, SnappyError> {
    if input.len() >= u32::MAX as usize {
        // despite using a usize for the length, snappy only supports u32
        return Err(SnappyError::InvalidInput);
    }

    // SAFETY: snappy_max_compressed_length does only arithmetic and is safe to call
    let max_compressed_len = unsafe { snappy_max_compressed_length(input.len()) };

    if max_compressed_len >= u32::MAX as usize {
        // compressed data would not fit into the maximum buffer size
        return Err(SnappyError::BufferTooSmall);
    }

    let mut output = Vec::with_capacity(max_compressed_len);
    let mut output_length = max_compressed_len;
    // SAFETY: snappy_compress is safe to call with a valid input and output buffer.
    // The output buffer is guaranteed to be large enough.
    unsafe {
        let res = snappy_compress(
            input.as_ptr() as *const c_char,
            input.len(),
            output.as_mut_ptr() as *mut c_char,
            &mut output_length,
        );
        snappy_to_result(res)?;
    }

    assert!(output_length <= max_compressed_len);
    // SAFETY: the output buffer is guaranteed to be large enough
    // SAFETY: snappy_compress has filled the output buffer with valid data up to `output_length`
    unsafe {
        output.set_len(output_length);
    }

    Ok(output)
}

pub fn uncompress(input: &[u8]) -> Result<Vec<u8>, SnappyError> {
    if input.len() >= u32::MAX as usize {
        // no valid snappy-compressed data can be larger than u32::MAX
        return Err(SnappyError::InvalidInput);
    }

    unsafe {
        let res = snappy_validate_compressed_buffer(input.as_ptr() as *const c_char, input.len());
        snappy_to_result(res)?;
    }

    let output_length = {
        let mut output_length = 0;

        // SAFETY: snappy_uncompressed_length is safe to call with a valid input buffer.
        unsafe {
            let res = snappy_uncompressed_length(
                input.as_ptr() as *const c_char,
                input.len(),
                &mut output_length,
            );
            snappy_to_result(res)?;
        }

        output_length
    };

    let mut output = Vec::with_capacity(output_length);
    let mut actual_output_length = output_length;
    unsafe {
        let res = snappy_uncompress(
            input.as_ptr() as *const c_char,
            input.len(),
            output.as_mut_ptr() as *mut c_char,
            &mut actual_output_length,
        );
        snappy_to_result(res)?;
    };

    assert!(actual_output_length <= output_length);
    // SAFETY: the output buffer is guaranteed to be large enough
    // SAFETY: snappy_compress has filled the output buffer with valid data up to `actual_output_length`
    unsafe {
        output.set_len(actual_output_length);
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_and_uncompress() {
        let input = b"Hello, world!";
        let compressed = compress(input).unwrap();
        let uncompressed = uncompress(&compressed).unwrap();
        assert_eq!(input, uncompressed.as_slice());
    }

    #[test]
    fn compress_invalid_input() {
        let input = vec![0; u32::MAX as usize + 1];
        assert!(compress(&input).is_err());
    }

    #[test]
    fn uncompress_invalid_input() {
        let input = vec![0; u32::MAX as usize + 1];
        assert!(uncompress(&input).is_err());
    }
}
