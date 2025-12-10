# Any Type

## Overview

The `Any` trait enables runtime type inspection and downcasting for dynamic typing scenarios.

## Code Example

```rust
use std::any::Any;

let string_value: &dyn Any = &"Hello, Any!".to_string();
let int_value: &dyn Any = &42i32;

if let Some(s) = string_value.downcast_ref::<String>() {
    println!("String value: {}", s);
}

if let Some(i) = int_value.downcast_ref::<i32>() {
    println!("Integer value: {}", i);
}
```

## Key Concepts

- **Any**: Trait for runtime type information
- **downcast_ref**: Safely convert to concrete type
- Useful for plugin systems, dynamic dispatch

## Learn More

- [std::any documentation](https://doc.rust-lang.org/std/any/)
