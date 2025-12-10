# Memory Operations

## Overview

The `std::mem` module provides functions for memory inspection and manipulation.

## Code Example

```rust
use std::mem;

let value = 42;
let size = mem::size_of_val(&value);
let align = mem::align_of_val(&value);
println!("Value: {}, Size: {} bytes, Alignment: {} bytes", value, size, align);

let mut data = vec![1, 2, 3];
let capacity_before = data.capacity();
data.reserve(10);
let capacity_after = data.capacity();
println!("Capacity before: {}, after: {}", capacity_before, capacity_after);
```

## Key Concepts

- **`size_of()`**: Get type/value size in bytes
- **`align_of()`**: Get type/value alignment
- **`drop()`**: Manually drop a value
- **`swap()`**: Swap two mutable locations

## Learn More

- [std::mem documentation](https://doc.rust-lang.org/std/mem/)
