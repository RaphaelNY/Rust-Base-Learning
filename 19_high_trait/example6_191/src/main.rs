unsafe trait Foo {
    fn foo(&self);
}
unsafe impl Foo for i32 {
    fn foo(&self) {
        println!("foo");
    }
}

fn main() {
    println!("Hello, world!");
}
