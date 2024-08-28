use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}
// impl Add for Point {
// 	type Output = Point;

// 	fn add(self, rhs: Self) -> Self::Output {
// 		Point {
// 			x: self.x + rhs.x,
// 			y: self.y + rhs.y,
// 		}
// 	}
//}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
	type Output = Millimeters;

	fn add(self, rhs: Meters) -> Self::Output {
		Millimeters(self.0 + rhs.0 * 1000)
	}
}

fn main() {
	// assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
	assert_eq!(Millimeters(1000) + Meters(1), Millimeters(2000));
}
