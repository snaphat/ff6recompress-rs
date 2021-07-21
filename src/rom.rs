use std::error::Error;

mod aplib;
mod error;
mod hex_to;
mod json;
mod lzss;

fn conv_addr(addr: usize) -> usize
{
    if addr & 0x408000 != 0 { addr & 0x3FFFFF } else { 0x0 }
}

pub struct Rom
{
    rom:         Vec<u8>,
    saved_bytes: usize,
}

impl Rom
{
    pub fn new(bytes: Vec<u8>) -> Rom
    {
        Rom { rom: bytes, saved_bytes: 0 }
    }

    pub fn _recompress(&mut self, offset: usize) -> Result<(Vec<u8>, usize), Box<dyn Error>>
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

        let offset_end = offset + orig_compressed_size;
        Ok((recompressed, offset_end))
    }

    pub fn recompress(&mut self, json_entry: &str, bank_offset: usize) -> ()
    {
        let config = json::Config::new();
        config.extract("cinematicProgram").unwrap();
        let offset = conv_addr(bank_offset); // FIXME: ?
        let (data, offset_end) = self._recompress(offset).unwrap();
        // json.add_freespace(bank_offset+data.size(), bank_offset+old_size);
        // json.insert(json_entry, bank_offset, bank_offset+data.size());
        self.rom.splice(offset..offset_end, data);
        //copy(data.begin(), data.end(), &rom[offset]);
    }
}
