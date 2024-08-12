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