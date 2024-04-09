use std::thread;
use std::time::Duration;
use std::collections::HashMap;


 // used Hashmap to rewrite the Cacher struct,makes it can be used for different income.
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let x = self.value.get(&arg);
        match self.value.get(&arg) { // the value is a Option, so need to use * to get the value. self.value.get(&arg) return a Option<&T>,
            Some(v) => *v, // Some(&T) => v :&u32, *v :u32
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe>{
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

pub fn generate_workout(intensity: u32, random_number: u32) {

    let expensive_closure = |num| {
         // let expensive_closure = |num: u32| -> u32 {x + 1};
         // let expensive_closure = (num: u32) -> u32 {x + 1};
         // let expensive_closure = |num| {x + 1};
         // let expensive_closure = |num| x + 1;
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut expensive_closure = Cacher::new(expensive_closure);

    // let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} push ups!", expensive_closure.value(intensity));
        println!("Next, do {} sit ups!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);
         // in the second or more times, the value is not calculated again,will keep the value from first used.

        assert_eq!(v2, 2);
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        // for val in v1_iter {
        //    println!("Got: {}", val);
        //}

        assert_eq!(v1_iter.next(), Some(&1)); // it will change v1_iter to the next position.so v1_iter need be mutable.
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_sum_2() {
        let v1 = vec![1, 2, 3];
         // v1.iter().map(|x| x + 1); // it doesn't do anything, because the iterator is lazy, means it won't do anything until it's used.and what in v1 will not change.
        let v2 :Vec<_> = v1.iter().map(|x| x + 1).collect();
    }
}