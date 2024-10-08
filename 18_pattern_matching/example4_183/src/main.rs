#[allow(unused)]
fn main() {
	// match statement
	{
		let x = 1;

		match x {
			1 => println!("one"),
			2 => println!("two"),
			3 => println!("three"),
			_ => println!("anything"),
		}
	}

	//
	{
		let x = Some(5);
		let y = 10;

		match x {
			Some(50) => println!("Got 50"),
			Some(y) => println!("Matched, y = {:?}", y), // <- this y istn't the same as the outer y
			_ => println!("Default case, x = {:?}", x),
		}
		println!("at the end: x = {:?}, y = {:?}", x, y);
	}

	// multiple patterns
	{
		let x = 1;

		match x {
			1 | 2 => println!("one or two"),
			3 => println!("three"),
			_ => println!("anything"),
		}
	}

	// range patterns
	{
		let x = 5;

		match x {
			1..=5 => println!("one through five"),
			_ => println!("something"),
		}

		let x = 'c';
		match x {
			'a'..='j' => println!("early ASCII letter"),
			'k'..='z' => println!("late ASCII letter"),
			_ => println!("something"),
		}
	}

	// destructuring structs
	{
		struct Point {
			x: i32,
			y: i32,
		}

		let p = Point { x: 0, y: 7 };

		let Point { x: a, y: b } = p;
		assert_eq!(0, a);
		assert_eq!(7, b);

		match p {
			Point { x, y: 0 } => println!("On the x axis at {}", x),
			Point { x: 0, y } => println!("On the y axis at {}", y),
			Point { x, y } => println!("On neither axis at ({}, {})", x, y),
		}
	}

	// destructuring enums
	{
		enum Message {
			Quit,
			Move { x: i32, y: i32 },
			Write(String),
			ChangeColor(i32, i32, i32),
		}

		let msg = Message::ChangeColor(0, 160, 255);

		match msg {
			Message::Quit => {
				println!("The Quit variant has no data to destructure.")
			}
			Message::Move { x, y } => {
				println!(
					"Move in the x direction {} and in the y direction {}",
					x, y
				);
			}
			Message::Write(text) => println!("Text message: {}", text),
			Message::ChangeColor(r, g, b) => {
				println!(
					"Change the color to red {}, green {}, and blue {}",
					r, g, b
				)
			}
		}
	}

	// destructuring nested structs and enums
	{
		enum Color {
			Rgb(i32, i32, i32),
			Hsv(i32, i32, i32),
		}

		enum Message {
			Quit,
			Move { x: i32, y: i32 },
			Write(String),
			ChangeColor(Color),
		}

		let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

		match msg {
			Message::ChangeColor(Color::Rgb(r, g, b)) => {
				println!(
					"Change the color to red {}, green {}, and blue {}",
					r, g, b
				)
			}
			Message::ChangeColor(Color::Hsv(h, s, v)) => {
				println!(
					"Change the color to hue {}, saturation {}, and value {}",
					h, s, v
				)
			}
			_ => ()
		}
	}

	// destructuring structs and tuples
	{
		struct Point {
			x: i32,
			y: i32,
		}

		let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

		println!("feet = {}, inches = {}, x = {}, y = {}", feet, inches, x, y);
	}

	// ignoring values in a pattern
	{ // _
		fn foo(_: i32, y: i32) {
			println!("This code only uses the y parameter: {}", y);
		}

		foo(3, 4);
	}
	{
		let mut setting_value = Some(5);
		let new_setting_value = Some(10);

		match (setting_value, new_setting_value) {
			(Some(_), Some(_)) => {
				println!("Can't overwrite an existing customized value");
			}
			_=> {
				setting_value = new_setting_value;
			}
		}
		println!("setting is {:?}", setting_value);
	}
	{
		let numbers = (1, 2, 3, 4, 5);
		match numbers {
			(first, _, third, _, fifth) => {
				println!("Some numbers: {}, {}, and {}", first, third, fifth);
			}
		}
	}

	// use '_' to ignore an entire value
	{
		let _x = 5;
		let y = 10; // <- this y will warn because it's unused

		let s = Some(String::from("Hello!"));

		if let Some(_s) = s { // <- s is moved here by "_s"
			println!("found  s.");
		}

		// println!("{:?}", s); <- this will cause an error.
	}

	// use '..' to ignore remaining parts of a value
	{
		struct Point {
			x: i32,
			y: i32,
			z: i32,
		}

		let origin = Point { x: 0, y: 0, z: 0 };

		match origin {
			Point { x, .. } => println!("x is {}", x),
		}
	}
	{
		let numbers = (2, 4, 8, 16, 32);

		match numbers {
			(first, .., last) => {
				println!("Some numbers: {}, {}", first, last);
			}
		}
	}
	{
		let numbers = (2, 4, 8, 16, 32);

		match numbers {
			(_, second, ..) => {
				println!("Some numbers Second 2: {}", second);
			}
		}
		match numbers {
			(.., fourth, _) => {
				println!("Some numbers Second 4: {}", fourth);
			}
		}
	}

	// use match guard to add condition to a match arm
	{
		let num = Some(4);

		match num {
			Some(x) if x < 5 => println!("less than five: {}", x),
			Some(x) => println!("{}", x),
			None => (),
		}
	}
	{
		let x = Some(5);
		let y = 10;

		match x {
			Some(50) => println!("Got 50"),
			Some(n) if n == y => println!("Matched, n = {}", n),
			_ => println!("Default case, x = {:?}", x),
		}

		println!("at the end: x = {:?}, y = {}", x, y);
	}
	{
		let x = 4;
		let y = false;

		match x {
			4 | 5 | 6 if y => println!("yes"),
			_ => println!("no"),
		}
	}

	// @ bindings
	{
		enum Message {
			Hello { id: i32 },
		}

		let msg = Message::Hello { id: 5 };

		match msg {
			Message::Hello { id: id_variable @ 3..=7 } => {
				println!("Found an id in range: {}", id_variable);
			}
			Message::Hello { id: 10..=12 } => {
				println!("Found an id in another range");
			}
			Message::Hello { id } => {
				println!("Found some other id: {}", id);
			}
		}
	}
}
