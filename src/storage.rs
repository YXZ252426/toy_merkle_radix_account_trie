use rlp::{Rlp, RlpStream};

use crate::{Hash, keccak256};

pub type StorageKey = [u8; 32];
pub type StorageValue = Vec<u8>;

pub fn storage_trie_key(slot_key: &StorageKey) -> Hash {
    keccak256(slot_key)
}

pub fn encode_storage_value(value: &[u8]) -> Vec<u8> {
    let mut stream = RlpStream::new();
    stream.append(&value);
    stream.out().to_vec()
}

pub fn decode_storage_value( encoded: &[u8]) -> Option<StorageValue> {
    Rlp::new(encoded).as_val().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_trie_key_is_deterministic() {
        let slot_key = [0x11u8; 32];

        assert_eq!(storage_trie_key(&slot_key), storage_trie_key(&slot_key));
    }

    #[test]
    fn different_storage_keys_hash_differently() {
        let first_key = [0x11u8; 32];
        let second_key = [0x22u8; 32];

        assert_ne!(storage_trie_key(&first_key), storage_trie_key(&second_key));
    }

    #[test]
    fn storage_value_rlp_round_trips() {
        let value = b"slot-value".to_vec();
        let encoded = encode_storage_value(&value);

        assert_eq!(decode_storage_value(&encoded), Some(value));
    }

    #[test]
    fn empty_storage_value_rlp_round_trips() {
        let value = Vec::new();
        let encoded = encode_storage_value(&value);

        assert_eq!(decode_storage_value(&encoded), Some(value));
    }

    #[test]
    fn decode_storage_value_rejects_invalid_rlp() {
        assert_eq!(decode_storage_value(&[0xff]), None);
    }
}
