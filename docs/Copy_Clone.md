Here is a simple note using your example.

```rust
type Address = [u8; 20];

let alice: Address = [0x11u8; 20];

account_trie.insert_account(alice, alice_account.clone());
```

## `Clone`

`Clone` means a value can be explicitly duplicated by calling `.clone()`.

```rust
let account2 = alice_account.clone();
```

If `Account` has:

```rust
#[derive(Clone)]
struct Account {
    nonce: u64,
    balance: u128,
    code: Vec<u8>,
}
```

then Rust clones each field.

For simple numbers like `u64`, it just copies the value.

For heap data like `Vec<u8>`, it usually allocates a new vector and copies the data.

So:

```rust
alice_account.clone()
```

creates another `Account` value, so the original `alice_account` can still be used later.

## `Copy`

`Copy` means a value is duplicated automatically when passed or assigned.

```rust
let alice2 = alice;
```

Because `Address = [u8; 20]`, and `u8` is `Copy`, the whole address is also `Copy`.

So this:

```rust
account_trie.insert_account(alice, alice_account.clone());
```

does not move `alice` permanently. Rust copies the 20 bytes automatically.

You can still use `alice` after the call:

```rust
account_trie.insert_account(alice, alice_account.clone());

println!("{:?}", alice); // OK
```

## Key Difference

`Clone` is explicit:

```rust
let b = a.clone();
```

`Copy` is automatic:

```rust
let b = a;
```

For your example:

```rust
account_trie.insert_account(alice, alice_account.clone());
```

means:

```rust
// alice is Copy, so Rust copies it automatically
// alice_account is Clone, so you manually clone it
account_trie.insert_account(alice, alice_account.clone());
```

Short version:

```rust
Address        -> Copy, cheap 20-byte copy
Account        -> Clone, explicit duplicate
Account.clone() -> may copy heap data depending on fields
```