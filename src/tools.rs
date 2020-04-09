use std::path::Path;
use std::io::{self, BufReader, Read};
use std::fs::File;

/// Calculates the checksum of a file and returns the checksum in bytes form, and in a little-endian representation in hex.
#[inline]
pub fn get_checksum(file_path: &Path, hash_length: u8) -> io::Result<([u64; 8], String)> {
    use super::{
        constants::IV,
        blake_compress,
    };

    let (mut f, f_size) = prepare_file_for_reading(file_path)?;

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

    let little_endian_repr = get_little_endian_string(&h);

    Ok((h, little_endian_repr))
}

#[inline]
fn prepare_file_for_reading(file_path: &Path) -> io::Result<(BufReader<File>, u64)> {
    let read_f = File::open(file_path)?;
    let size = read_f.metadata().unwrap().len();

    Ok((BufReader::new(read_f), size))
}

// Gets final hash in little endian form
fn get_little_endian_string(h: &[u64; 8]) -> String {
    let mut out = String::new();

    for num in h.iter() {
        let eight = get_8(num);
        for sub in eight.iter() {
            out += format!("{:x}", sub).as_str();
        }
        out += " ";
    }

    out
}

// gets 8 bytes from a single u64. Inverse of get_64
#[inline]
fn get_8(inp: &u64) -> [u8; 8] {
    let mut out = [0u8; 8];

    for i in (0usize..64).step_by(8) {
        out[i/8] = (inp >> i) as u8;
    }

    out
}