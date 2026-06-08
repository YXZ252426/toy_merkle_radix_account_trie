# Execution Layer Milestones

Goal: evolve this toy account trie into a small but complete execution-layer library that can process blocks, apply transactions, update database-backed state, and expose reusable APIs for later projects.

This project should stay intentionally small. It does not need full Ethereum compatibility at first, but the data model and root calculation should follow Ethereum-like concepts closely enough that later compatibility work is incremental instead of a rewrite.

## Milestone 0: Stabilize the Current Demo

Purpose: make the existing toy trie behavior explicit before replacing internals.

Status: completed as the baseline for the current branch-only toy trie. Current limitations are recorded in `docs/CURRENT_LIMITATIONS.md`.

Deliverables:

- Keep the current account insert/get/proof demo working.
- Add focused tests for account encoding, trie insert/get, valid proof, and invalid proof.
- Document current limitations: only branch nodes, no leaf/extension nodes, no deletion, no real Ethereum MPT compact encoding.

Acceptance criteria:

- `cargo test` passes.
- `cargo run` still prints a valid Alice proof and rejects a fake account proof.

## Milestone 1: Turn the Demo into a Library Crate

Purpose: stop building everything in `main.rs` and create reusable module boundaries before the real trie grows.

Status: completed for the current toy trie baseline. The crate now exposes public library APIs through `src/lib.rs`, and `src/main.rs` is only an example binary. A separate `rlp` module was intentionally skipped for now because RLP usage is still local to account and trie encoding.

Deliverables:

- Add `src/lib.rs`.
- Split code into modules:
  - `types`: `Hash`, `Address`, numeric aliases.
  - `crypto`: `keccak256`.
  - `rlp`: small encode/decode helpers if needed.
  - `trie`: generic trie and proof logic.
  - `account`: account model and account trie wrapper.
  - `execution`: placeholder module for later block/transaction processing.
- Keep `src/main.rs` as a small example binary using the library.
- Replace panics in public APIs with `Result` where invalid input can come from callers.

Acceptance criteria:

- External code can import the crate and create an `AccountTrie`.
- The example binary uses only public library APIs.
- Tests cover public APIs, not private demo internals.

## Milestone 2: Replace the Toy Radix Trie with a Real MPT Core

Purpose: implement the real trie structure before adding storage, transactions, and blocks on top of it.

Status: Phase 2A completed. The project now has a real MPT core with `Branch`, `Leaf`, and `Extension` nodes, hex-prefix compact path encoding, RLP node encoding/decoding, an in-memory MPT node database, `MptTrie::get`, `MptTrie::insert`, inclusion proofs, and `AccountTrie` backed by `MptTrie`. Ethereum inline small-node references, delete, and non-inclusion proofs are intentionally deferred to a later compatibility/hardening pass.

Deliverables:

- Implement node types:
  - `Branch`
  - `Leaf`
  - `Extension`
- Implement hex-prefix compact encoding for leaf and extension paths.
- Implement Ethereum-style node references:
  - inline encoded node when small enough, if you choose to model that now.
  - hashed reference for larger encoded nodes.
- Implement insert, get, update, and delete.
- Support inclusion proofs and non-inclusion proofs.
- Define the empty trie root consistently.
- Keep the trie generic over byte keys and byte values.

Acceptance criteria:

- Tests cover path splitting, shared prefixes, overwrites, deletes, and proof verification.
- Root hashes are deterministic across insertion order for the same final key/value set.
- The old branch-only trie is removed or clearly isolated as historical/demo code.

Phase 2A notes:

- Node references are always 32-byte hashes. Inline child references are not modeled yet.
- Inclusion proofs are implemented. Non-inclusion proofs are deferred.
- Insert/get/proof are implemented. Delete is deferred.
- The old branch-only `MerkleRadixTrie` remains isolated as legacy/demo code while `AccountTrie` uses `MptTrie`.

## Milestone 3: Add Account State Trie and Storage Tries

Purpose: model world state as accounts, and each account's contract storage as its own trie.

Status: completed for the current in-memory execution state model. `State` now wraps the account trie, supports account create/load/update, supports per-account storage slot reads and writes, updates the account `storage_root` after storage writes, and therefore updates the global state root. Storage trie keys use `keccak256(slot_key)` and storage values use RLP byte encoding. Shared persistent node databases and reopening storage tries from an arbitrary historical root are deferred to Milestone 5.

