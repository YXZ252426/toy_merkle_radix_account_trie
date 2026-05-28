use toy_merkle_radix_account_trie::{Account, AccountTrie, Address, Hash, keccak256};

fn print_address(label: &str, address: Address) {
    println!("{label}: 0x{}", hex::encode(address));
}

fn print_hash(label: &str, hash: Hash) {
    println!("{label}: 0x{}", hex::encode(hash));
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
    let root = account_trie.root_hash();
    print_hash("account trie root", root);

    println!();
    println!("=== read account ===");
    let loaded_alice = account_trie
        .get_account(alice)
        .expect("alice account should exist");

    println!("alice account: {:?}", loaded_alice);

    assert_eq!(loaded_alice, alice_account);

    println!();
    println!("=== generate proof ===");
    let proof = account_trie
        .prove_account(alice)
        .expect("proof should exist");

    println!("proof node count: {}", proof.len());

    println!();
    println!("=== verify proof ===");
    let ok = AccountTrie::verify_account_proof(root, alice, &alice_account, &proof);

    println!("valid alice proof: {ok}");

    println!();
    println!("=== fake proof test ===");
    let fake_alice_account = Account::new_eoa(1, 999_999);

    let fake_ok = AccountTrie::verify_account_proof(root, alice, &fake_alice_account, &proof);

    println!("valid fake alice proof: {fake_ok}");
    assert!(ok);
    assert!(!fake_ok);
}
