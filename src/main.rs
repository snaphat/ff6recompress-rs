extern crate apultra;
use apultra::DecompressionError;
use std::cell::RefCell;
use std::{error::Error, fs, io};
fn conv_addr(addr: usize) -> usize {
    if addr & 0x408000 != 0 {
        addr & 0x3FFFFF
    } else {
        0x0
    }
}

struct Rom {
    rom: Vec<u8>,
}

fn open(path: &str) -> Result<Vec<u8>, io::Error> {
    let bytes = fs::read(path)?;
    return Ok(bytes);
    //return Some();
    //file.read_to_string(&mut contents)?;
}

fn unpack_lzss(input: &[u8]) -> Result<(Vec<u8>, usize), DecompressionError> {

    if input.len() < 2 { return Err(DecompressionError); }

    // Get length of compressed data.
    let length = input[0] as usize | (input[1] as usize) << 8;

    // Check if length valid.
    if length == 0 {
        return Err(DecompressionError);
    };

    // Get slice of data of the exact length (for OoB handling).
    let mut src = input[2..length+2].iter();
    let s = RefCell::new(2); // Source index.

    // Smart wrapper for iterator. Returns DecompressionError if iterating past the end of the buffer.
    let mut next = || -> Result<u8, DecompressionError> {
        src.next().ok_or(DecompressionError).map(|val| {
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
    while *s.borrow() < length {
        // read header
        let mut header = next()?;

        for _pass in 0..8 {
            let mut line: Vec<u8> = Vec::new();

            if header & 1 != 0 {
                // single byte (uncompressed)
                let c = next()?;
                line.push(c);
                buffer[b] = c;
                b = b + 1 & 0x7FF;
            } else {
                // 2-bytes (compressed)
                let mut w = next()? as usize;
                w |= (next()? as usize) << 8;
                let r = (w >> 11) + 3;
                w &= 0x07FF;

                for i in 0..r {
                    let c = buffer[(w + i) & 0x07FF];
                    line.push(c);
                    buffer[b] = c;
                    b = b + 1 & 0x7FF;
                }
            }
            // copy this pass to the destination buffer
            dest.append(&mut line);

            // Break if we're at the end of the compressed data.
            if *s.borrow() == length {
                break;
            };

            header >>= 1;
        }
    }

    return Ok((dest, length)); // (decompressed data, original compressed size)
}

fn aplib_compress(input: &[u8]) -> Result<Vec<u8>, impl Error> {
    let window_size = 1024;
    let dictionary_size = 0;
    let flags = 0;
    let progress = None;
    let stats = None;
    apultra::compress(input, window_size, dictionary_size, flags, progress, stats)
}

fn aplib_decompress(input: &[u8]) -> Result<Vec<u8>, impl Error> {
    let dictionary_size = 0;
    let flags = 0;
    apultra::decompress(input, dictionary_size, flags)
}

impl Rom {
    fn _recompress(self, offset: usize) {
        let offset = conv_addr(offset);
        let (uncompressed, compressed_size) = unpack_lzss(&self.rom[offset..]).unwrap();
        let a = aplib_compress(&uncompressed);
        let a = a.unwrap();
        let b = aplib_decompress(&a);
        let b = b.unwrap();
        let c = aplib_compress(&b);
        let c = c.unwrap();

        println!(
            "{}\n{}\n{}\n{}\n{}\n",
            uncompressed.len(),
            compressed_size,
            a.len(),
            b.len(),
            c.len()
        );
        assert_eq!(a, c);
        //auto [unpacked, old_size] = unpack_lzss(&rom[offset]);
        //auto repacked             = pack_aplib(unpacked, 0x10000);

        //if (repacked.size() > old_size)
        //    std::cout << "warning: " << repacked.size() << ">=" << old_size << std::endl;
        //else {
        //    auto save = old_size - repacked.size();
        //    saved_bytes += save;
        //}

        // Return repacked data (cannot add to rom until we check if it is a duplicate).
        //return make_tuple(move(repacked), old_size); // <u8vec, uint*>
    }
}

fn main() {
    let rom = match open("Final Fantasy III (USA) (Rev 1).sfc") {
        | Ok(bytes) => Rom { rom: bytes },
        | Err(e) => {
            println!("{}", e);
            return;
        }
    };

    rom._recompress(0xC4C008);

    conv_addr(23);
    println!("Hello, world!");
}