Deliverables:

- Define an `Account` that stores:
  - nonce
  - balance
  - storage root
  - code hash
- Implement `State` or `StateTrie` APIs:
  - create account
  - load account
  - update account
  - delete/tombstone account if needed
  - read storage slot
  - write storage slot
- Implement storage trie key/value encoding:
  - key: `keccak256(slot_key)`
  - value: RLP or canonical byte encoding for the slot value.
- Ensure account storage root changes when storage changes.
- Keep account trie and storage trie backed by the same node database abstraction.

Acceptance criteria:

- Updating Alice's balance changes the state root.
- Updating a contract storage slot changes that account's storage root and then the global state root.
- Rewriting a storage slot to the same value leaves the final root unchanged.

Milestone 3 notes:

- `State` keeps storage tries in memory per account address.
- `State::set_storage_slot` returns `StateError` when the account is missing or when the account has a non-empty storage root that this in-memory state cannot reconstruct.
- Delete/tombstone behavior is still deferred because `MptTrie` delete is not implemented yet.
- Account trie and storage trie both use `MptTrie`, but they do not yet share a database backend. A database trait and shared `MemoryDb` are part of Milestone 5.

## Milestone 4: Add Transaction and Receipt Tries

Purpose: support block-level roots beyond state root.

Status: completed for ordered transaction and receipt root calculation. The project now has simple transfer transaction and receipt data models, deterministic RLP encoding/decoding for both, RLP-encoded ordered trie indexes, `transaction_root`, and `receipt_root`. These roots are derived from ordered lists using `MptTrie`. Actual transaction execution, receipt generation from state transitions, block headers, and root validation are deferred to later milestones.

Deliverables:

- Define transaction types for the first execution model:
  - simple transfer transaction first.
  - optional contract/storage transaction later.
- Define receipt type:
  - success/failure
  - gas used placeholder or simple accounting field
  - logs placeholder if needed later.
- Build transaction trie:
  - key: RLP-encoded transaction index.
  - value: encoded transaction.
- Build receipt trie:
  - key: RLP-encoded transaction index.
  - value: encoded receipt.
- Add root calculation helpers for ordered transaction and receipt lists.

Acceptance criteria:

- A block with the same ordered transactions produces the same transaction root.
- Reordering transactions changes the transaction root.
- Processing transactions produces receipts and a receipt root.

Milestone 4 notes:

- Transaction values are simple transfer transactions with `from`, `to`, `nonce`, and `value`.
- Receipt values contain `success`, `gas_used`, and an optional error string.
- Transaction and receipt trie keys are RLP-encoded list indexes.
- `transaction_root` and `receipt_root` are deterministic for the same ordered inputs and change when item content or ordering changes.
- Receipts are not produced by an executor yet. They are explicitly supplied to `receipt_root`.

## Milestone 5: Add Persistent Database Abstractions

Purpose: separate trie/state logic from storage backend so later code can use memory, file, or embedded DB storage.

Status: completed for the in-memory database abstraction layer. The project now has a `NodeDatabase` trait, a `MemoryNodeDb` implementation, and `MptTrie<Db = MemoryNodeDb>` that depends on the trait instead of directly depending on `HashMap`. Trie state can be split into `(database, root)` and reopened with the same database and saved root. Account, storage, transaction, receipt, and `State` account snapshots now have tests covering root/database reopen behavior. File-backed databases, staged overlays, and full storage-trie persistence inside `State` are deferred.

Deliverables:

- Define a node database trait, for example:
  - `get(hash) -> Option<Vec<u8>>`
  - `put(encoded_node) -> Hash`
  - optional batch writes.
- Implement `MemoryDb`.
- Optionally add a simple file-backed or sled/rocksdb-backed implementation later.
- Decide how state snapshots are represented:
  - root hash plus shared database.
  - staged overlay for block execution.

Acceptance criteria:

- Trie code depends on a database trait, not directly on `HashMap`.
- Tests can run with `MemoryDb`.
- A state root can be saved and later used to reopen/read the same state from the same database.

Milestone 5 notes:

