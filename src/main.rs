use std::collections::HashMap;

use sha3::{Digest, Keccak256};
use rlp::{Rlp,RlpStream}; 


type Hash = [u8; 32];
type Address = [u8; 20];

// keccak256(data)
fn keccak256(data: &[u8]) -> Hash {
    let digest = Keccak256::digest(data);

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&digest);

    hash
}

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
#[derive(Debug, Clone, PartialEq, Eq)]
struct Account {
    nonce: u64,
    balance: u64,
    storage_root: Hash,
    code_hash: Hash,
}

impl Account {
    fn new_eoa(nonce: u64, balance: u64) -> Self {
        Self { 
            nonce, 
            balance, 
            storage_root: [0; 32], 
            code_hash: [0; 32], 
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(4);

        stream.append(&self.nonce);
        stream.append(&self.balance);
        stream.append(&self.storage_root.to_vec());
        stream.append(&self.code_hash.to_vec());

        stream.out().to_vec()
    }

    fn decode(bytes: &[u8]) -> Self {
        let rlp = Rlp::new(bytes);

        let nonce: u64 = rlp.val_at(0).expect("invalid nonce");
        let balance: u64 = rlp.val_at(1).expect("invalid balance");

        let storage_root_vec: Vec<u8> = rlp.val_at(2).expect("invalid storage root");
        let code_hash_vec: Vec<u8> = rlp.val_at(3).expect("invalid code hash");

        assert_eq!(storage_root_vec.len(), 32);
        assert_eq!(code_hash_vec.len(), 32);

        let mut storage_root = [0u8; 32];
        storage_root.copy_from_slice(&storage_root_vec);

        let mut code_hash = [0u8; 32];
        code_hash.copy_from_slice(&code_hash_vec);

        Self { 
            nonce, 
            balance, 
            storage_root, 
            code_hash 
        }
    }
}
#[derive(Debug, Clone)]
struct BranchNode {
    children: [Option<Hash>; 16],
    value: Option<Vec<u8>>,
}

impl BranchNode {
    fn new() -> Self {
        BranchNode { 
            children: [None; 16], 
            value: None 
        }
    }

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

    fn decode(bytes: &[u8]) -> Self {
        let rlp = Rlp::new(bytes);

        let item_count = rlp.item_count().expect("invalid branch node rlp");
        assert_eq!(item_count, 17, "branch node must have 17 elements");

        let mut node = BranchNode::new();

        for i in 0..16 {
            let data: Vec<u8> = rlp.val_at(i).expect("invalid child field");

            if data.is_empty() {
                node.children[i] = None;
            } else {
                assert_eq!(data.len(), 32, "child hash must be 32 bytes");

                let mut hash = [0u8; 32];
                hash.copy_from_slice(&data);

                node.children[i] = Some(hash);
            }
        }

        let value: Vec<u8> = rlp.val_at(16).expect("invalid value field");

        if !value.is_empty() {
            node.value = Some(value);
        }

        node
    }
}

type NodeDb = HashMap<Hash, Vec<u8>>;

#[derive(Debug, Clone)]
struct MerkleRadixTrie {
    db: NodeDb,
    root: Option<Hash>,
}

impl MerkleRadixTrie {
    fn new() -> Self {
        MerkleRadixTrie { 
            db: HashMap::new(), 
            root: None, 
        }
    }

    fn root_hash(&self) -> Hash {
        [0; 32]
    }

    fn insert(&mut self, key: &[u8], value: Vec<u8>) {
        let nibbles = bytes_to_nibbles(key);

        let new_root = self.insert_at(self.root, &nibbles, &value);

        self.root = Some(new_root);
    }

    fn insert_at(
        &mut self,
        node_hash: Option<Hash>,
        nibbles: &[usize],
        value: &[u8],
    ) -> Hash {
        let mut node = match node_hash {
            Some(hash) => {
                let encoded_hash = self.db.get(&hash).expect("missing node in db");
                BranchNode::decode(encoded_hash)
            }
            None => {
                BranchNode::new()
            }
        };

        if nibbles.is_empty() {
            node.value = Some(value.to_vec());
        } else {
            let index = nibbles[0];

            let old_child_hash = node.children[index];

            let new_child_hash = self.insert_at(
                old_child_hash, 
                &nibbles[1..], 
                value
            );

            node.children[index] = Some(new_child_hash);
        }

        let encoded_node = node.encode();
        let node_hash = keccak256(&encoded_node);

        self.db.insert(node_hash, encoded_node);

        node_hash
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let nibbles = bytes_to_nibbles(key);
        
        let mut current_hash = self.root?;

        for nibble in nibbles {
            let encoded_node = self.db.get(&current_hash)?;

            let node = BranchNode::decode(encoded_node);

            current_hash = node.children[nibble]?
        }

        let encoded_node = self.db.get(&current_hash)?;
        let node = BranchNode::decode(encoded_node);
        
        node.value
    }

    fn prove(&self, key: &[u8]) -> Option<Vec<Vec<u8>>> {
        None
    }
}

fn verify_proof(
    root: Hash,
    key: &[u8],
    expected_value: &[u8],
    proof: &[Vec<u8>],
) -> bool {
    true
}

#[derive(Debug, Clone)]
struct AccountTrie {
    trie: MerkleRadixTrie,
}

impl AccountTrie {
    fn new() -> Self {
        AccountTrie { trie: MerkleRadixTrie::new() }
    }

    fn root_hash(&self) -> Hash {
        self.trie.root_hash()
    }

    fn insert_account(&mut self, address: Address, account: Account) {
        let account_key = keccak256(&address);
        let encode_account = account.encode();

        self.trie.insert(&account_key, encode_account);
    }

    fn get_account(&self, address: Address) -> Option<Account> {
        let account_key = keccak256(&address);
        let encoded_account = self.trie.get(&account_key)?;
        Some(Account::decode(&encoded_account))
    }

    fn prove_account(&self, address: Address) -> Option<Vec<Vec<u8>>> {
        None
    }

    fn verify_account_proof(
        root: Hash,
        address: Address,
        account: &Account,
        proof: &[Vec<u8>],
    ) -> bool {
        true
    }
}

fn print_address(lable: &str, address: Address) {
    println!("{lable}: 0x{}", hex::encode(address));
}

fn print_hash(lable: &str, hash: Hash) {
    println!("{lable}: 0x{}", hex::encode(hash));
}
fn main() {
    let mut account_trie = AccountTrie::new();

    let alice: Address = [0x11u8; 20];
    let bob: Address = [0x22u8; 20];
    let carol: Address = [0x33u8; 20];

    let alice_account = Account::new_eoa(1, 1_000);
    let bob_account = Account::new_eoa(2, 2_000);
    let carol_account = Account::new_eoa(3, 3_000);

    account_trie.insert_account(alice, alice_account.clone());
    account_trie.insert_account(bob, bob_account.clone());
    account_trie.insert_account(carol, carol_account.clone());

    println!("=== address ===");
    print_address("alice", alice);
    print_address("bob", bob);
    print_address("carol", carol);

    println!();
    println!("=== hashed account keys ===");
    print_hash("keccak256(alice)", keccak256(&alice));
    print_hash("keccak256(bob)", keccak256(&bob));
    print_hash("keccak256(carol)", keccak256(&carol));

    println!();
    println!("=== root ===");
    let loaded_alice = account_trie
        .get_account(alice)
        .expect("alice account should exist");

    println!("alice account: {:?}", loaded_alice);

    assert_eq!(loaded_alice, alice_account);

    

}
