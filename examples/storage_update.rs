use toy_merkle_radix_account_trie::{Account, Address, State, build_genesis_state};

fn main() {
    let contract: Address = [0x33u8; 20];
    let slot_key = [0x44u8; 32];
    let slot_value = b"stored-value".to_vec();

    let mut state = build_genesis_state(vec![(contract, Account::new_eoa(0, 0))]);
    let old_state_root = state.root_hash();
    let old_storage_root = state
        .get_account(contract)
        .expect("contract account should exist")
        .storage_root;

    state
        .set_storage_slot(contract, slot_key, slot_value.clone())
        .expect("storage write should succeed");

    let updated_account = state
        .get_account(contract)
        .expect("contract account should still exist");
    let loaded_value = state
        .get_storage_slot(contract, slot_key)
        .expect("storage slot should exist");
    let proof = state
        .prove_account(contract)
        .expect("account proof should exist after storage update");
    let proof_valid =
        State::verify_account_proof(state.root_hash(), contract, &updated_account, &proof);

    assert_eq!(loaded_value, slot_value);
    assert_ne!(updated_account.storage_root, old_storage_root);
    assert_ne!(state.root_hash(), old_state_root);
    assert!(proof_valid);

    println!("old state root: 0x{}", hex::encode(old_state_root));
    println!("new state root: 0x{}", hex::encode(state.root_hash()));
    println!(
        "new storage root: 0x{}",
        hex::encode(updated_account.storage_root)
    );
    println!("account proof valid: {proof_valid}");
}
