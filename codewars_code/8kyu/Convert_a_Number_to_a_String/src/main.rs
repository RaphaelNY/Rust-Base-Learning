
use either::Either;

fn main() {
    println!("Convert a Number to a String!");
}

fn number_to_string(i: i32) -> String {
    i.to_string()
}

fn string_to_number(s: &str) -> i32 {
    s.parse().unwrap()
    // s.parse::<i32>().unwrap()
}

fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    arr.iter().map(|x| match x {
        Either::Left(i) => *i,
        Either::Right(s) => s.parse::<i32>().unwrap()
    }).sum()
    // arr.iter().cloned().map(|e| e.left_or_else(|x| x.parse::<i32>().unwrap())).sum()
}

#[cfg(test)]
mod tests {
    use super::number_to_string;

    fn dotest(n: i32, expected: &str) {
        let actual = number_to_string(n);
        assert_eq!(actual, expected, "With n = {n}\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest(67, "67");
        dotest(79585, "79585");
        dotest(1+2, "3");
        dotest(1-2, "-1");
    }
////////////////////////////////////////////////////////////////////////////////////////////////////
    use super::string_to_number;
    use rand::prelude::*;

    #[test]
    fn returns_expected() {
        assert_eq!(string_to_number("1234"), 1234);
        assert_eq!(string_to_number("605"), 605);
        assert_eq!(string_to_number("1405"), 1405);
        assert_eq!(string_to_number("-7"), -7);
    }

    #[test]
    fn works_on_random() {
        let mut rng = thread_rng();
        for _ in 0..5 {
            let num : i32 = rng.gen();
            let input = num.to_string();
            assert_eq!(string_to_number(&input), num);
        }
    }

    use super::sum_mix;
    use either::Either;

    fn dotest2(arr: &[Either<i32, String>], expected: i32) {
        let actual = sum_mix(arr);
        assert!(actual == expected, "With arr = {arr:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests2() {
        dotest2(&[Either::Left(9), Either::Left(3), Either::Right("7".to_string()), Either::Right("3".to_string())], 22);
        dotest2(&[Either::Right("5".to_string()), Either::Right("0".to_string().to_string()), Either::Left(9), Either::Left(3), Either::Left(2), Either::Left(1), Either::Right("9".to_string()), Either::Left(6), Either::Left(7)], 42);
        dotest2(&[Either::Right("3".to_string()), Either::Left(6), Either::Left(6), Either::Left(0), Either::Right("5".to_string()), Either::Left(8), Either::Left(5), Either::Right("6".to_string()), Either::Left(2), Either::Right("0".to_string())], 41);
        dotest2(&[Either::Right("1".to_string()), Either::Right("5".to_string()), Either::Right("8".to_string()), Either::Left(8), Either::Left(9), Either::Left(9), Either::Left(2), Either::Right("3".to_string())], 45);
        dotest2(&[Either::Left(8), Either::Left(0), Either::Left(0), Either::Left(8), Either::Left(5), Either::Left(7), Either::Left(2), Either::Left(3), Either::Left(7), Either::Left(8), Either::Left(6), Either::Left(7)], 61);
    }
}