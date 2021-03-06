use std::cell::RefCell;

use crate::{
    error::Error::{
        LZSSDecompressInputError, LZSSDecompressOOBError, LZSSDecompressSizeError,
        LZSSDecompressZeroError,
    },
    result::Result,
};
pub fn decompress(input: &[u8]) -> Result<(Vec<u8>, usize)>
{
    // Check if the input is long enough to contain length bytes.
    if input.len() < 2
    {
        return Err(LZSSDecompressInputError());
    }

    // Get length of compressed data.
    let low = unsafe { *input.get_unchecked(0) } as usize;
    let high = unsafe { *input.get_unchecked(1) as usize } << 8;
    let length = high | low;

    // Check if length valid.
    if length == 0
    {
        return Err(LZSSDecompressZeroError());
    }

    // Check if length is within bounds if input data.
    if length > input.len()
    {
        return Err(LZSSDecompressSizeError(input.len(), length));
    }

    // Get slice of data of the exact length (for OoB handling).
    let mut src = unsafe { input.get_unchecked(2..length) }.iter();
    let s = RefCell::new(2); // Source index.

    // Smart wrapper for iterator. Returns DecompressionError if iterating past the end of the buffer.
    let mut next = || -> Result<u8> {
        src.next().ok_or(LZSSDecompressOOBError(length - 2)).map(|val| {
            *s.borrow_mut() += 1; // Update source index.
            *val // Return the next value.
        })
    };

    // allocate intermediate buffer starting at index 0x07DE (ff6 start).
    let mut buffer: [u8; 0x800] = [0; 0x800];
    let mut b = 0x07DE;

    // Reserve space for decompressed data.
    let mut dest: Vec<u8> = Vec::new();
    dest.reserve_exact(length);

    // Decompress data.
    while *s.borrow() < length
    {
        // read header
        let mut header = next()?;

        for _pass in 0..8
        {
            let mut line: Vec<u8> = Vec::new();

            if header & 1 != 0
            {
                // single byte (uncompressed)
                let c = next()?;
                line.push(c);
                let ptr = unsafe { buffer.get_unchecked_mut(b) };
                *ptr = c;
                b = b + 1 & 0x7FF;
            }
            else
            {
                // 2-bytes (compressed)
                let mut w = next()? as usize;
                w |= (next()? as usize) << 8;
                let r = (w >> 11) + 3;
                w &= 0x07FF;

                for i in 0..r
                {
                    let c = unsafe { *buffer.get_unchecked((w + i) & 0x07FF) };
                    line.push(c);
                    let ptr = unsafe { buffer.get_unchecked_mut(b) };
                    *ptr = c;
                    b = b + 1 & 0x7FF;
                }
            }
            // copy this pass to the destination buffer
            dest.append(&mut line);

            // Break if we're at the end of the compressed data.
            if *s.borrow() == length
            {
                break;
            };

            header >>= 1;
        }
    }

    Ok((dest, length)) // (decompressed data, original compressed size)
}

#[cfg(test)]
mod tests
{
    use super::{
        decompress, LZSSDecompressInputError, LZSSDecompressOOBError, LZSSDecompressSizeError,
        LZSSDecompressZeroError,
    };

    #[test]
    fn decompression()
    {
        let (data, csize) = decompress(&[0x06, 0x00, 0x01, 0x11, 0xDE, 0x37]).unwrap();
        assert_eq!(csize, 6);
        assert_eq!(&data[..], &[0x11; 10]);
    }

    #[test]
    fn decompression_error_data_too_short()
    {
        let err = decompress(&[0]).unwrap_err();
        assert_eq!(err.to_string(), LZSSDecompressInputError().to_string());
    }

    #[test]
    fn decompression_error_length_zero()
    {
        let err = decompress(&[0, 0]).unwrap_err();
        assert_eq!(err.to_string(), LZSSDecompressZeroError().to_string());
    }

    #[test]
    fn decompression_error_length_less_than_decoded_size()
    {
        let err = decompress(&[5, 0, 1]).unwrap_err();
        assert_eq!(err.to_string(), LZSSDecompressSizeError(3, 5).to_string());
    }

    #[test]
    fn decompression_error_data_oob()
    {
        let err = decompress(&[3, 0, 1, 1]).unwrap_err();
        assert_eq!(err.to_string(), LZSSDecompressOOBError(1).to_string());
    }
}
