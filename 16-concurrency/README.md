# General
- *Concurrent programming* : Different parts of a program execute independently.
- *Parallel programming* :  Different parts of a program execute at the same time.
- An executed program’s code is run in a *process*. One can also have independent parts that run simultaneously. The features that run these independent parts are called *threads*.
- Problems: Race conditions, deadlocks, hard-to-reproduce bugs

# Threads
- The model where a language calls the operating system APIs to create threads is sometimes called *1:1*, meaning one operating system thread per one language thread. The Rust standard library only provides an implementation of 1:1 threading.
- To create a new thread, call the `thread::spawn` function and pass it a closure.
- A slogan from the Go Language docs: "Do not communicate by sharing memory; instead share memory by communicating."
- One major tool Rust has for accomplishing message-sending concurrency is the *channel*. A channel in programming has two halves: a transmitter and a receiver.

# Channels
- Use `mpsc::channel` from `std::sync` to create a new channel; `mpsc` stands for *multiple producer, single consumer*.
- The `send` function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it.

# Shared-State
- *Mutex* is an abbreviation for *mutual exclusion*, as in, a mutex allows only one thread to access some data at any given time.
- To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s *lock*.
- Motivation for `Arc<T>`:
	- `` `Rc<Mutex<i32>>` cannot be sent between threads safely``
	- `` the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``
- *A* in `Arc<T>` stands for *atomic*, meaning it's an *atomically reference counted* type.
- `Mutex<T>` provides interior mutability, as the `Cell` family does.
- `Mutex<T>` comes with the risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.

# Traits
- Two concurrency concepts are embedded in the language: the `std::marker` traits `Sync` and `Send`.
- The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads.
- The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads.
- Any type `T` is `Sync` if `&T` (an immutable reference to `T`) is `Send`.
- Almost all primitive types are `Send` and `Sync`, and type composition carries these traits.
