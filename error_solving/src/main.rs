use std::fs::File;
use std::io::ErrorKind;
use::std::io;
use::std::io::Read;
use::std::convert::From; // used in transport of errors
/// # error solving
 /// part one:
 ///     - avilable to recover:
 ///          - such as file cannot found, we can try it again.
 ///     - unavilable to recover:
 ///          - such as out of memory
 ///          - such as out of index
 /// part two:
 ///     - result<T, E>

fn main() {
     // unavilable to recover error && panic!
     // when panic! is called, your program will print a failure message, unwind and clean up the stack, and then quit.
     // you can set RUST_BACKTRACE=1 to see the backtrace
     // you can show or abort the stack using.
     // in normal mode, when panic appear, hte program will show using stack, rust will walk back by the call
     //     stack and clean up the data from each function it encounters.
     // or you can stop the program immediately without cleaning up. the memory will cleaning up by OS.
     // to make the file size smaller, you can set from show to abort.
     //    in cargo.toml, add the following code:
        //    [profile.release]
        //    panic = 'abort'
     // panic!("crash and burn");
    let vec = vec![1, 2, 3];
     // v[99]; panic: index out of bounds: the len is 3 but the index is 99
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // panic can appear in two ways: our code or the library code.
     // we can call the panic! function to know where is the wrong code.
     // you can set the RUST_BACKTRACE=1 to see the backtrace.
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // error function: result<T, E>{
         //     Ok(T),
         //     Err(E),
         // }
     // if succeed, return OK(T), if failed, return Err(E)
      // let f = File::open("hello.txt");
      // match f {
       //    Ok(file) => file,
      //     Err(error) => panic!("Problem opening the file: {:?}", error),
      // };
      // match f {
      //     Ok(file) => file,
      //     Err(error) => match error.kind() {
      //         ErrorKind::NotFound => match File::create("hello.txt") {
      //             Ok(fc) => fc,
      //             Err(e) => panic!("Problem creating the file: {:?}", e),
      //         },
      //         other_error => panic!("Problem opening the file: {:?}", other_error),
      //     },
      // };
      // match f {
      //    Ok(file) => file,
      //    Err(error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
      //         Ok(fc) => fc,
      //         Err(e) => panic!("Problem creating the file: {:?}", e),
      //     },
      //     Err(error) => panic!("Problem opening the file: {:?}", error),
      // };
     // f.unwrap_or_else(|error| match error.kind() {
     //    ErrorKind::NotFound => match File::create("hello1.txt") {
     //        Ok(fc) => fc,
     //        Err(e) => panic!("Problem creating the file: {:?}", e),
     //    },
     //     other_error => panic!("Problem opening the file: {:?}", other_error),
     // });
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // closure
     // Result<T, E> has many methods, they take closure as varies and use match to do.
     // let f = File::open("hello2.txt").unwrap_or_else(|error| {
     //     if error.kind() == ErrorKind::NotFound {
     //         File::create("hello.txt").unwrap_or_else(|error| {
     //            panic!("Problem creating the file: {:?}", error);
     //         })
     //     } else {
     //        panic!("Problem opening the file: {:?}", error);
     //     }
     // });
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // unwrap and expect
     // unwrap: if the result is Ok, return the value, if the result is Err, call panic!
     // expect: same as unwrap, but you can set the panic! message.
     // let f = File::open("hello.txt").unwrap();
     // let f = File::open("hello.txt").expect("Failed to open hello.txt");
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let f = File::open("hello2.txt").expect("😭couldn't open file: hello3.txt"); // .expect can init the panic! message.
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // transport error
    let result = read_username_from_file1();
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // ? operator
    let result = read_username_from_file2();
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // the ? operator can only be used in function that return Result<T, E>, ? and from

}

fn read_username_from_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? operator will return the error to the caller.
     // if f returned is not io::Error,we can use from tans the error to io::Error.
     // let mut f = match File::open("hello.txt") {
     //    Ok(file) => file,
     //    Err(e) => return Err(e),
     // };
    let mut s = String::new();
    f.read_to_string(&mut s)?; // ? operator will return the error to the caller.
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; // ? operator will return the error to the caller.
    Ok(s)
}
/*
    fn main() -> Result<(), Box<dyn Error>> {
    fs::read_to_string("hello.txt")?;
    Ok(())
}
    you can think that Box<dyn Error> can be any one type of error.
*/

/// 9.4 when we need to use panic!
///    - when we have more information than the compiler does.
///    - if we are design a might fail function, we always used panic!,return a Result.
