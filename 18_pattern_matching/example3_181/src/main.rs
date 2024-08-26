#[allow(unused)]
fn main() {
    {
        let a = 10;

        let (x, u, z) = (1, 2, 3);

        // let (q, w) = (1, 2, 3); <- error: mismatched types
    }

    {
        let point = (3, 5);
        print_coordinates(&point);
    }
}

#[allow(unused)]
fn foo(_x: i32) {
    // code
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}