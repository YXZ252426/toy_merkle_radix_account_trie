# Current Limitations

This document records the project state after Milestone 2 Phase 2A.

## Trie Structure

- `MptTrie` has branch, leaf, and extension nodes.
- Leaf and extension paths use hex-prefix compact encoding.
- `MptTrie` supports insert, get, and inclusion proofs.
- The old branch-only `MerkleRadixTrie` still exists as legacy/demo code.
- `AccountTrie` now uses `MptTrie`.

## Node References

- Every child reference is a 32-byte hash.
- The implementation does not inline small RLP nodes.
- The node database is an in-memory `HashMap<Hash, Vec<u8>>`.
- There is no database trait yet, so trie logic is coupled to the memory backend.

## State Model

- The account trie stores only account values and is now backed by `MptTrie`.
- Contract storage tries are not implemented.
- Transaction and receipt tries are not implemented.
- Account `storage_root` and `code_hash` are placeholders in the current demo.

## Operations

- MPT insert and get are implemented.
- MPT inclusion proofs are implemented for existing keys.
- Non-inclusion proofs are not implemented.
- Delete is not implemented.
- There is no staged state, rollback, or block-level commit model.

## Error Handling

- Internal decode paths currently use `expect` and `assert_eq`.
- Public-library style `Result` errors are not implemented yet.
- MPT proof verification rejects malformed proof nodes without panicking.
- Some legacy toy trie decode paths can still panic on malformed internal data.

## Execution Layer

- There are no transaction types.
- There are no block or header types.
- There is no block processor.
- State transitions are not implemented.

This is acceptable for Milestone 2 Phase 2A. Later milestones should add storage tries, transaction/receipt tries, database abstraction, and block execution without treating the current in-memory MPT as a complete Ethereum-compatible trie.
