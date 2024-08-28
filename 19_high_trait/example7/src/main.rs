pub trait Animal {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Animal2<T> {
    fn next(&mut self) -> Option<T>;
}


struct Counter {}

impl Animal for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
} // this type Counter can only implement Animal trait once.
impl Animal2<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        todo!()
    }
}
impl Animal2<f32> for Counter {
    fn next(&mut self) -> Option<f32> {
        todo!()
    }
}




fn main() {
    println!("Hello, world!");
}
