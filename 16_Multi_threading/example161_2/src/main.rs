use std::thread;

fn main() {
	let v = vec![1,2,3];
	let handle = thread::spawn(move || {
		println!("Here's a vector: {:?}", v);
	});

	// drop(v); // Oh no! We dropped v too soon! (This will cause a compile error.

	handle.join().unwrap();
}
