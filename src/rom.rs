use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    io::{stdout, Write},
    ops::AddAssign,
};

use crate::{aplib, json, lzss, result::Result};

fn conv_addr(addr: usize) -> usize
{
    if addr & 0x408000 != 0 { addr & 0x3FFFFF } else { 0x0 }
}

trait TblPtrTrait
{
    fn splice_ptr(&mut self, r: TblEntry, ptr: usize) -> ();

    fn extract_ptr(&self, r: TblEntry) -> usize;
}

impl TblPtrTrait for [u8]
{
    fn splice_ptr(&mut self, r: TblEntry, ptr: usize) -> ()
    {
        let mut ptr = ptr; // store mutable copy.
        let tbl_entry = &mut self[r.idx..r.idx + r.len];
        for i in 0..r.len
        {
            tbl_entry[i] = ptr as u8; // store in big endian.
            ptr >>= 8; // shift right.
        }
    }

    fn extract_ptr(&self, r: TblEntry) -> usize
    {
        let mut ptr: usize = 0;
        let tbl_entry = &self[r.idx..r.idx + r.len];
        for i in 0..r.len
        {
            ptr += (tbl_entry[i] as usize) << (8 * i);
        }
        ptr
    }
}

#[derive(Copy, Clone)]
struct TblEntry
{
    idx: usize,
    len: usize,
}

impl AddAssign<usize> for TblEntry
{
    fn add_assign(&mut self, other: usize)
    {
        self.idx += self.len * other;
    }
}

pub struct Rom
{
    rom:         Vec<u8>,
    config:      json::Config,
    saved_bytes: usize,
}

impl Rom
{
    pub fn new(bytes: Vec<u8>) -> Rom
    {
        Rom { rom: bytes, saved_bytes: 0, config: json::Config::default() }
    }

    pub fn _recompress(&mut self, offset: usize) -> Result<Vec<u8>>
    {
        let (uncompressed, orig_compressed_size) = lzss::decompress(&self.rom[offset..])?;
        let recompressed = aplib::compress(&uncompressed)?;

        if recompressed.len() > orig_compressed_size
        {
            println!("warning: {} >= {}", recompressed.len(), orig_compressed_size);
        }
        else
        {
            let save = orig_compressed_size - recompressed.len();
            self.saved_bytes += save;
        };

        //let offset_end = offset + orig_compressed_size;
        Ok(recompressed)
    }

    #[rustfmt::skip]
    pub fn recompress(&mut self, json_entry: &str) -> Result<()>
    {
        // Extract data pointer logic.
        let data = self.config.extract(json_entry)?;
        print!(" \x1b[33m-\x1b[36m {}\x1b[33m...\x1b[39m", data.name);
        stdout().flush().unwrap();
        let data_range = match data.table
        {
            | None =>       // single entry.
            {
                let bank_offset = data.range.start;
                let offset      = conv_addr(bank_offset);
                let data        = self._recompress(offset)?;
                let data_len    = data.len();
                let data_entry  = offset..offset + data_len;
                self.rom.splice(data_entry, data);
                bank_offset..bank_offset + data_len
            }
            | Some(tbl) =>  // multiple entries.
            {
                let mut tbl_entry = TblEntry { idx: conv_addr(tbl.range.start), len: tbl.ptr_size };

                // extract init table data pointer for next entry & get next data ptrs.
                let init_dp    = self.rom.extract_ptr(tbl_entry);
                let mut old_dp = init_dp;
                let mut new_dp = init_dp;

                // Lookup table for detecting duplicate entries.
                let mut lookup_tbl = HashMap::new();

                for _ in 0..tbl.arr_len
                {
                    // Compute data offsets.
                    let old_do = conv_addr(tbl.offset + old_dp);
                    let new_do = conv_addr(tbl.offset + new_dp);

                    // ensure old dp is after the initial.
                    let data = match old_dp < init_dp
                    {
                        | true  => vec![0u8; 0],              // invalid pointer.
                        | false => self._recompress(old_do)?, // valid pointer.
                    };

                    // Hash newly compressed data.
                    let mut hash = DefaultHasher::new();
                    data.hash(&mut hash);
                    let hash = hash.finish();

                    // Splice in data (if any).
                    let data_len   = data.len();
                    let data_entry = new_do..new_do + data_len;
                    self.rom.splice(data_entry, data);

                    // Try to insert data into lookup table and get returned dp.
                    let dp = match lookup_tbl.try_insert(hash, new_dp)
                    {
                        | Ok(value) => *value,   // new entry.
                        | Err(kv)   => kv.value, // duplicate entry.
                    };
                    self.rom.splice_ptr(tbl_entry, dp); // splice in data ptr.

                    // extract table pointer for next entry & get next data ptrs.
                    tbl_entry += 1;
                    old_dp     = self.rom.extract_ptr(tbl_entry);
                    new_dp    += data_len;
                }
                tbl.offset + init_dp..tbl.offset + new_dp
            }
        };

        // Insert updated json entry with new data range.
        self.config.update(json_entry, data_range)?;

        println!("{:width$}\x1b[31mdone\x1b[39m", "", width = (55 - data.name.len()));

        Ok(())
    }

    pub fn process(&mut self) -> Result<()>
    {
        let entries = [
            "battleBackgroundGraphics",
            "battleBackgroundLayout",
            "cinematicProgram",
            "creditsGraphics",
            "endingGraphics",
            "floatingIslandCinematic",
            "mapAnimationGraphicsLayer3",
            "mapGraphicsLayer3",
            "mapLayouts",
            "mapOverlayProperties",
            "mapTileProperties",
            "mapTilesets",
            "worldGraphics3",
            "worldLayout3",
            "worldPalette3",
            "titleIntroGraphics",
            "vectorApproachGraphics",
            "vectorApproachLayout",
            "worldCloudsGraphics",
            "worldCloudsLayout",
            "worldGraphics1",
            "worldLayout1",
            "worldOfRuinCinematic",
            "worldGraphics2",
            "worldLayout2",
        ];

        println!("\x1b[33mFile Size (bytes)\x1b[36m: \x1b[32m{}\x1b[39m", self.rom.len());
        println!("\n\x1b[33mRecompressing\x1b[36m:\x1b[39m");

        for entry in entries.iter()
        {
            self.recompress(entry)?;
        }

        println!("\n\x1b[33mTotal savings (bytes)\x1b[36m: \x1b[32m{}\x1b[39m", self.saved_bytes);

        Ok(())
    }
}
