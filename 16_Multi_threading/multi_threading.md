# multi_threading

## Introduction

- Concurrent: different parts can run independently
- Parallel: different parts can run at the same time

## 16.1 

### thread and process

- code run in process, managed by OS
- in your programs, independent part of code run in thread

### cause of multi-threading

- *Competitive state*: Threads accessing data or resources in an inconsistent order
- *Deadlock*: Two threads are waiting for each other to use up the resources they hold, and the threads cannot continue
- *Bugs* that only occur in certain situations are difficult to reliably replicate and fix

### how to use thread

- use OS api to create thread: 1:1 model
  - when need small runtime
- use language api to create thread: M:N model
  - when need large runtime
- Rust needs balance runtime support
- in Rust std, only support 1:1 model

### thread in Rust

- use `std::thread::spawn` to create thread
  - `spawn` return a `JoinHandle`
  - `spawn` need a closure (code worked in new thread)
  - ***example 1***
- use `join handle` to wait thread finish
  - `JoiinHandle` keep the ownership
    - use join method can wait another thread finish
  - `join` return a `Result`

### move closure

- `move closure` always used with `thread::spawn`, it allowed you to use data of another thread.
- when create thread, the ownership of value will be transferred to new thread
- ***example 2***

## 16.2

### thread communication

- message passing: send Message between threads
- thread (Actor) model: send Message to Actor, Actor will process a message
- Rust: Channel(support by std)

### channel

- channel include two parts: `Sender` and `Receiver`
- call `Sender` to send messages
- call `Receiver` will check and receive messages arrived
- if `Sender` or `Receiver` drop, the channel will be closed

#### how to create Channel

- use `std::sync::mpsc::channel` to create Channel
  - `mspc` is multi-producer, single-consumer
  - `channel` return a tuple, include `Sender` and `Receiver`
  - ***example 3***

#### send Message

- receive messages you want to send
- return: `Result<T,E>`
  - if `Receiver` drop, return `Err`

#### receive Message

- `recv()`: it will Block the current thread, until a message is received from Channel.
  - if receive Message, return `Result<T,E>`
  - if `Sender` drop, return `Err`
- `try_recv()`: it will not block the current thread
  - immediately return `Result<T,E>`
    - if receive Message, return `Ok()`, with the Message
    - if no Message, return `Err`
  - you can use `loop` to check the `try_recv` 's result.
### Channel and ownership transfer

- ***example 4***

### send more than one Message

- ***example 5***

### create more than one Receiver by clone

- ***example 6***

## 16.3

### Using sharing to achieve concurrency

- `Channel` likes single-ownership, once ownership gives ot Channel, it will be dropped.
- Shared memory concurrency is similar to multi ownership: multiple threads can access the same block of memory simultaneously

### Mutex
> use Mutex to protect shared data,it allowed only one thread to access the data at the same time.

- `Mutex`: mutual exclusion
- in one time, only one thread can access the data
- if you wan to access the data: 
  - thread must have mutex(lock)
    - lock datastructures is one part of mutex. it can follow who has the own visit power.

#### two rules of Mutex

- before get data, you must try to get lock.
- after using mutex locked data, you must release lock that other thread can access the data.

#### Mutex<T> 's API

- `Mutex::new()`: create a new `Mutex<T>`
  - `Mutex<T>` is a smart pointer
- before access data, used `lock()` to get lock.
  - it will block the current thread until get lock
  - lock may fail, return `Result<T,E>`(`Err`)
  - `lock()` return a `MutexGuard` (smart pointer, implement `Deref` && `Drop`)
- ***example 7***

#### Multiple threads share `Mutex<T>` && Multi threaded multiple ownership

- ***example 8***

#### Using Arc<T>for atomic reference counting

- `Arc<T>` used in multiple threads, `Rc<T>` used in single thread
  - A: atomic
  - `Arc<T>` need more cost than `Rc<T>`, because it need to ensure atomicity
- `Arc<T>` 's API is the same to `Rc<T>`
- ***example 8***

### `RefCell<T>` and `Rc<T>` vs `Mutex<T>` and `Arc<T>`

- `Mutex<T>` support mutable inside just like Cell.
- we used `RefCell<T>` to change the value in `Rc<T>`
- we used `Mutex<T>` to change the value in `Arc<T>`
- warning: `Mutex<T>` have dead lock risk.

## 16.4

### Send && Sync Trait

> `std::marker::Sync` and `std::marker::Send` are two special traits in Rust, they are used to ensure that the data can be safely shared between threads.

### Send: allow ownership transfer between threads

- implement `Send` can transfer ownership between threads
- almost all Rust types implement `Send`
  - `Rc<T>` and `RefCell<T>` have not `Send` Trait, they only allowed in single thread
- Any type consisting entirely of the `Send` type is also marked as `Send`
- Except for the original pointer, almost all basic types are `Send`

### Sync: allow shared references between threads

- type implement `Sync` can safely share references between threads
  - just like: if `T` is `Sync`, that `&T` is `Send`
- all basic types are `Sync`
- consisting entirely of the `Sync` type is also marked as `Sync`
  - but `Rc<T>` is not `Sync`
  - `RefCell<T>` and `Cell<T>` family are not `Sync`
  - `Mutex<T>` and `Arc<T>` are `Sync`
  - Manually implementing `Send` and `Sync` is not secure, you must design your own code to ensure their security