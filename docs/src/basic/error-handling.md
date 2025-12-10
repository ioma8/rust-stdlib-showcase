# Error Handling

## Overview

Rust uses the `Result<T, E>` type for recoverable errors. This explicit error handling prevents forgotten error checks and makes error paths clear.

## Code Example

```rust
let result: Result<i32, &str> = Ok(42);

match result {
    Ok(value) => println!("Success: {}", value),
    Err(e) => println!("Error: {}", e),
}
```

## Explanation

- **`Result<T, E>`**: Enum with `Ok(T)` for success or `Err(E)` for failure
- **`match`**: Pattern matching to handle both cases
- Forces explicit handling of success and failure paths

## Key Concepts

### Result Type
```rust
enum Result<T, E> {
    Ok(T),   // Success value
    Err(E),  // Error value
}
```

### Error Propagation with `?`
The `?` operator propagates errors automatically:
```rust
fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("config.txt")?;  // Returns early if Err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

### Common Result Methods
```rust
let result: Result<i32, &str> = Ok(42);

// Unwrap (panics on Err)
let value = result.unwrap();

// Unwrap with message
let value = result.expect("Failed to get value");

// Unwrap or default
let value = result.unwrap_or(0);

// Unwrap or compute default
let value = result.unwrap_or_else(|_| 0);

// Check if Ok/Err
if result.is_ok() { }
if result.is_err() { }

// Map success value
let doubled = result.map(|x| x * 2);

// Map error value
let result = result.map_err(|e| format!("Error: {}", e));
```

## Use Cases

- **I/O Operations**: File, network operations that can fail
- **Parsing**: Converting strings to numbers, dates, etc.
- **Validation**: Checking user input, business rules
- **API Calls**: HTTP requests, database queries
- **Resource Acquisition**: Opening files, connections

## Best Practices

✅ **Do:**
- Use `?` for error propagation in functions returning Result
- Provide context with `.map_err()` or custom error types
- Use `expect()` only when failure is truly unexpected
- Return `Result` from functions that can fail

❌ **Don't:**
- Use `unwrap()` in production code (use `?` or `expect()`)
- Ignore `Result` values (compiler warns about this)
- Create errors for control flow (use Option for that)
- Swallow errors without handling them

## Error Types

### Standard Library Errors
```rust
// I/O errors
fn read_file() -> Result<String, std::io::Error> { }

// Parse errors
fn parse_num(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

// Generic string errors
fn validate(s: &str) -> Result<(), String> {
    if s.is_empty() {
        Err("String is empty".to_string())
    } else {
        Ok(())
    }
}
```

### Custom Error Types
```rust
#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    ParseError(String),
    NotFound,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "I/O error: {}", e),
            MyError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            MyError::NotFound => write!(f, "Not found"),
        }
    }
}

impl std::error::Error for MyError {}
```

## Common Patterns

### Multiple Error Types
```rust
// Convert between error types
fn process() -> Result<i32, String> {
    let file = File::open("num.txt")
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    let num: i32 = "123".parse()
        .map_err(|e| format!("Failed to parse: {}", e))?;
    
    Ok(num)
}
```

### Early Return Pattern
```rust
fn validate_user(user: &User) -> Result<(), String> {
    if user.name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    
    if user.age < 18 {
        return Err("User must be 18+".to_string());
    }
    
    Ok(())
}
```

### Collecting Results
```rust
// Stop at first error
let results: Result<Vec<i32>, _> = vec!["1", "2", "3"]
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();

// Partition successes and failures
let (oks, errs): (Vec<_>, Vec<_>) = results
    .into_iter()
    .partition(Result::is_ok);
```

### Fallible Iterator
```rust
// Process items, stop on first error
fn process_all(items: &[&str]) -> Result<(), String> {
    for item in items {
        process_item(item)?;  // Stops on first error
    }
    Ok(())
}
```

## Result vs Panic

Use `Result` for:
- Expected, recoverable errors
- External I/O operations
- User input validation
- Business logic errors

Use `panic!` for:
- Programming bugs (assertions)
- Invariant violations
- Unrecoverable errors
- Prototyping

## Related Features

- [Option Type](./option.md) - Handling absence of values
- [Panic Handling](../advanced/panic.md) - Recovering from panics

## Learn More

- [std::result documentation](https://doc.rust-lang.org/std/result/)
- [The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Error Handling Survey](https://blog.burntsushi.net/rust-error-handling/)
