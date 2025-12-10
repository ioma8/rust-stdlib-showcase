# Option Type

## Overview

The `Option<T>` type represents an optional value: either `Some(T)` containing a value, or `None` representing absence. This eliminates null pointer errors at compile time.

## Code Example

```rust
let some_value: Option<i32> = Some(5);
let none_value: Option<i32> = None;

println!("Some value: {:?}", some_value);
println!("None value: {:?}", none_value);

if let Some(val) = some_value {
    println!("Unwrapped some value: {}", val);
}
```

## Explanation

- **`Option<T>`**: Enum that can be `Some(T)` or `None`
- **`if let`**: Pattern matching syntax for conditional unwrapping
- No null pointers - absence is explicitly represented by `None`

## Key Concepts

### Option Definition
```rust
enum Option<T> {
    Some(T),  // Contains a value
    None,     // No value
}
```

### Pattern Matching
```rust
let x: Option<i32> = Some(5);

match x {
    Some(val) => println!("Got: {}", val),
    None => println!("No value"),
}
```

### Common Methods
```rust
let x = Some(5);

// Unwrap (panics if None)
let val = x.unwrap();

// Unwrap with message
let val = x.expect("Value required");

// Unwrap or default
let val = x.unwrap_or(0);

// Unwrap or compute default
let val = x.unwrap_or_else(|| 0);

// Check if Some/None
if x.is_some() { }
if x.is_none() { }

// Map value if Some
let doubled = x.map(|n| n * 2);

// And then (flatMap)
let result = x.and_then(|n| Some(n * 2));

// Filter
let filtered = x.filter(|&n| n > 3);

// Take value, leave None
let taken = x.take();
```

## Use Cases

- **Optional Fields**: Struct fields that may not be present
- **Search Results**: Finding items that may not exist
- **Configuration**: Optional settings with defaults
- **Nullable Return Values**: Functions that may not return a value
- **Partial Data**: Data that may be incomplete

## Best Practices

✅ **Do:**
- Use `Option` instead of sentinel values (like -1, null)
- Use `?` operator with Option in functions returning Option
- Provide defaults with `unwrap_or()` or `unwrap_or_else()`
- Use `if let` or `match` for clear intent

❌ **Don't:**
- Use `unwrap()` without good reason (prefer `?` or explicit handling)
- Return `Option<bool>` (use Result or custom enum instead)
- Overuse `is_some()` checks (prefer pattern matching)

## Common Patterns

### Chaining Operations
```rust
fn find_user_city(user_id: i32) -> Option<String> {
    find_user(user_id)
        .and_then(|user| user.address)
        .map(|addr| addr.city)
}
```

### Optional with Default
```rust
let config = Config {
    timeout: config_value.unwrap_or(30),
    retries: config_value.unwrap_or_default(),
};
```

### Option in Structs
```rust
struct User {
    name: String,
    email: Option<String>,  // Email is optional
    phone: Option<String>,
}

impl User {
    fn contact_info(&self) -> String {
        self.email.as_ref()
            .or(self.phone.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("No contact info")
            .to_string()
    }
}
```

### Iterator Methods Returning Option
```rust
let numbers = vec![1, 2, 3, 4, 5];

// Find first matching element
let first_even = numbers.iter().find(|&&x| x % 2 == 0);

// Get first/last element
let first = numbers.first();  // Option<&i32>
let last = numbers.last();

// Get by index
let third = numbers.get(2);  // Option<&i32>
```

### Converting Option to Result
```rust
let opt: Option<i32> = Some(5);

// Convert to Result
let result: Result<i32, &str> = opt.ok_or("No value found");

// With computed error
let result = opt.ok_or_else(|| "No value found");
```

### Early Return with ?
```rust
fn get_config_value(key: &str) -> Option<String> {
    let config = load_config()?;  // Return None if load fails
    let value = config.get(key)?;  // Return None if key missing
    Some(value.clone())
}
```

## Option vs Result

Use `Option` when:
- Value may or may not be present
- Absence is not an error (just empty)
- No additional error information needed

Use `Result` when:
- Operation can fail with specific errors
- Need to know why operation failed
- Error information is important for debugging

## Advanced Techniques

### Option Combinators
```rust
let x = Some(5);
let y = Some(10);

// Combine two Options
let sum = x.and_then(|a| y.map(|b| a + b));

// Choose first Some
let first = x.or(y);

// XOR (one but not both)
let xor = x.xor(y);
```

### Transpose (Option<Result> ⟷ Result<Option>)
```rust
let x: Option<Result<i32, &str>> = Some(Ok(5));
let y: Result<Option<i32>, &str> = x.transpose();
```

### Flattening Nested Options
```rust
let nested: Option<Option<i32>> = Some(Some(5));
let flattened: Option<i32> = nested.flatten();
```

## Related Features

- [Error Handling](./error-handling.md) - Using Result for errors
- [Collections](./collections.md) - Methods returning Option

## Learn More

- [std::option documentation](https://doc.rust-lang.org/std/option/)
- [The Rust Book - Option](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values)
