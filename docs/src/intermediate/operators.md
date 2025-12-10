# Operator Overloading

## Overview

Rust allows operator overloading through trait implementation (`Add`, `Sub`, `Mul`, etc.).

## Code Example

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Add for Complex {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

let c1 = Complex { real: 1.0, imag: 2.0 };
let c2 = Complex { real: 3.0, imag: 4.0 };
let c3 = c1 + c2;
println!("Complex addition: {:?} + {:?} = {:?}", c1, c2, c3);
```

## Key Concepts

- Implement operator traits to enable custom operators
- Supports: `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `<<`, `>>`, etc.
- Can implement for assignment operators too

## Learn More

- [std::ops documentation](https://doc.rust-lang.org/std/ops/)
