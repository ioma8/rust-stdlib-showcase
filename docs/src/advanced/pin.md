# Pin and PhantomPinned

## Overview

`Pin<P>` prevents moving of a value in memory, enabling self-referential structs and safe async implementations.

## Code Example

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

struct SelfReferential {
    data: String,
    data_ptr: *const String,
    _pin: PhantomPinned,
}

impl SelfReferential {
    fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfReferential {
            data,
            data_ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });
        
        let data_ptr = &boxed.data as *const String;
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).data_ptr = data_ptr;
        }
        boxed
    }
    
    fn get_data(&self) -> &str {
        &self.data
    }
}

let pinned = SelfReferential::new("Pinned data".to_string());
println!("Pinned data: {}", pinned.get_data());
```

## Key Concepts

- **Pin**: Guarantees value won't move in memory
- **PhantomPinned**: Marks type as !Unpin
- Essential for self-referential structs and async/await

## Learn More

- [std::pin documentation](https://doc.rust-lang.org/std/pin/)
- [Pin and suffering](https://fasterthanli.me/articles/pin-and-suffering)
