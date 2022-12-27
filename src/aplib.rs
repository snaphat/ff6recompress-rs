use crate::{
    error::Error::{AplibDecompressInvalidheaderError, AplibDecompressShortHeaderError},
    result::Result,
};
pub fn compress(input: &[u8]) -> Result<Vec<u8>>
{
    let window_size = 0x10000;
    let dictionary_size = 0;
    let flags = 0;
    let progress = None;
    let stats = None;
    let mut buf = apultra::compress(input, window_size, dictionary_size, flags, progress, stats)?;
    let prefix = vec![0xFF, 0xFF]; // Add 0xFFFF prefix.
    buf.splice(0..0, prefix);
    buf.resize(buf.len(), 0);
    Ok(buf)
}

pub fn decompress(input: &[u8]) -> Result<Vec<u8>>
{
    if input.len() < 2
    {
        return Err(AplibDecompressShortHeaderError());
    }

    let low = unsafe { *input.get_unchecked(0) } as usize;
    let high = unsafe { *input.get_unchecked(1) } as usize;

    if high != 0xFF || low != 0xFF
    {
        return Err(AplibDecompressInvalidheaderError());
    }
    let dictionary_size = 0;
    let flags = 0;

    let buf = unsafe { input.get_unchecked(2..) };
    Ok(apultra::decompress(buf, dictionary_size, flags)?)
}

#[cfg(test)]
mod tests
{
    use std::intrinsics::transmute;

    use super::{AplibDecompressInvalidheaderError, AplibDecompressShortHeaderError};
    #[test]
    fn compress()
    {
        let input_data = vec![0; 100];
        let compressed = super::compress(&input_data).unwrap();
        assert_eq!(compressed.len(), 8);
        assert_eq!(compressed, [0xFF, 0xFF, 0, 173, 1, 86, 192, 0]);
    }

    #[test]
    fn decompress()
    {
        let input_data = vec![0xFF, 0xFF, 0, 173, 1, 86, 192, 0];
        let decompressed = super::decompress(&input_data).unwrap();
        assert_eq!(decompressed.len(), 100);
        assert_eq!(decompressed, [0; 100]);
    }

    #[test]
    fn compress_size_zero_error()
    {
        let input_data = vec![];
        let err = super::compress(&input_data).unwrap_err();
        assert_eq!(err.to_string(), "Aplib Compression Error: Input size of zero");
    }

    #[test]
    fn compress_mem_alloc_error()
    {
        let raw = [0xFF, 0xFF, 0xFF, 0xFF];
        let data: &[u8] = unsafe { transmute(raw) };
        let err = super::compress(&data).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Aplib Compression Error: memory allocation failed because the memory allocator returned an error"
        );
    }

    #[test]
    fn decompress_short_header_error()
    {
        let input_data = vec![0];
        let err = super::decompress(&input_data).unwrap_err();
        assert_eq!(err.to_string(), AplibDecompressShortHeaderError().to_string());
    }

    #[test]
    fn decompress_invalid_header_error()
    {
        let input_data = vec![0, 173, 1, 86, 192, 0];
        let err = super::decompress(&input_data).unwrap_err();
        assert_eq!(err.to_string(), AplibDecompressInvalidheaderError().to_string());
    }

    #[test]
    fn decompress_size_zero_error()
    {
        let input_data = vec![0xFF, 0xFF];
        let err = super::decompress(&input_data).unwrap_err();
        assert_eq!(err.to_string(), "Aplib Decompression Error: Input size of zero");
    }

    #[test]
    fn decompress_mem_alloc_error()
    {
        let input_data = vec![0xFF, 0xFF, 0x11];
        let err = super::decompress(&input_data).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Aplib Decompression Error: memory allocation failed because the memory allocator returned an error"
        );
    }
}
