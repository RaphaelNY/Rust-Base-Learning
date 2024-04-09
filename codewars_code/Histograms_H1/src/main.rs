fn main() {
    println!("Histogram-H1 task");
}

fn histogram(results: &[u32;6]) -> String {
    let mut sall = String::new();
    let mut count = 1;
    for x in results.iter() {
        let mut s = String::new();
        {
            let mut y  = *x;
            loop{
                if y == 0 {
                    break;
                }
                y= y - 1;
                s = s + "#";
            }
        }
        if x == &0 {
            sall = format!("{:?}|\n", count) + &*sall;
        }else {
            sall = format!("{:?}|{} {:?}\n", count, s, x) + &*sall;
        }
        count = count + 1;
    }
    sall
}
/*
    a better solution:
    fn histogram(results: &[u32;6]) -> String {
        let mut i = results.len();
        let mut s = String::new();
        while i > 0 {
            let v = results[i-1];
            s += &(if v > 0 {format!("{}|{} {}\n", i, "#".repeat(v as usize), v)} else {format!("{}|\n", i)});
            i -= 1;
        }
        s
    }
*/

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::histogram;

    fn dotest(a: &[u32;6], expected: &str) {
        let actual = histogram(a);
        assert!(actual == expected, "With histogram = {a:?}\nExpected \"{expected}\" but got \"{actual}\"");
    }

    #[test]
    fn fixed_tests() {
        dotest(&[7,3,10,1,0,5], "6|##### 5\n5|\n4|# 1\n3|########## 10\n2|### 3\n1|####### 7\n");
        dotest(&[0,0,0,0,0,0],"6|\n5|\n4|\n3|\n2|\n1|\n");
    }
}