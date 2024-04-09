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
}