- `MptNodeDb` remains as a compatibility alias for `MemoryNodeDb`.
- `MptTrie::new()` still creates an in-memory trie, preserving the existing public API.
- `MptTrie::with_db(db)` allows callers to supply a custom node database implementation.
- `MptTrie::into_parts()` and `MptTrie::from_root(db, root)` support root/database snapshots.
- `AccountTrie`, `StorageTrie`, transaction trie builders, receipt trie builders, and `State` account snapshots can be reopened from saved root/database pairs.
- `State` can reopen account state from a saved account root/database, but its per-account storage trie map is still in-memory and not fully persisted as a state snapshot.
- No file-backed or embedded database implementation exists yet.
- There is still no staged overlay, rollback, or block-level commit model.

## Milestone 6: Define Block and Execution Types

Purpose: create the objects the execution layer will process.

Deliverables:

- Define `Header`:
  - parent hash
  - number
  - state root
  - transactions root
  - receipts root
  - timestamp or simple metadata
- Define `Block`:
  - header
  - transactions
- Define `ExecutionResult`:
  - post-state root
  - receipts
  - transaction root
  - receipt root
- Define `ExecutionError`.
- Keep consensus fields minimal; execution only needs enough data to apply transactions and verify roots.

Acceptance criteria:

- A block can be encoded/decoded or at least deterministically hashed.
- Header roots are derived from actual state, transactions, and receipts.

## Milestone 7: Implement State Transition and Block Processing

Purpose: reach the first complete execution-layer loop: input parent state plus block transactions, output new state root.

Deliverables:

- Implement `Executor` or `BlockProcessor`.
- Apply simple transfer transactions:
  - check sender exists.
  - check nonce.
  - check balance.
  - subtract from sender.
  - add to recipient.
  - increment sender nonce.
  - emit receipt.
- Add atomic block execution:
  - either all valid transactions are committed.
  - or define per-transaction failure semantics explicitly.
- Compute final state root, transaction root, and receipt root.
- Validate that computed roots match block header roots when processing an existing block.

Acceptance criteria:

- Given a genesis state and one block, processing changes balances and returns a new state root.
- Invalid nonce or insufficient balance fails predictably.
- Reprocessing the same block from the same parent state produces the same result.

## Milestone 8: Public Library API and Examples

Purpose: make the crate useful for follow-up projects without exposing internal details.

Deliverables:

- Public API entry points:
  - create genesis state.
  - build a block.
  - process a block.
  - query account.
  - query storage.
  - generate and verify proofs.
- Add examples:
  - `examples/account_proof.rs`
  - `examples/process_block.rs`
  - `examples/storage_update.rs`
- Add crate-level documentation with the execution flow.

Acceptance criteria:

- A user can process a block without directly constructing trie internals.
- Examples compile and run.
- Public structs have stable names and clear ownership rules.

## Milestone 9: Hardening and Compatibility Pass

Purpose: improve correctness after the simple execution layer works end to end.

Deliverables:

- Add property-style tests for trie operations if practical.
- Add malformed RLP tests.
- Add proof tampering tests.
- Compare selected trie roots against known fixtures if available.
- Audit panic paths and replace public-facing panics with errors.
- Review naming against Ethereum concepts: state root, storage root, transaction root, receipt root.

Acceptance criteria:

- Public APIs return useful errors.
- Trie tests cover common structural edge cases.
- The implementation is ready for a second phase: EVM-like execution, logs bloom, gas, or networking.

## Recommended Build Order

1. Milestone 0: lock down current behavior with tests.
2. Milestone 1: split into a library crate.
3. Milestone 2: implement the real MPT core.
4. Milestone 3: build account state and storage trie on top of MPT.
5. Milestone 5: introduce the database abstraction before block execution becomes stateful.
6. Milestone 4: add transaction and receipt tries.
7. Milestone 6: define block, header, and execution result types.
8. Milestone 7: implement block processing.
9. Milestone 8: expose clean public APIs and examples.
10. Milestone 9: harden and compare against fixtures.

The only intentional ordering difference is that database abstraction comes before transaction/receipt tries in the build order. That keeps state, storage, transaction, and receipt tries using the same storage model instead of refactoring every trie later.

## First Implementation Slice

The next concrete task should be Milestone 0 plus the first half of Milestone 1:

- Move reusable code into `src/lib.rs`.
- Keep the current binary as an example driver.
- Add tests for current proof behavior.
- Define initial module names and public types.

After that, the real MPT replacement can happen behind the same public API with much less churn.
