static HELLOW_WORLD: &str = "Hello, world!";

fn main() {
    println!("{}", HELLOW_WORLD);
}

#[no_mangle]
pub extern "C" fn call_from_C() {
    println!("Just called a Rust function from C!");
}
