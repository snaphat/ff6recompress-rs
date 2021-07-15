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

fn unpack_lzss(input: &[u8]) -> (Vec<u8>, usize) {
    // Original decode:
    let src = input;
    let mut dest: Vec<u8> = Vec::new();
    let mut s: usize = 0; // source pointer
    let mut buffer: Vec<u8> = vec![0; 0x800];
    let mut b = 0x07DE;
    let mut line: Vec<u8> = vec![0; 34];
    let mut length: usize = src[s] as usize;
    s += 1;
    length |= (src[s] as usize) << 8;
    s += 1;
    if length == 0 {
        return (dest, 0);
    }

    while s < length {
        // read header
        let mut header = src[s];
        s += 1;

        for _pass in 0..8 {
            let mut l = 0;
            if header & 1 != 0 {
                // single byte (uncompressed)
                let c = src[s];
                s += 1;
                line[l] = c;
                l += 1;
                buffer[b] = c;
                b += 1;
                b &= 0x07FF;
            } else {
                // 2-bytes (compressed)
                let mut w: usize = src[s] as usize;
                s += 1;
                w |= (src[s] as usize) << 8;
                s += 1;
                let r = (w >> 11) + 3;
                w &= 0x07FF;

                for i in 0..r {
                    let c = buffer[(w + i) & 0x07FF];
                    line[l] = c;
                    l += 1;
                    buffer[b] = c;
                    b += 1;
                    b &= 0x07FF;
                }
            }
            // copy this pass to the destination buffer
            dest.extend_from_slice(&line[..l]);

            // reached end of compressed data
            if s >= length {
                break;
            }
            header >>= 1;
        }
    }

    return (dest, s); // (decompressed data, original compressed size)
}

fn aplib_compress(input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let window_size = 1024;
    let dictionary_size = 0;
    let flags = 0;
    let progress = None;
    let stats = None;
    apultra::compress(input, window_size, dictionary_size, flags, progress, stats)
}

fn aplib_decompress(input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let dictionary_size = 0;
    let flags = 0;
    apultra::decompress(input, dictionary_size, flags)
}

impl Rom {
    fn _recompress(self, offset: usize) {
        let offset = conv_addr(offset);
        let (uncompressed, compressed_size) = unpack_lzss(&self.rom[offset..]);
        let a = aplib_compress(&uncompressed);
        let a = a.unwrap();
        let b = aplib_decompress(&a);
        let b = b.unwrap();
        let c = aplib_compress(&b);
        let c = c.unwrap();

        println!("{}\n{}\n{}\n{}\n", compressed_size, a.len(), b.len(), c.len());
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
