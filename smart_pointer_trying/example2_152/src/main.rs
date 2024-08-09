use std::ops::Deref;

fn hello(name: &str) {
	println!("Hello, {}!", name);
}

#[allow(unused)]
fn main() {
	let m = MyBox::new(String::from("Rust"));
	let x = &"Rust";
	let y = &(*m)[..];
	let z = &m[..];
	hello(&m);
	hello(&(*m)[..]);
	hello(&m[..]);
	// &m &MyBox<String>
	// deref &String
	// &m == *(m.deref())
	// m.deref() returns &"Rust": &&str
	// *(m.deref()) returns "Rust": &str
}

struct MyBox<T>(T);
impl <T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}
impl <T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}
