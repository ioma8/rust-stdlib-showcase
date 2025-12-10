# Iterators

## Overview

Iterators are Rust's primary tool for working with sequences. They enable functional programming patterns with methods like `map`, `filter`, and `fold`.

## Code Example

```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .filter(|x| x > &3)
    .collect();

println!("Original: {:?}", numbers);
println!("Doubled and filtered (>3): {:?}", doubled);
```

## Explanation

- **`iter()`**: Creates iterator over immutable references
- **`map()`**: Transforms each element
- **`filter()`**: Keeps elements matching predicate
- **`collect()`**: Consumes iterator into collection

## Key Concepts

### Iterator Types
```rust
let vec = vec![1, 2, 3];

let iter = vec.iter();         // &T
let iter_mut = vec.iter_mut(); // &mut T
let into_iter = vec.into_iter(); // T (consumes vec)
```

### Common Methods
```rust
// Transforming
.map(|x| x * 2)
.filter(|x| x > &0)
.filter_map(|x| Some(x))
.flat_map(|x| vec![x, x])

// Aggregating  
.sum(), .product()
.min(), .max()
.count()
.fold(0, |acc, x| acc + x)
.reduce(|acc, x| acc + x)

// Taking/Skipping
.take(5)
.skip(2)
.take_while(|x| x < &10)
.skip_while(|x| x < &5)

// Finding
.find(|x| x == &3)
.position(|x| x == &3)
.any(|x| x > &5)
.all(|x| x > &0)
```

## Use Cases

- **Data Transformation**: Process collections functionally
- **Lazy Evaluation**: Compute only what's needed
- **Chaining Operations**: Compose operations clearly
- **Infinite Sequences**: Generate unlimited data on-demand

## Best Practices

✅ **Do:**
- Chain operations for clarity
- Use iterator methods instead of manual loops
- Leverage lazy evaluation for efficiency
- Use `collect()` to convert to collections

❌ **Don't:**
- Collect unnecessarily (stay in iterator chain)
- Use `unwrap()` in iterator chains without good reason
- Forget iterators are lazy (must consume them)

## Related Features

- [Collections](../basic/collections.md) - Data structures to iterate over

## Learn More

- [std::iter documentation](https://doc.rust-lang.org/std/iter/)
- [The Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
