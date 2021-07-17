use std::{error::Error, fs, io};
mod aplib;
mod lzss;

fn conv_addr(addr: usize) -> usize {
    if addr & 0x408000 != 0 {
        addr & 0x3FFFFF
    } else {
        0x0
    }
}

struct Rom {
    rom: Vec<u8>,
    saved_bytes: usize,
}

fn open(path: &str) -> Result<Vec<u8>, io::Error> {
    let bytes = fs::read(path)?;
    return Ok(bytes);
    //return Some();
    //file.read_to_string(&mut contents)?;
}

impl Rom {
    fn _recompress(mut self, offset: usize) -> Result<(Vec<u8>, usize), Box<dyn Error>> {
        let offset = conv_addr(offset);
        let (uncompressed, orig_compressed_size) = lzss::decompress(&self.rom[offset..])?;
        let recompressed = aplib::compress(&uncompressed)?;

        if recompressed.len() > orig_compressed_size {
            println!("warning: {} >= {}", recompressed.len(), orig_compressed_size);
        } else {
            let save = orig_compressed_size - recompressed.len();
            self.saved_bytes += save;
        };

        Ok((recompressed, orig_compressed_size))
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
        | Ok(bytes) => Rom { rom: bytes, saved_bytes: 0 },
        | Err(e) => {
            println!("{}", e);
            return;
        }
    };

    rom._recompress(0xC4C008);

    println!("Hello, world!");
}
