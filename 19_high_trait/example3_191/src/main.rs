fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}
