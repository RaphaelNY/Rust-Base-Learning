use std::io::Error;
use std::fmt;

type Kilometers = i32;

fn type_alias() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
}

fn take_long_type(_f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // --snip--
    Box::new(|| println!("Hello, world!"))
}

fn long_type() {
    let f:Box<dyn Fn() + Send + 'static> = Box::new(|| println!("Hello, world!"));
    take_long_type(f);
    let f = returns_long_type();
    f();
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

fn bar() -> ! { // The never type
    panic!();
}

// DST
fn generic<T: ?Sized>(_t: &T) {
    // --snip--
}

// fn generic<T>(t: T) {}
// fn generic<T: Sized>(t: T) {}

// ?sized trait
// fn generic<T: ?sized>(t: &T) {}

// function pointer
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    println!("Hello, world!");
}
