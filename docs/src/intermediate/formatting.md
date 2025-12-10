# Custom Formatting

## Overview

The `std::fmt` module allows custom display formatting by implementing the `Display` and `Debug` traits.

## Code Example

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

let point = Point { x: 10, y: 20 };
println!("Custom formatted point: {}", point);
```

## Key Concepts

- **`Display`**: For user-facing output (`{}`)
- **`Debug`**: For programmer-facing output (`{:?}`)
- Implement traits to control formatting

## Learn More

- [std::fmt documentation](https://doc.rust-lang.org/std/fmt/)
