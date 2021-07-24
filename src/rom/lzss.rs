extern crate apultra;
use std::cell::RefCell;

use self::apultra::DecompressionError;
use self::apultra::Error;

pub fn decompress(input: &[u8]) -> Result<(Vec<u8>, usize), Error>
{
    // Check if the input is long enough to contain length bytes.
    if input.len() < 2
    {
        return Err(DecompressionError());
    }

    // Get length of compressed data.
    let length = input[0] as usize | (input[1] as usize) << 8;

    // Check if length valid.
    if length == 0
    {
        return Err(DecompressionError());
    }

    // Check if length is within bounds if input data.
    if length > input.len()
    {
        return Err(DecompressionError());
    }

    // Get slice of data of the exact length (for OoB handling).
    let mut src = input[2..length + 2].iter();
    let s = RefCell::new(2); // Source index.

    // Smart wrapper for iterator. Returns DecompressionError if iterating past the end of the buffer.
    let mut next = || -> Result<u8, Error> {
        src.next().ok_or(DecompressionError()).map(|val| {
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
                buffer[b] = c;
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
                    let c = buffer[(w + i) & 0x07FF];
                    line.push(c);
                    buffer[b] = c;
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
