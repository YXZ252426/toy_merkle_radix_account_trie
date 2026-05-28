use sha3::{Digest, Keccak256};

use crate::types::Hash;

// Hash arbitrary bytes with Keccak-256, the hash function used by Ethereum.
pub fn keccak256(data: &[u8]) -> Hash {
    let digest = Keccak256::digest(data);

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&digest);

    hash
}
