fn main() {
    println!("Hello, world!");
}

fn spinning_rings (inner_max: u8, outer_max: u8) -> u8 {
    let mut inner = 0;
    let mut outer = 0;
    let mut x = 0;
    loop {
        x += 1;
        inner = (inner + inner_max) % (inner_max + 1);
        outer = (outer + 1) % (outer_max + 1);

        if inner == outer {
            break;
        }
    }
    x
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::spinning_rings;

    #[test]
    fn fixed_tests() {
        assert_eq!(spinning_rings(2, 3), 5);
        assert_eq!(spinning_rings(3, 2), 2);
        assert_eq!(spinning_rings(1, 1), 1);
        assert_eq!(spinning_rings(2, 2), 3);
        assert_eq!(spinning_rings(3, 3), 2);
    }
}