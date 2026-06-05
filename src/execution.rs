use crate::account::{Account, AccountTrie};
use crate::types::{Address, Hash};


#[derive(Debug, Clone, Default)]
pub struct State {
    accounts: AccountTrie,
}

impl State {
    pub fn new() -> Self {
        Self { accounts: AccountTrie::new() }
    }
    pub fn root_hash(&self) -> Hash {
        self.accounts.root_hash()
    }
    pub fn create_account(&mut self, address: Address, account: Account) {
        self.accounts.insert_account(address, account);
    }
    pub fn update_account(&mut self, address: Address, account: Account) {
        self.accounts.insert_account(address, account);
    }
    pub fn get_account(&self, address: Address) -> Option<Account> {
        self.accounts.get_account(address)
    }
    pub fn prove_account(&self, address: Address) -> Option<Vec<Vec<u8>>> {
        self.accounts.prove_account(address)
    }
    pub fn verify_account_proof(
        root: Hash,
        address: Address,
        account: &Account,
        proof: &[Vec<u8>],
    ) -> bool {
        AccountTrie::verify_account_proof(root, address, account, proof)
    }
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn empty_state_has_empty_account_root() {
        let state = State::new();

        assert_eq!(state.root_hash(), AccountTrie::new().root_hash());
    }
    
    #[test]
    fn state_creates_and_loads_account() {
        let mut state = State::new();
        let address = [0x11u8; 20];
        let account = Account::new_eoa(1, 100);

        state.create_account(address, account.clone());

        assert_eq!(state.get_account(address), Some(account));
    }

    #[test]
    fn updating_account_changes_state_root() {
        let mut state = State::new();
        let address = [0x11u8; 20];

        state.create_account(address, Account::new_eoa(1, 100));
        let first_root = state.root_hash();
        state.update_account(address, Account::new_eoa(1, 200));

        assert_ne!(state.root_hash(), first_root);
    }

    #[test]
    fn writing_same_account_keeps_state_root_stable() {
        let mut state = State::new();
        let address = [0x11u8; 20];
        let account = Account::new_eoa(1, 100);

        state.create_account(address, account.clone());
        let first_root = state.root_hash();
        state.update_account(address, account);

        assert_eq!(state.root_hash(), first_root);
    }

    #[test]
    fn state_account_proof_verifies() {
        let mut state = State::new();
        let address = [0x11u8; 20];
        let account = Account::new_eoa(1, 100);

        state.create_account(address, account.clone());
        let proof = state
            .prove_account(address)
            .expect("account proof should exist");

        assert!(State::verify_account_proof(
            state.root_hash(), 
            address, 
            &account,
            &proof
        ));
    }
}