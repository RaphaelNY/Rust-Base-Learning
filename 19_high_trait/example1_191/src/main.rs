fn main() {
    let mut num =  5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("r is: {}", *r); // <- error: process didn't exit successfully: `target\debug\example1_191.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
    }
}
