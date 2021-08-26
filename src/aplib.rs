use crate::{error::DecompressionError, result::Result};

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
        return Err(DecompressionError("Invalid size (<2)"));
    }

    let low = unsafe { *input.get_unchecked(0) } as usize;
    let high = unsafe { *input.get_unchecked(1) } as usize;

    if high != 0xFF || low != 0xFF
    {
        return Err(DecompressionError("Invalid header"));
    }
    let dictionary_size = 0;
    let flags = 0;

    let buf = unsafe { input.get_unchecked(2..) };
    Ok(apultra::decompress(buf, dictionary_size, flags)?)
}

#[cfg(test)]
mod tests
{
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
        let input_data: Vec<u8> = vec![0xFF, 0xFF, 0, 173, 1, 86, 192, 0];
        let decompressed = super::decompress(&input_data).unwrap();
        assert_eq!(decompressed.len(), 100);
        assert_eq!(decompressed, [0; 100]);
    }

    #[test]
    fn decompress_size_error()
    {
        let input_data: Vec<u8> = vec![0];
        let err = super::decompress(&input_data).unwrap_err();
        assert_eq!(err.to_string(), "Decompression Error: `Invalid size (<2)`");
    }

    #[test]
    fn decompress_header_error()
    {
        let input_data: Vec<u8> = vec![0, 173, 1, 86, 192, 0];
        let err = super::decompress(&input_data).unwrap_err();
        assert_eq!(err.to_string(), "Decompression Error: `Invalid header`");
    }
}
