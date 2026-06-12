use toy_merkle_radix_account_trie::{
    Account, Address, Transaction, build_block, build_genesis_state,
};

fn main() {
    let alice: Address = [0x11u8; 20];
    let bob: Address = [0x22u8; 20];

    let mut state = build_genesis_state(vec![
        (alice, Account::new_eoa(0, 1_000)),
        (bob, Account::new_eoa(0, 50)),
    ]);
    let parent_root = state.root_hash();
    let transactions = vec![
        Transaction::new_transfer(alice, bob, 0, 100),
        Transaction::new_transfer(alice, bob, 1, 50),
    ];

    let block = build_block([0u8; 32], 1, 1_700_000_001, &state, transactions)
        .expect("block should build from valid transactions");
    let result = state
        .process_block(&block)
        .expect("block should process from parent state");

    let alice_account = state
        .get_account(alice)
        .expect("alice should remain in state");
    let bob_account = state.get_account(bob).expect("bob should remain in state");

    assert_ne!(result.post_state_root, parent_root);
    assert_eq!(result.post_state_root, state.root_hash());
    assert_eq!(alice_account.nonce, 2);
    assert_eq!(alice_account.balance, 850);
    assert_eq!(bob_account.balance, 200);
    assert_eq!(block.header.state_root, result.post_state_root);
    assert_eq!(block.header.transactions_root, result.transactions_root);
    assert_eq!(block.header.receipts_root, result.receipts_root);

    println!("block hash: 0x{}", hex::encode(block.hash()));
    println!("parent state root: 0x{}", hex::encode(parent_root));
    println!("post state root: 0x{}", hex::encode(result.post_state_root));
    println!("receipts: {}", result.receipts.len());
}
