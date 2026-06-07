# Limitations Devlog

This document records limitations by milestone. Older entries are historical snapshots, not the current project state.

## After Milestone 0

Milestone 0 stabilized the original demo and made the toy behavior explicit.

- The trie was still a branch-only toy radix trie.
- There were no leaf or extension nodes.
- Paths did not use Ethereum hex-prefix compact encoding.
- Account keys and storage roots were demo-level placeholders.
- Proofs only matched the branch-only structure.
- There was no reusable library boundary yet.
- There were no storage tries, transaction tries, receipt tries, or execution APIs.
- Trie decode paths could panic on malformed internal data.

## After Milestone 1

Milestone 1 split the demo into reusable modules and exposed a library API.

- The public crate API existed, but the trie was still the branch-only toy trie.
- `execution` existed only as a placeholder module.
- Account values had deterministic RLP encoding, but account state was not a real world-state model yet.
- `storage_root` and `code_hash` were stored on accounts but not backed by real storage or code data.
- There were still no leaf or extension nodes, no compact path encoding, no storage trie, and no transaction/receipt trie.
- There was no persistent database abstraction.

## After Milestone 2

Milestone 2 Phase 2A replaced the account trie backend with a real MPT core.

- `MptTrie` had branch, leaf, and extension nodes.
- Leaf and extension paths used hex-prefix compact encoding.
- `MptTrie` supported insert, get, and inclusion proofs.
- `AccountTrie` used `MptTrie`.
- The old branch-only `MerkleRadixTrie` still existed as legacy/demo code.
- Every child reference was a 32-byte hash; inline small RLP nodes were not modeled.
- The node database was still an in-memory `HashMap<Hash, Vec<u8>>`.
- Trie logic was still coupled to the memory backend because there was no database trait.
- Contract storage tries were not implemented yet.
- Transaction and receipt tries were not implemented.
- Non-inclusion proofs were not implemented.
- Delete was not implemented.
- There was no staged state, rollback, or block-level commit model.

## After Milestone 3

Milestone 3 added an in-memory world-state wrapper and per-account storage tries.

- `State` wraps the account trie and supports account create, load, and update.
- Contract storage tries are implemented for the current in-memory `State`.
- Storage trie keys are `keccak256(slot_key)`.
- Storage trie values use RLP byte encoding.
- A storage write updates the account `storage_root` and then the global state root.
- Storage tries are kept in a per-address in-memory map and cannot yet be reopened from an arbitrary non-empty storage root.
- Account trie and storage trie both use `MptTrie`, but they do not yet share a database backend.
- `code_hash` is stored on accounts, but code bytes are not stored or executed.
- Transaction and receipt tries are not implemented.
- Account deletion and storage slot deletion are not implemented.
- Non-inclusion proofs are not implemented.
- There is no staged state, rollback, or block-level commit model.
- Some internal decode paths still use `expect` and `assert_eq`.
- `State::set_storage_slot` returns `StateError` for missing accounts and unavailable storage tries.
- MPT proof verification rejects malformed proof nodes without panicking.
- Some legacy toy trie decode paths can still panic on malformed internal data.
- There are no transaction types, block types, headers, or block processor.
- Transaction-driven state transitions are not implemented.

This is acceptable for Milestone 3. Later milestones should add transaction/receipt tries, database abstraction, block types, and block execution without treating the current in-memory MPT as a complete Ethereum-compatible trie.
