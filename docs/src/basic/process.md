# Process Execution

## Overview

The `std::process` module enables spawning and interacting with child processes. Run external commands, capture output, and manage subprocesses safely.

## Code Example

```rust
use std::process::Command;

let output = Command::new("echo")
    .arg("Hello from subprocess")
    .output()
    .expect("Failed to execute command");

println!("Command output: {}", 
         String::from_utf8_lossy(&output.stdout).trim());
```

## Explanation

- **`Command::new()`**: Creates a new command to execute
- **`arg()`**: Adds command-line argument
- **`output()`**: Runs command and waits, capturing output
- **`String::from_utf8_lossy()`**: Converts bytes to string safely

## Key Concepts

### Building Commands
```rust
use std::process::Command;

let output = Command::new("ls")
    .arg("-l")
    .arg("-a")
    .current_dir("/tmp")
    .env("MY_VAR", "value")
    .output()?;
```

### Execution Methods
```rust
// Capture output (wait for completion)
let output = Command::new("echo").arg("test").output()?;

// Wait for exit status only
let status = Command::new("sleep").arg("1").status()?;

// Stream output (inherit stdio)
let status = Command::new("cargo")
    .arg("build")
    .status()?;

// Full control with spawn
let mut child = Command::new("long-running")
    .spawn()?;
let status = child.wait()?;
```

### Output Structure
```rust
pub struct Output {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

// Check success
if output.status.success() {
    println!("Command succeeded");
}

// Get exit code
if let Some(code) = output.status.code() {
    println!("Exit code: {}", code);
}
```

## Use Cases

- **Build Tools**: Run compilers, bundlers
- **System Administration**: Execute system commands
- **Testing**: Run test executables, check behavior
- **Pipeline Processing**: Chain commands together
- **Integration**: Interface with external tools

## Best Practices

✅ **Do:**
- Check exit status before using output
- Use `.status()` when output not needed (more efficient)
- Properly escape/validate user input in commands
- Handle both stdout and stderr

❌ **Don't:**
- Ignore command errors
- Use shell interpolation with untrusted input (security risk)
- Assume command exists on all platforms
- Forget to read/consume child process output (can deadlock)

## Advanced Usage

### Piping Between Commands
```rust
use std::process::{Command, Stdio};

let ps_child = Command::new("ps")
    .arg("aux")
    .stdout(Stdio::piped())
    .spawn()?;

let grep_output = Command::new("grep")
    .arg("rust")
    .stdin(ps_child.stdout.unwrap())
    .output()?;
```

### Streaming Output
```rust
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

let mut child = Command::new("cargo")
    .arg("build")
    .stdout(Stdio::piped())
    .spawn()?;

if let Some(stdout) = child.stdout.take() {
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("Build: {}", line?);
    }
}

child.wait()?;
```

### Timeout Implementation
```rust
use std::time::Duration;
use std::thread;

let mut child = Command::new("long-process").spawn()?;

let timeout = Duration::from_secs(5);
thread::sleep(timeout);

// Kill if still running
match child.try_wait()? {
    Some(status) => println!("Exited with: {}", status),
    None => {
        child.kill()?;
        println!("Process killed (timeout)");
    }
}
```

## Platform Considerations

### Unix-Specific
```rust
#[cfg(unix)]
use std::os::unix::process::CommandExt;

#[cfg(unix)]
let command = Command::new("ls")
    .uid(1000)  // Set user ID
    .gid(1000); // Set group ID
```

### Windows-Specific
```rust
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
let command = Command::new("cmd")
    .creation_flags(0x08000000); // CREATE_NO_WINDOW
```

## Common Patterns

### Check Tool Availability
```rust
fn has_command(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
```

### Run with Error Context
```rust
fn run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", cmd, e))?;
    
    if !output.status.success() {
        return Err(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

## Security Considerations

⚠️ **Never** construct commands with untrusted input:
```rust
// BAD - shell injection vulnerability
Command::new("sh")
    .arg("-c")
    .arg(format!("ls {}", user_input));

// GOOD - arguments properly escaped
Command::new("ls")
    .arg(user_input);
```

## Related Features

- [Environment Variables](./environment.md) - Setting process environment
- [Error Handling](./error-handling.md) - Handling command errors

## Learn More

- [std::process documentation](https://doc.rust-lang.org/std/process/)
