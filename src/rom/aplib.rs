use std::error::Error;

pub fn compress(input: &[u8]) -> Result<Vec<u8>, impl Error>
{
    let window_size = 1024;
    let dictionary_size = 0;
    let flags = 0;
    let progress = None;
    let stats = None;
    apultra::compress(input, window_size, dictionary_size, flags, progress, stats)
}

pub fn decompress(input: &[u8]) -> Result<Vec<u8>, impl Error>
{
    let dictionary_size = 0;
    let flags = 0;
    apultra::decompress(input, dictionary_size, flags)
}
