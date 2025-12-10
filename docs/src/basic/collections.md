# Collections

## Overview

Rust's `std::collections` module provides essential data structures. `HashMap` stores key-value pairs with O(1) average lookup, while `HashSet` stores unique values.

## Code Example

```rust
use std::collections::{HashMap, HashSet};

// HashMap example
let mut map = HashMap::new();
map.insert("key1", "value1");
map.insert("key2", "value2");
println!("HashMap: {:?}", map);

// HashSet example
let mut set = HashSet::new();
set.insert("item1");
set.insert("item2");
println!("HashSet: {:?}", set);
```

## Explanation

### HashMap
- **Key-Value Storage**: Fast lookups by key
- **Generic Types**: `HashMap<K, V>` where K: Hash + Eq
- **Methods**: `insert()`, `get()`, `remove()`, `contains_key()`

### HashSet
- **Unique Elements**: Automatically prevents duplicates
- **Fast Lookups**: O(1) average `contains()` check
- **Based on HashMap**: Essentially `HashMap<T, ()>`

## Key Concepts

### HashMap Operations
```rust
let mut scores = HashMap::new();

// Insert
scores.insert("Blue", 10);

// Get (returns Option)
if let Some(score) = scores.get("Blue") {
    println!("Score: {}", score);
}

// Update or insert
scores.entry("Blue").or_insert(50);

// Iterate
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### HashSet Operations
```rust
let mut set = HashSet::new();

// Insert (returns bool if new)
set.insert(1);

// Check membership
if set.contains(&1) {
    println!("Has 1");
}

// Set operations
let set2 = HashSet::from([2, 3]);
let union: HashSet<_> = set.union(&set2).collect();
let intersection: HashSet<_> = set.intersection(&set2).collect();
```

## Use Cases

### HashMap
- **Caching**: Store computed results by key
- **Indexing**: Fast lookup tables
- **Configuration**: Key-value settings
- **Counting**: Word/item frequency counting

### HashSet
- **Deduplication**: Remove duplicates from data
- **Membership Testing**: Fast "is this present?" checks
- **Set Operations**: Union, intersection, difference
- **Visited Tracking**: Graph algorithms, avoiding revisits

## Best Practices

✅ **Do:**
- Preallocate capacity if size is known: `HashMap::with_capacity(n)`
- Use `entry()` API for update-or-insert patterns
- Consider `BTreeMap`/`BTreeSet` for sorted iteration

❌ **Don't:**
- Use types without `Hash + Eq` as HashMap keys
- Modify keys after insertion (breaks invariants)
- Assume iteration order (use BTreeMap for sorted order)

## Common Patterns

### Frequency Counter
```rust
let mut freq = HashMap::new();
for word in text.split_whitespace() {
    *freq.entry(word).or_insert(0) += 1;
}
```

### Cache Pattern
```rust
let mut cache = HashMap::new();
let result = cache.entry(key)
    .or_insert_with(|| expensive_computation(key));
```

## Other Collections

- **Vec**: Dynamic array, contiguous memory
- **VecDeque**: Double-ended queue
- **BTreeMap/BTreeSet**: Sorted collections
- **LinkedList**: Doubly-linked list

## Related Features

- [Iterators](../intermediate/iterators.md) - Iterating over collections
- [Memory Operations](../intermediate/memory.md) - Capacity management

## Learn More

- [std::collections documentation](https://doc.rust-lang.org/std/collections/)
- [The Rust Book - Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
