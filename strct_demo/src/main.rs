
#[derive(Debug)] // this is a annotate
struct Rectangle {
    width: u32,
    height: u32,
}impl Rectangle {
    fn rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    } // Constructor Function

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    } // Correlation Function
} // more than one impl block for a struct is allowed

fn main() {
    let width1 = 20;
    let height1 = 30;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    let dim = (20, 30);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(dim)
    );

    let ect1 = Rectangle { width: 20, height: 30 };
    let ect2 = Rectangle::rectangle(20, 30);
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&ect1) // this is a borrowing of ect1
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        ect2.area()
    );

    println!("ect1 is {:?} ", ect1);
    println!("ect1 is {:#?}", ect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}