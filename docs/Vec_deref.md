`Vec<T>` owns heap data.

Internally:

```text
Vec<T> = ptr + len + capacity
```

When `Vec<T>` derefs, it returns a slice reference:

```rust
&[T]
```

A slice reference contains:

```text
&[T] = ptr + len
```

So deref gives a borrowed view of the initialized data only. It does not expose `capacity`.

Example:

```rust
let v = vec![1, 2, 3];

let s: &[u8] = &v; // &Vec<u8> coerces to &[u8]
```

This does not move or copy the data. It only borrows the bytes.

`&[T]` can read elements, but cannot grow the vector because it has no capacity information.