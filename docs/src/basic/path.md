# Path Operations

## Overview

The `std::path` module provides cross-platform path manipulation. `Path` and `PathBuf` handle filesystem paths safely across Windows, Unix, and other platforms.

## Code Example

```rust
use std::path::Path;

let path = Path::new("example.txt");
println!("Path exists: {}", path.exists());
println!("Path: {}", path.display());
```

## Explanation

- **`Path::new()`**: Creates a path reference from a string
- **`exists()`**: Checks if path exists in filesystem
- **`display()`**: Formats path for display (handles encoding)

## Key Concepts

### Path vs PathBuf
- **`Path`**: Borrowed path slice (like `&str`)
- **`PathBuf`**: Owned path (like `String`)

```rust
use std::path::{Path, PathBuf};

// Borrowed
let path: &Path = Path::new("/tmp/file.txt");

// Owned
let mut path_buf = PathBuf::from("/tmp");
path_buf.push("file.txt");
```

### Path Components
```rust
let path = Path::new("/home/user/file.txt");

// File name
println!("{:?}", path.file_name());  // Some("file.txt")

// Extension
println!("{:?}", path.extension());  // Some("txt")

// Parent directory
println!("{:?}", path.parent());  // Some("/home/user")

// Components
for component in path.components() {
    println!("{:?}", component);
}
```

### Path Construction
```rust
let mut path = PathBuf::from("/home");
path.push("user");
path.push("file.txt");
// /home/user/file.txt

// Join paths
let path = Path::new("/home").join("user").join("file.txt");
```

## Use Cases

- **File Location**: Build paths to files and directories
- **Path Validation**: Check if paths exist or are valid
- **Path Parsing**: Extract filename, extension, parent
- **Cross-Platform**: Write code that works on any OS
- **Configuration**: Handle user-provided paths safely

## Best Practices

✅ **Do:**
- Use `Path` and `PathBuf` instead of strings for paths
- Use `join()` to combine path components
- Handle relative vs absolute paths appropriately
- Use `canonicalize()` to resolve symlinks and `..`

❌ **Don't:**
- Manually concatenate paths with `/` or `\\`
- Assume path separators (use `std::path::MAIN_SEPARATOR`)
- Ignore path encoding issues (use `display()` or `to_string_lossy()`)

## Common Operations

### Check Path Type
```rust
use std::path::Path;

let path = Path::new("file.txt");

if path.is_file() {
    println!("It's a file");
}

if path.is_dir() {
    println!("It's a directory");
}

if path.is_symlink() {
    println!("It's a symbolic link");
}
```

### Path Manipulation
```rust
let path = PathBuf::from("/home/user/file.txt");

// Change extension
let new_path = path.with_extension("md");
// /home/user/file.md

// Change filename
let new_path = path.with_file_name("other.txt");
// /home/user/other.txt

// Remove filename
let mut path = path;
path.pop();  // Now: /home/user
```

### Canonicalize Paths
```rust
// Resolve . and .. and symlinks
let canonical = Path::new("../file.txt").canonicalize()?;
println!("Absolute path: {}", canonical.display());
```

## Common Patterns

### Build Output Path
```rust
let input = Path::new("input.txt");
let output = input.with_extension("out");
```

### Iterate Directory
```rust
use std::fs;

for entry in fs::read_dir(".")? {
    let entry = entry?;
    let path = entry.path();
    println!("{}", path.display());
}
```

### Safe Path Join
```rust
fn safe_join(base: &Path, user_path: &str) -> Option<PathBuf> {
    let joined = base.join(user_path);
    if joined.starts_with(base) {
        Some(joined)
    } else {
        None  // Prevent directory traversal
    }
}
```

## Related Features

- [File I/O](./file-io.md) - Reading and writing files
- [Environment Variables](./environment.md) - Getting system paths

## Learn More

- [std::path documentation](https://doc.rust-lang.org/std/path/)
- [The Rust Book - Paths](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html)
