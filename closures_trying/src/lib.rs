use std::thread;
use std::time::Duration;


struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
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

        let v1 = c.value(1);
        let v2 = c.value(2);
         // in the second or more times, the value is not calculated again,will keep the value from first used.

        assert_eq!(v2, 2);
    }
}