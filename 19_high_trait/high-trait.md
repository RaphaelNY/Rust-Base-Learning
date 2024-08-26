# high-trait

- unsafe rust
- high-level Trait
- high-level type
- high-level function && closure
- macro

## 19.1 unsafe rust

### Match named variables

- Hidden Second Language. It is not subject to a mandatory memory security guarantee.
  - Same as regular Rust, but endowed with additional 'superpowers'
- Reasons for the existence of unsafe rust
  - static analysis is not perfect. use unsafe rust is telling yourself what you are doing,and take on corresponding risks.
- Computer hardware itself is not secure, Rust needs to be able to perform underlying system compilation.

### Unsafe rust superpowers

- use `unsafe` to define functions and methods, switch to *unsafe Rust*, it wil open a block, inside is the usafe code.
- There are four actions in *Unsafe Rust* can be performed:
  - Dereference a raw pointer
  - Call an unsafe function or method
  - Access or modify a mutable static variable
  - Implement an unsafe trait
- pay attention:
  - unsafe rust didn't close the borrow check or stop to use other safety checks.
  - Any memory security related errors must be left in the unsafe block
  - Isolate unsafe code as much as possible, preferably encapsulate it in a secure abstraction, and provide a secure API

### Dereference a raw pointer

- raw pointer: 
  - mutable raw pointer: `*mut T`
  - immutable raw pointer: `*const T`, means when Pointer be dereferenced, the value can't be changed directly.
  - attention: this `*` is different from dereference, it is a part of type.
- difference between raw pointer and reference:
  - allowed to keep the mutable and immutable pointer or many pointers to the same memory location's mutable pointer to ignore borrow rules.
  - cannot confirm that it will pointer to a valid memory location.
  - allowed to be `null` or invalid memory address.
  - didn't implement `Deref` or `Drop` trait.
- ***example1***
- why dereference raw pointer:
  - to call C code
  - to build some safe abstraction that checker can't understand.

### Call an unsafe function or method

- unsafe function: 
  - `unsafe fn function_name() {}`
  - unsafe block: `unsafe {}`
- ***example1 --lib***

### create a safe abstraction of unsafe code

- the function includes unsafe code, but it doesn't need to be marked as unsafe.
- package the unsafe code in safe function is a common abstraction.
- ***example2***

### use `extern` to call external code

- `extern` keyword: Simplify the process of creating and using External Function Interfaces (FFIs)
- FFI: It allows one programming language to define functions and enables other programming languages to call these functions
- ***example3***
- ABI(application binary interface): define the using rule of function in the binary file.

### call rust function from other languages

- you can use `extern` to create Interface that other languages can call.
- add `extern` before `fn` and ordered `ABI`
- you need add `#[no_mangle]` to the function, to prevent the function name from being changed by the compiler.
- ***example4***

### Access or modify a mutable static variable

- rust supports global variables, but because of some questions of ownership, data competition, etc.
- in rust, global variables are called static variables, and they are immutable by default.
- ***example4***

#### static variable

- static variable: `static NAME: TYPE = VALUE;`
- static variable's lifetime: the entire life of the program
- static variable's memory location: fixed memory location
- name: SCREAMING_SNAKE_CASE
- static variables can just use `'static` lifetime borrow. 
- visit immutable static variable is safe.

#### difference between static variable and constant

- constant: `const NAME: TYPE = VALUE;`
- constant: can be used in the same way as static variables, but the value of constant is fixed at compile time.
- constant allowed copy.
- static variable can be mutable, but visit and change it is *unsafe*. 
- ***example5***

### Implement an unsafe trait

- when a trait is unsafe, it means that the trait has some requirements that the compiler can't verify.
- use unsafe to implement an unsafe trait.
  - this trait can just be implemented in unsafe code.
- ***example6***

#### when using unsafe code

- compiler cannot guarantee the safety of the code, so you need to ensure that the code is safe.
- when you have sufficient reasons, you can use unsafe code.
- marked by `unsafe` keyword, you can find it easily.
