# Rc and RefCell

## Overview

`Rc<T>` provides shared ownership, while `RefCell<T>` enables interior mutability with runtime borrow checking.

## Code Example

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

// Clone the Rc to show shared ownership
let shared_clone = Rc::clone(&shared_data);

// Modify through RefCell
shared_data.borrow_mut().push(4);

println!("Original Rc count: {}", Rc::strong_count(&shared_data));
println!("Shared data: {:?}", shared_data.borrow());
println!("Clone also sees: {:?}", shared_clone.borrow());
```

## Key Concepts

- **Rc**: Reference counting for shared ownership (single-threaded)
- **RefCell**: Runtime-checked interior mutability
- Use `Arc<Mutex<T>>` for multi-threaded version

## Learn More

- [std::rc documentation](https://doc.rust-lang.org/std/rc/)
- [std::cell documentation](https://doc.rust-lang.org/std/cell/)
