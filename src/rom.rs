use std::error::Error;

mod aplib;
mod lzss;

fn conv_addr(addr: usize) -> usize {
    if addr & 0x408000 != 0 {
        addr & 0x3FFFFF
    } else {
        0x0
    }
}

pub struct Rom {
    rom: Vec<u8>,
    saved_bytes: usize,
}

impl Rom {
    pub fn new(bytes: Vec<u8>) -> Rom {
        Rom { rom: bytes, saved_bytes: 0 }
    }
    pub fn _recompress(mut self, offset: usize) -> Result<(Vec<u8>, usize), Box<dyn Error>> {
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
    }
}
