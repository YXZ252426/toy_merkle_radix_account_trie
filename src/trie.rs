use std::collections::HashMap;

use rlp::{Rlp, RlpStream};

use crate::crypto::keccak256;
use crate::types::Hash;

// Convert each byte into two 4-bit nibbles so the trie can branch 16 ways.
fn bytes_to_nibbles(bytes: &[u8]) -> Vec<usize> {
    let mut nibbles = Vec::with_capacity(bytes.len() * 2);

    for byte in bytes {
        let high = (byte >> 4) as usize;
        let low = (byte & 0x0f) as usize;

        nibbles.push(high);
        nibbles.push(low);
    }

    nibbles
}

// A radix branch node has one child slot per nibble plus an optional value.
#[derive(Debug, Clone)]
struct BranchNode {
    children: [Option<Hash>; 16],
    value: Option<Vec<u8>>,
}

impl BranchNode {
    fn new() -> Self {
        BranchNode {
            children: [None; 16],
            value: None,
        }
    }

    // RLP layout: 16 child hash fields followed by the value field.
    fn encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(17);

        for child in self.children.iter() {
            match child {
                Some(hash) => {
                    let hash_bytes: &[u8] = hash;
                    stream.append(&hash_bytes);
                }
                None => {
                    stream.append_empty_data();
                }
            }
        }

        match &self.value {
            Some(value) => {
                stream.append(value);
            }
            None => {
                stream.append_empty_data();
            }
        }

        stream.out().to_vec()
    }

    // Decode a branch node and rebuild empty child slots from empty RLP fields.
    fn decode(bytes: &[u8]) -> Option<Self> {
        let rlp = Rlp::new(bytes);

        if rlp.item_count().ok()? != 17 {
            return None;
        }

        let mut node = BranchNode::new();

        for i in 0..16 {
            let data: Vec<u8> = rlp.val_at(i).ok()?;

            if data.is_empty() {
                node.children[i] = None;
            } else {
                if data.len() != 32 {
                    return None;
                }

                let mut hash = [0u8; 32];
                hash.copy_from_slice(&data);

                node.children[i] = Some(hash);
            }
        }

        let value: Vec<u8> = rlp.val_at(16).ok()?;

        if !value.is_empty() {
            node.value = Some(value);
        }

        Some(node)
    }
}

// Content-addressed node storage: hash(encoded_node) -> encoded_node.
type NodeDb = HashMap<Hash, Vec<u8>>;

#[derive(Debug, Clone)]
pub struct MerkleRadixTrie {
    db: NodeDb,
    root: Option<Hash>,
}

impl Default for MerkleRadixTrie {
    fn default() -> Self {
        Self::new()
    }
}

impl MerkleRadixTrie {
    pub fn new() -> Self {
        MerkleRadixTrie {
            db: HashMap::new(),
            root: None,
        }
    }

    // Return the current root hash; an empty trie uses the zero hash sentinel.
    pub fn root_hash(&self) -> Hash {
        self.root.unwrap_or([0u8; 32])
    }

    // Insert by walking the key nibble by nibble and rebuilding hashes upward.
    pub fn insert(&mut self, key: &[u8], value: Vec<u8>) {
        let nibbles = bytes_to_nibbles(key);

        let new_root = self.insert_at(self.root, &nibbles, &value);

        self.root = Some(new_root);
    }

    fn insert_at(&mut self, node_hash: Option<Hash>, nibbles: &[usize], value: &[u8]) -> Hash {
        // Existing nodes are immutable by hash; decode, modify, then store a new hash.
        let mut node = match node_hash {
            Some(hash) => {
                let encoded_hash = self.db.get(&hash).expect("missing node in db");
                BranchNode::decode(encoded_hash).expect("stored node should decode")
            }
            None => BranchNode::new(),
        };

        if nibbles.is_empty() {
            // The full key has been consumed, so the account bytes live here.
            node.value = Some(value.to_vec());
        } else {
            let index = nibbles[0];

            let old_child_hash = node.children[index];

            let new_child_hash = self.insert_at(old_child_hash, &nibbles[1..], value);

            node.children[index] = Some(new_child_hash);
        }

        let encoded_node = node.encode();
        let node_hash = keccak256(&encoded_node);

        // Store the new version of this node under its content hash.
        self.db.insert(node_hash, encoded_node);

        node_hash
    }

    // Look up a value by following the child hash selected by each key nibble.
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let nibbles = bytes_to_nibbles(key);

        let mut current_hash = self.root?;

        for nibble in nibbles {
            let encoded_node = self.db.get(&current_hash)?;

            let node = BranchNode::decode(encoded_node)?;

            current_hash = node.children[nibble]?
        }

        let encoded_node = self.db.get(&current_hash)?;
        let node = BranchNode::decode(encoded_node)?;

        node.value
    }

    // Return every encoded node on the path from root to the key's value node.
    pub fn prove(&self, key: &[u8]) -> Option<Vec<Vec<u8>>> {
        let nibbles = bytes_to_nibbles(key);

        let mut proof = Vec::new();
        let mut current_hash = self.root?;
        for nibble in nibbles {
            let encoded_node = self.db.get(&current_hash)?.clone();
            let node = BranchNode::decode(&encoded_node)?;

            proof.push(encoded_node);
            current_hash = node.children[nibble]?;
        }

        let encoded_node = self.db.get(&current_hash)?.clone();
        proof.push(encoded_node);

        Some(proof)
    }
}

// Verify that a proof connects the given root hash to the expected key/value.
pub fn verify_proof(root: Hash, key: &[u8], expected_value: &[u8], proof: &[Vec<u8>]) -> bool {
    let nibbles = bytes_to_nibbles(key);

    // This toy trie has one branch node per nibble plus the final value node.
    if proof.len() != nibbles.len() + 1 {
        return false;
    }

    let mut expected_hash = root;
    for depth in 0..proof.len() {
        let encoded_node = &proof[depth];
        let actual_hash = keccak256(encoded_node);

        // Each proof node must hash to the parent reference we expected.
        if expected_hash != actual_hash {
            return false;
        }

        let Some(node) = BranchNode::decode(encoded_node) else {
            return false;
        };

        if depth == nibbles.len() {
            return node.value.as_deref() == Some(expected_value);
        }

        let nibble = nibbles[depth];
        match node.children[nibble] {
            None => {
                return false;
            }
            Some(child_hash) => {
                // The next proof item must match this child hash.
                expected_hash = child_hash;
            }
        }
    }
    false
}
