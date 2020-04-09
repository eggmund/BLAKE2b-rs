/// BLAKE2b implementation in Rust.
/// No hash salting (yet), just a small project.

// Word size: 8 bytes = u64
// Vector size: 16 words = 16 u64s
// Block size: 128 bytes = 16 u64s

// Rotation constants:
// R1 = 32, R2 = 24, R3 = 16, R4 = 63

pub mod tools;

mod constants {
    /// Initialization vector -> Initial state of hash.
    pub const IV: [u64; 8] = [
        0x6A09E667F3BCC908, 0xBB67AE8584CAA73B,
        0x3C6EF372FE94F82B, 0xA54FF53A5F1D36F1,
        0x510E527FADE682D1, 0x9B05688C2B3E6C1F,
        0x1F83D9ABFB41BD6B, 0x5BE0CD19137E2179
    ];

    /// Round constants
    pub const SIGMA: [[usize; 16]; 12] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
        [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
        [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
        [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
        [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
        [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
        [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
        [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
        [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], // Same as first line
        [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3]  // Same as second line
    ];
}

/// Mixing function.
#[inline]
pub fn mix(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: &u64, y: &u64) {
    v[a] = v[a].wrapping_add(v[b].wrapping_add(*x));  // v[a] += v[b] + x
    v[d] = (v[d] ^ v[a]).rotate_right(32);

    v[c] = v[c].wrapping_add(v[d]);     // v[c] += v[d]
    v[b] = (v[b] ^ v[c]).rotate_right(24);

    v[a] = v[a].wrapping_add(v[b].wrapping_add(*y));   // v[a] += v[b] + y
    v[d] = (v[d] ^ v[a]).rotate_right(16);

    v[c] = v[c].wrapping_add(v[d]);   // v[c] += v[d]
    v[b] = (v[b] ^ v[c]).rotate_right(63);
}

/// Compression function
#[inline]
pub fn blake_compress(h: &mut [u64; 8], block: &[u8; 128], t: u64, last_block: bool) {
    use constants::{IV, SIGMA};
    let mut v: [u64; 16] = [
        h[0], h[1], h[2], h[3],
        h[4], h[5], h[6], h[7],
        IV[0], IV[1], IV[2], IV[3],
        IV[4] ^ t, IV[5], IV[6], IV[7]
    ];

    if last_block {
        v[14] = !v[14];
    }

    // Message block
    let mut m = [0u64; 16];
    for i in (0..128).step_by(8) {
        m[i/8] = get_64(&block[i..i+8]);
    }

    for i in 0usize..12 {
        mix(&mut v, 0, 4,  8, 12, &m[SIGMA[i][0]], &m[SIGMA[i][1]]);
        mix(&mut v, 1, 5,  9, 13, &m[SIGMA[i][2]], &m[SIGMA[i][3]]);
        mix(&mut v, 2, 6, 10, 14, &m[SIGMA[i][4]], &m[SIGMA[i][5]]);
        mix(&mut v, 3, 7, 11, 15, &m[SIGMA[i][6]], &m[SIGMA[i][7]]);

        mix(&mut v, 0, 5, 10, 15, &m[SIGMA[i][ 8]], &m[SIGMA[i][ 9]]);   // Rows have been shifted
        mix(&mut v, 1, 6, 11, 12, &m[SIGMA[i][10]], &m[SIGMA[i][11]]);
        mix(&mut v, 2, 7,  8, 13, &m[SIGMA[i][12]], &m[SIGMA[i][13]]);
        mix(&mut v, 3, 4,  9, 14, &m[SIGMA[i][14]], &m[SIGMA[i][15]]);
    }

    // Add this hash to final hash
    for i in 0..8 {
        h[i] ^= v[i] ^ v[i + 8];
    }
}

// Gets a single 64 bit word from list of 8 bytes
#[inline]
fn get_64(inp: &[u8]) -> u64 {
    inp[0] as u64 ^
    ((inp[1] as u64) << 8) ^
    ((inp[2] as u64) << 16) ^
    ((inp[3] as u64) << 24) ^
    ((inp[4] as u64) << 32) ^
    ((inp[5] as u64) << 40) ^
    ((inp[6] as u64) << 48) ^
    ((inp[7] as u64) << 56)
}


#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
}
