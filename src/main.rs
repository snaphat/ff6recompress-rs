use apultra;
use std::{fs, io};

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

impl Rom {
    fn _recompress(self, offset: usize) {
        let offset = conv_addr(offset);
        let a = apultra::compress(&self.rom[offset..], 0, 0, 0, None, None);
        let a = a.unwrap();
        let b = apultra::decompress(&a, 0, 0);
        let b = b.unwrap();
        let c = apultra::compress(&b, 0, 0, 0, None, None);
        let c = c.unwrap();

        println!("{}\n{}\n{}", a.len(), b.len(), c.len());
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
