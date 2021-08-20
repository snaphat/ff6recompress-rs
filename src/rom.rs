use crate::{aplib, json, lzss, result::Result};

fn conv_addr(addr: usize) -> usize
{
    if addr & 0x408000 != 0 { addr & 0x3FFFFF } else { 0x0 }
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

    pub fn _recompress(&mut self, offset: usize) -> Result<(Vec<u8>, usize)>
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

    pub fn recompress(&mut self, entry: &str) -> Result<()>
    {
        // Extract data pointer logic.
        let data = self.config.extract(entry)?;
        let bank_offset = data.range.start;
        let offset = conv_addr(bank_offset);

        match data.table
        {
            | None =>
            {
                let (data, _) = self._recompress(offset)?;
                self.config.insert(entry, bank_offset..bank_offset + data.len())?;
            },
            | Some(table) =>
            {
                println!("{}", data.name);
                let extract_ptr = |s: &[u8]| -> usize {
                    let mut t = s[0] as usize;
                    for i in 1..table.ptr_size
                    {
                        t += (s[i] as usize) << (8 * i);
                    }
                    t
                };

                let mut tbl_entry = table.range.start;
                //let mut data_loc = ;
                for i in 0..table.arr_len
                {
                    let data_ptr = extract_ptr(&self.rom[conv_addr(tbl_entry)..]);
                    println!("{}", data_ptr);
                    let (data, _) = self._recompress(conv_addr(table.offset+data_ptr))?;
                    tbl_entry += table.ptr_size;
                }
            },
        };

        Ok(())

        // if let Some(data) = data {

        // };
        // println!("{}", a.name);

        // self.config.insert(entry, 5..10).unwrap();
        // let offset = conv_addr(bank_offset); // FIXME: ?
        // let (data, offset_end) = self._recompress(offset).unwrap();
        // // json.add_freespace(bank_offset+data.size(), bank_offset+old_size);
        // // json.insert(json_entry, bank_offset, bank_offset+data.size());
        // self.rom.splice(offset..offset_end, data);
        // //copy(data.begin(), data.end(), &rom[offset]);
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

        for entry in entries.iter()
        {
            self.recompress(entry)?;
        }
        Ok(())
    }
}
