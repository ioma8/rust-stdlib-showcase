# File I/O

## Overview

Rust's `std::fs` and `std::io` modules provide safe, cross-platform file operations. Create, read, write, and manage files with explicit error handling.

## Code Example

```rust
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

// Write to file
let file_path = "test_file.txt";
{
    let mut file = File::create(file_path)
        .expect("Unable to create file");
    file.write_all(b"Hello, Rust!")
        .expect("Unable to write to file");
}

// Read from file
let file = File::open(file_path)
    .expect("Unable to open file");
let mut buf_reader = BufReader::new(file);
let mut line = String::new();
buf_reader.read_line(&mut line)
    .expect("Unable to read line");
println!("Read from file: {}", line.trim());

// Clean up
std::fs::remove_file(file_path)
    .expect("Unable to remove file");
```

## Explanation

- **`File::create()`**: Creates a new file (truncates if exists)
- **`File::open()`**: Opens existing file for reading
- **`write_all()`**: Writes entire byte buffer to file
- **`BufReader`**: Buffered reading for better performance
- **`read_line()`**: Reads until newline character

## Key Concepts

### File Opening Modes
```rust
use std::fs::OpenOptions;

// Read-only
let file = File::open("file.txt")?;

// Write (create or truncate)
let file = File::create("file.txt")?;

// Append
let file = OpenOptions::new()
    .append(true)
    .open("file.txt")?;

// Read and write
let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("file.txt")?;
```

### Buffered I/O
Always use buffered readers/writers for better performance:
```rust
use std::io::{BufReader, BufWriter};

// Buffered reading
let reader = BufReader::new(File::open("input.txt")?);

// Buffered writing
let writer = BufWriter::new(File::create("output.txt")?);
```

### Error Handling
File operations return `Result<T, std::io::Error>`:
```rust
match File::open("file.txt") {
    Ok(file) => { /* use file */ },
    Err(e) => eprintln!("Error: {}", e),
}
```

## Use Cases

- **Configuration**: Read/write config files
- **Logging**: Append to log files
- **Data Processing**: Read input, write output
- **Caching**: Store computed results to disk
- **Reports**: Generate text/CSV reports

## Best Practices

✅ **Do:**
- Use `BufReader`/`BufWriter` for line-by-line or buffered I/O
- Handle errors explicitly with `?` or `match`
- Use scopes `{}` to ensure files close promptly
- Use `std::fs` convenience functions when appropriate

❌ **Don't:**
- Forget to handle file errors (use `?` or `.expect()`)
- Read entire large files into memory at once
- Keep files open longer than necessary
- Assume file exists without checking

## Convenience Functions

```rust
use std::fs;

// Read entire file to string
let contents = fs::read_to_string("file.txt")?;

// Read entire file to bytes
let bytes = fs::read("file.bin")?;

// Write string to file
fs::write("file.txt", "content")?;

// Copy file
fs::copy("src.txt", "dst.txt")?;

// Remove file
fs::remove_file("file.txt")?;
```

## Common Patterns

### Read File Line by Line
```rust
use std::io::BufRead;

let file = File::open("file.txt")?;
let reader = BufReader::new(file);

for line in reader.lines() {
    let line = line?;
    println!("{}", line);
}
```

### Write Multiple Lines
```rust
let mut file = File::create("output.txt")?;
for line in &lines {
    writeln!(file, "{}", line)?;
}
```

## Related Features

- [Path Operations](./path.md) - Path manipulation and queries
- [Error Handling](./error-handling.md) - Handling I/O errors

## Learn More

- [std::fs documentation](https://doc.rust-lang.org/std/fs/)
- [std::io documentation](https://doc.rust-lang.org/std/io/)
