`&[u8]` = borrowed byte slice.

It means:

```rust
(pointer to first byte, length)
```

It does not own the bytes.

It can point to bytes from many sources:

```rust
let arr: [u8; 32] = [0; 32];
let vec: Vec<u8> = vec![1, 2, 3];

let a: &[u8] = &arr;
let b: &[u8] = &vec;
```

Use `&[u8]` when a function only needs to read bytes and does not care where they came from or how long they are.