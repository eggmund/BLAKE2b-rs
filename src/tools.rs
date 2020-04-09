use std::path::Path;
use std::io::{self, BufReader, Read};
use std::fs::File;

#[inline]
pub fn get_checksum(file_path: &Path, hash_length: u8) -> io::Result<[u64; 8]> {
    use super::{
        constants::IV,
        blake_compress,
    };

    let (mut f, f_size) = prepare_file_for_reading(file_path)?;
    println!("{}", f_size);

    let mut h = IV;
    h[0] = h[0] ^ (0x01010000 ^ hash_length as u64);

    let mut t = 0u64;
    let mut block = [0u8; 128];

    while f_size > 128 && t <= f_size-128 {
        f.read(&mut block)?;
        t = t.wrapping_add(128);
        blake_compress(&mut h, &block, t, false);
    }

    // Final block
    f.read(&mut block)?;
    //bytes_fed += 128;
    blake_compress(&mut h, &block, f_size, true);

    Ok(h)
}

#[inline]
fn prepare_file_for_reading(file_path: &Path) -> io::Result<(BufReader<File>, u64)> {
    let read_f = File::open(file_path)?;
    let size = read_f.metadata().unwrap().len();

    Ok((BufReader::new(read_f), size))
}