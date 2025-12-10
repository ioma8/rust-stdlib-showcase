# Environment Variables

## Overview

The `std::env` module provides access to environment variables and process information. Query, set, and iterate over environment variables safely.

## Code Example

```rust
use std::env;

// Iterate over environment variables
for (key, value) in env::vars().take(3) {
    println!("{}: {}", key, value);
}
```

## Explanation

- **`env::vars()`**: Returns iterator over all environment variables
- **`take(3)`**: Limits to first 3 variables
- Returns `(String, String)` tuples of key-value pairs

## Key Concepts

### Reading Variables
```rust
use std::env;

// Get specific variable (returns Result)
match env::var("HOME") {
    Ok(val) => println!("HOME: {}", val),
    Err(e) => println!("Couldn't read HOME: {}", e),
}

// Get with default
let editor = env::var("EDITOR").unwrap_or("vim".to_string());

// Get as OsString (non-UTF8 safe)
let path = env::var_os("PATH");
```

### Setting Variables
```rust
use std::env;

// Set for current process and children
env::set_var("MY_VAR", "my_value");

// Remove variable
env::remove_var("MY_VAR");
```

⚠️ **Note**: Changes only affect current process and child processes, not parent.

### Process Information
```rust
// Current directory
let current_dir = env::current_dir()?;

// Change directory
env::set_current_dir("/tmp")?;

// Command line arguments
let args: Vec<String> = env::args().collect();

// Current executable path
let exe = env::current_exe()?;
```

## Use Cases

- **Configuration**: Read config from environment
- **CI/CD**: Detect CI environment, read secrets
- **System Info**: Get user home, temp directory
- **Path Resolution**: Find executables in PATH
- **Feature Flags**: Enable/disable features via env vars

## Best Practices

✅ **Do:**
- Use `var_os()` for paths (handles non-UTF8)
- Provide defaults with `.unwrap_or()` or `.unwrap_or_else()`
- Document required environment variables
- Validate variable values after reading

❌ **Don't:**
- Assume variables exist (always handle `Result`)
- Store sensitive data in environment (can leak in logs)
- Modify environment in library code (side effects)
- Use `unwrap()` without good reason

## Common Environment Variables

### Unix/Linux
- `HOME`: User's home directory
- `USER`: Current username
- `PATH`: Executable search path
- `SHELL`: User's shell
- `LANG`: Locale settings

### Windows
- `USERPROFILE`: User's profile directory
- `USERNAME`: Current username
- `PATH`: Executable search path
- `TEMP`/`TMP`: Temporary directory

### Cross-Platform
```rust
use std::env;

fn get_home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}
```

## Common Patterns

### Configuration Loading
```rust
fn load_config() -> Config {
    Config {
        db_url: env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set"),
        port: env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse()
            .expect("PORT must be a number"),
        debug: env::var("DEBUG").is_ok(),
    }
}
```

### PATH Parsing
```rust
use std::env;

fn find_in_path(name: &str) -> Option<PathBuf> {
    env::var_os("PATH")?
        .split(|c| c == ':' || c == ';')
        .map(PathBuf::from)
        .find_map(|dir| {
            let path = dir.join(name);
            if path.exists() {
                Some(path)
            } else {
                None
            }
        })
}
```

### Temporary Directory
```rust
use std::env;

let temp_dir = env::temp_dir();
let temp_file = temp_dir.join("myapp_temp.txt");
```

## Command Line Arguments

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    println!("Program: {}", args[0]);
    
    if args.len() > 1 {
        println!("First argument: {}", args[1]);
    }
    
    // Skip program name
    for arg in env::args().skip(1) {
        println!("Arg: {}", arg);
    }
}
```

## Related Features

- [Path Operations](./path.md) - Working with filesystem paths
- [Process Execution](./process.md) - Spawning child processes

## Learn More

- [std::env documentation](https://doc.rust-lang.org/std/env/)
