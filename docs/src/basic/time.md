# Time Operations

## Overview

The `std::time` module provides types for measuring time and durations. `Instant` measures elapsed time, while `Duration` represents a span of time.

## Code Example

```rust
use std::time::{Duration, Instant};
use std::thread;

let start = Instant::now();
thread::sleep(Duration::from_millis(100));
let duration = start.elapsed();
println!("Slept for {:?}", duration);
```

## Explanation

- **`Instant::now()`**: Captures the current moment (monotonic clock)
- **`Duration::from_millis()`**: Creates a time span from milliseconds
- **`thread::sleep()`**: Pauses execution for a duration
- **`elapsed()`**: Calculates time passed since the instant

## Key Concepts

### Instant vs SystemTime
- **`Instant`**: Monotonic clock, never goes backward, for measuring durations
- **`SystemTime`**: System clock, can change, for wall-clock time

### Duration Creation
```rust
Duration::from_secs(5)
Duration::from_millis(100)
Duration::from_micros(500)
Duration::from_nanos(1000)
```

### Duration Operations
```rust
let d1 = Duration::from_secs(1);
let d2 = Duration::from_millis(500);
let total = d1 + d2;  // 1.5 seconds
let diff = d1 - d2;   // 0.5 seconds
```

## Use Cases

- **Performance Measurement**: Benchmark code execution time
- **Timeouts**: Implement operation timeouts
- **Rate Limiting**: Control request rates
- **Animations**: Time-based animations and updates
- **Delays**: Pause execution between operations

## Best Practices

✅ **Do:**
- Use `Instant` for measuring elapsed time
- Use `SystemTime` for timestamps and dates
- Check duration with `as_secs()`, `as_millis()` for specific units

❌ **Don't:**
- Compare `Instant` from different systems
- Use `SystemTime` for duration measurements (can go backward)
- Create tight sleep loops (wastes CPU)

## Common Patterns

### Timeout Pattern
```rust
let timeout = Duration::from_secs(5);
let start = Instant::now();

while start.elapsed() < timeout {
    // Try operation
    if try_operation() {
        break;
    }
    thread::sleep(Duration::from_millis(100));
}
```

### Benchmarking Pattern
```rust
let start = Instant::now();
expensive_operation();
let duration = start.elapsed();
println!("Operation took: {:?}", duration);
```

## Related Features

- [Threading](./threading.md) - Thread sleep operations
- [Process Execution](./process.md) - Timing external processes

## Learn More

- [std::time documentation](https://doc.rust-lang.org/std/time/)
