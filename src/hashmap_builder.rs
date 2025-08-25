use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

// Why the wrapper?
// Rust's orphan rule (coherence): you can only implement a trait if you own
// either the trait or the type. Since `Hasher` (std) and `Keccak256` (sha3)
// are both external, we define a local newtype `Keccak256Hasher` and
// implement `Hasher` for it.
pub struct Keccak256Hasher(Keccak256);

impl Keccak256Hasher {
    pub fn new() -> Self {
        Self(Keccak256::new())
    }
}

impl Hasher for Keccak256Hasher {
    fn finish(&self) -> u64 {
        let hash = self.0.clone().finalize();
        u64::from_be_bytes([
            hash[0], hash[1], hash[2], hash[3],
            hash[4], hash[5], hash[6], hash[7],
        ])
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }
}

#[derive(Clone, Copy, Default)]
pub struct Keccak256Builder;

// Why BuildHasher?
// `HashMap` needs a factory (`BuildHasher`) to create a fresh hasher state
// whenever it hashes keys. Our zero-sized `Keccak256Builder` provides that.
//
// Notes:
// - `Hasher::finish` returns a `u64`, so we truncate Keccak-256 to 64 bits. 
//   This increases collision risk and does not make `HashMap` cryptographically
//   secure. Use this only if you specifically need deterministic Keccak-based
//   hashing behavior, not for security.
// - Keccak-256 is much slower than typical map hashers so a performance hit 
//   is expected.
impl BuildHasher for Keccak256Builder {
    type Hasher = Keccak256Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        Keccak256Hasher::new()
    }
}

fn main() {
    let mut map = HashMap::with_hasher(Keccak256Builder::default());
    println!("Empty map: {:?}", map);
    
    map.insert("hello?".to_string(), "Hello!".to_string());
    map.insert("world".to_string(), "World!".to_string());
    
    println!("Map with data: {:?}", map);
    println!("Getting 'hello?': {:?}", map.get("hello?"));
    println!("Getting 'world': {:?}", map.get("world"));
}
