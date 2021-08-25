#![feature(trait_alias)]
#![feature(map_try_insert)]
use std::{fs, io};

mod aplib;
mod error;
mod get_checked;
mod hash;
mod hex;
mod json;
mod lzss;
mod result;
mod rom;

fn open(path: &str) -> Result<Vec<u8>, io::Error>
{
    let bytes = fs::read(path)?;
    return Ok(bytes);
    //return Some();
    //file.read_to_string(&mut contents)?;
}

// FIXME: Add checksum
fn main()
{
    let func = || -> Result<(), error::Error> {
        let bytes = open("Final Fantasy III (USA) (Rev 1).sfc")?;
        let mut rom = rom::Rom::new(bytes);
        rom.process()?;
        rom.save("test")?;
        Ok(())
    };

    match func()
    {
        | Err(e) => println!("{}", e),
        | _ => (),
    };
}
