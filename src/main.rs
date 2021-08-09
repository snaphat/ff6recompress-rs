#![feature(trait_alias)]
use std::{fs, io};

mod aplib;
mod error;
mod lzss;
mod result;
mod rom;
mod util;

fn open(path: &str) -> Result<Vec<u8>, io::Error>
{
    let bytes = fs::read(path)?;
    return Ok(bytes);
    //return Some();
    //file.read_to_string(&mut contents)?;
}

fn main()
{
    let mut rom = match open("Final Fantasy III (USA) (Rev 1).sfc")
    {
        | Ok(bytes) => rom::Rom::new(bytes),
        | Err(e) =>
        {
            println!("{}", e);
            return;
        },
    };

    rom.recompress("dd", 0xC4C008);

    println!("Hello, world!");
}
