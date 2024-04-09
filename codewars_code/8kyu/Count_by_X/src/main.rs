fn main() {
    println!("Count by x!");
}

fn count_by(x: u32, n: u32) -> Vec<u32> {
    vec![x; n as usize].iter().enumerate().map(|(i, _)| (i + 1) as u32 * x).collect::<Vec<u32>>()
    // (1..=n).map(|e| x*e).collect() // Alternative solution
}

fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
match operator {
        '+' => value1 + value2,
        '-' => value1 - value2,
        '*' => value1 * value2,
        '/' => value1 / value2,
        _ => panic!("Invalid operator")
    }
}

#[cfg(test)]
mod tests {
    use super::count_by;
    use itertools::Itertools;

    #[test]
    fn sample_tests() {
        assertion(vec![1,2,3,4,5,6,7,8,9,10], (1, 10));
        assertion(vec![2,4,6,8,10], (2, 5));
        assertion(vec![3,6,9,12,15,18,21], (3, 7));
        assertion(vec![50,100,150,200,250], (50, 5));
        assertion(vec![100,200,300,400,500,600], (100, 6));
    }

    fn assertion(expected : Vec<u32>, inputs : (u32, u32)) {
        let actual = count_by(inputs.0, inputs.1);

        assert!(
            expected == actual,
            "\nTest failed!\n expected: [{}]\n actual: [{}]\n x: {}\n n: {}\n"
            , expected.iter().join(", ")
            , actual.iter().join(", ")
            , inputs.0
            , inputs.1
        );
    }
////////////////////////////////////////////////////////////////////////////////////////////////////
    use super::basic_op;

    fn dotest(op: char, v1: i32, v2: i32, expected: i32) {
        let actual = basic_op(op, v1, v2);
        assert!(actual == expected,
                "With operator = '{op}', value1 = {v1}, value2 = {v2}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn example_tests() {
        dotest('+', 4, 7, 11);
        dotest('-', 15, 18, -3);
        dotest('*', 5, 5, 25);
        dotest('/', 49, 7, 7);
    }
}