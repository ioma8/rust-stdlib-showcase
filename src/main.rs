use std::thread;
use std::time::{Duration, Instant};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::env;
use std::process::Command;
use std::net::TcpListener;
use std::fmt;
use std::mem;
use std::ops::Add;
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

fn main() {
    println!("=== Rust Standard Library Showcase ===\n");

    // 1. Threading
    println!("1. Threading:");
    let handler = thread::Builder::new()
        .name("named thread".into())
        .spawn(|| {
            let handle = thread::current();
            println!("  Inner thread ID: {:?}, Name: {:?}", handle.id(), handle.name());
        })
        .unwrap();

    let main_handle = thread::current();
    println!("  Main thread ID: {:?}, Name: {:?}", main_handle.id(), main_handle.name());
    handler.join().unwrap();

    // 2. Time operations
    println!("\n2. Time operations:");
    let start = Instant::now();
    thread::sleep(Duration::from_millis(100));
    let duration = start.elapsed();
    println!("  Slept for {:?}", duration);

    // 3. Collections
    println!("\n3. Collections:");
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    println!("  HashMap: {:?}", map);

    let mut set = HashSet::new();
    set.insert("item1");
    set.insert("item2");
    println!("  HashSet: {:?}", set);

    // 4. File I/O
    println!("\n4. File I/O:");
    let file_path = "test_file.txt";
    {
        let mut file = File::create(file_path).expect("Unable to create file");
        file.write_all(b"Hello, Rust!").expect("Unable to write to file");
        println!("  Created file and wrote content");
    }

    let file = File::open(file_path).expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    buf_reader.read_line(&mut line).expect("Unable to read line");
    println!("  Read from file: {}", line.trim());
    std::fs::remove_file(file_path).expect("Unable to remove file");

    // 5. Path operations
    println!("\n5. Path operations:");
    let path = Path::new("example.txt");
    println!("  Path exists: {}", path.exists());
    println!("  Path: {}", path.display());

    // 6. Synchronization with Arc and Mutex
    println!("\n6. Synchronization:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("  Thread {} incremented counter to {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  Final counter value: {}", *counter.lock().unwrap());

    // 7. Environment variables
    println!("\n7. Environment:");
    for (key, value) in env::vars().take(3) {
        println!("  {}: {}", key, value);
    }

    // 8. Process execution
    println!("\n8. Process execution:");
    let output = Command::new("echo")
        .arg("Hello from subprocess")
        .output()
        .expect("Failed to execute command");
    
    println!("  Command output: {}", String::from_utf8_lossy(&output.stdout).trim());

    // 9. Error handling with Result
    println!("\n9. Error handling:");
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("  Success: {}", value),
        Err(e) => println!("  Error: {}", e),
    }

    // 10. Option type
    println!("\n10. Option type:");
    let some_value: Option<i32> = Some(5);
    let none_value: Option<i32> = None;
    
    println!("  Some value: {:?}", some_value);
    println!("  None value: {:?}", none_value);
    
    if let Some(val) = some_value {
        println!("  Unwrapped some value: {}", val);
    }

    // 11. Network operations (TCP)
    println!("\n11. Network operations:");
    // Simple TCP listener demonstration (will fail if port is in use, but shows the API)
    let listener_result = TcpListener::bind("127.0.0.1:0"); // Use random available port
    match listener_result {
        Ok(listener) => {
            let local_addr = listener.local_addr().unwrap();
            println!("  TCP listener bound to: {}", local_addr);
            // Don't actually accept connections to keep the demo simple
        }
        Err(e) => println!("  TCP bind failed: {}", e),
    }

    // 12. Iterators
    println!("\n12. Iterators:");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .filter(|x| x > &3)
        .collect();
    println!("  Original: {:?}", numbers);
    println!("  Doubled and filtered (>3): {:?}", doubled);

    // 13. Custom formatting with fmt
    println!("\n13. Custom formatting:");
    
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Point({}, {})", self.x, self.y)
        }
    }
    
    let point = Point { x: 10, y: 20 };
    println!("  Custom formatted point: {}", point);

    // 14. Memory operations
    println!("\n14. Memory operations:");
    let value = 42;
    let size = mem::size_of_val(&value);
    let align = mem::align_of_val(&value);
    println!("  Value: {}, Size: {} bytes, Alignment: {} bytes", value, size, align);
    
    let mut data = vec![1, 2, 3];
    let capacity_before = data.capacity();
    data.reserve(10);
    let capacity_after = data.capacity();
    println!("  Vector capacity before reserve: {}, after reserve: {}", capacity_before, capacity_after);

    // 15. Operator overloading
    println!("\n15. Operator overloading:");
    
    #[derive(Debug, Copy, Clone)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    
    impl Add for Complex {
        type Output = Self;
        
        fn add(self, other: Self) -> Self {
            Self {
                real: self.real + other.real,
                imag: self.imag + other.imag,
            }
        }
    }
    
    let c1 = Complex { real: 1.0, imag: 2.0 };
    let c2 = Complex { real: 3.0, imag: 4.0 };
    let c3 = c1 + c2;
    println!("  Complex addition: {:?} + {:?} = {:?}", c1, c2, c3);

    // 16. Pin and PhantomPinned (self-referential structs)
    println!("\n16. Pin and PhantomPinned:");
    
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
    println!("  Pinned data: {}", pinned.get_data());

    // 17. Future and async basics
    println!("\n17. Future and async basics:");
    
    struct SimpleFuture {
        value: i32,
    }
    
    impl Future for SimpleFuture {
        type Output = i32;
        
        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(self.value)
        }
    }
    
    let future = SimpleFuture { value: 42 };
    let mut boxed_future = Box::pin(future);
    
    // Create a dummy waker for polling
    struct NoOpWaker;
    impl std::task::Wake for NoOpWaker {
        fn wake(self: std::sync::Arc<Self>) {}
    }
    let waker: Waker = std::sync::Arc::new(NoOpWaker).into();
    let mut cx = Context::from_waker(&waker);
    
    match boxed_future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => println!("  Future resolved with value: {}", value),
        Poll::Pending => println!("  Future is pending"),
    }

    // 18. Rc and RefCell (interior mutability)
    println!("\n18. Rc and RefCell:");
    
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
    
    // Clone the Rc to show shared ownership
    let shared_clone = Rc::clone(&shared_data);
    
    // Modify through RefCell
    shared_data.borrow_mut().push(4);
    
    println!("  Original Rc count: {}", Rc::strong_count(&shared_data));
    println!("  Shared data: {:?}", shared_data.borrow());
    println!("  Clone also sees: {:?}", shared_clone.borrow());

    // 19. Any type for dynamic typing
    println!("\n19. Any type for dynamic typing:");
    
    let string_value: &dyn Any = &"Hello, Any!".to_string();
    let int_value: &dyn Any = &42i32;
    
    if let Some(s) = string_value.downcast_ref::<String>() {
        println!("  String value: {}", s);
    }
    
    if let Some(i) = int_value.downcast_ref::<i32>() {
        println!("  Integer value: {}", i);
    }

    // 20. Panic handling and recovery
    println!("\n20. Panic handling and recovery:");
    
    let result = std::panic::catch_unwind(|| {
        println!("  About to panic...");
        panic!("This is a controlled panic!");
    });
    
    match result {
        Ok(_) => println!("  No panic occurred"),
        Err(e) => {
            if let Some(s) = e.downcast_ref::<&str>() {
                println!("  Caught panic: {}", s);
            } else {
                println!("  Caught unknown panic");
            }
        }
    }

    println!("\n=== Complete 20-Feature Showcase Complete ===");
}
