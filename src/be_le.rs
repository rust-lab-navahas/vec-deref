use sha3::{Digest, Keccak256};
use std::fmt::Write;

fn to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

fn to_bin(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 8);
    for b in bytes {
        write!(&mut s, "{:08b}", b).unwrap();
    }
    s
}

fn slice_vec<T: AsRef<[u8]>>(hash: T) {
    let bytes: &[u8] = hash.as_ref();
    let h_slice = to_bin(bytes);

    let owned: Vec<u8> = bytes.to_vec();
    let h_vec = to_bin(&owned);

    assert_eq!(h_slice, h_vec);
}

fn le_be(hash: &[u8]) {
    let hmap_le = u64::from_le_bytes(hash.try_into().unwrap());
    let hmap_be = u64::from_be_bytes(hash.try_into().unwrap());
    assert_ne!(hmap_le, hmap_be);

    // little-endian manual assembly
    let mut h_le: u64 = 0;
    for (i, &b) in hash.iter().enumerate() {
        h_le |= (b as u64) << (8 * i);
    }

    // big-endian manual assembly
    let mut h_be: u64 = 0;
    for &b in hash {
        h_be = (h_be << 8) | (b as u64);
    }

    assert_eq!(hmap_be, h_be);
    assert_eq!(hmap_le, h_le);
    println!("\nFull u64 binary:");
    println!("LE u64: {:064b}", hmap_le);
    println!("BE u64: {:064b}", hmap_be);
}

fn main() {
    let mut hasher = Keccak256::new();
    hasher.update("hello");
    hasher.update("world!");
    let hash = hasher.finalize();
    let h_string = to_hex(&hash);
    println!("Hex string: {}", h_string);

    let first8 = &hash[0..8];

    le_be(first8);
    slice_vec(&hash);
}
