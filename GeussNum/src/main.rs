use std::cmp::Ordering;
use std::io; // prelude
use rand::Rng; // trait
fn main() {
    println!("welcome to my guessing Game!\nnow you can guess a number you like:");

    let secret_number = rand::thread_rng().gen_range(1..101); // i32 u32 i64

    let mut guess = String::new();
    loop {
        io::stdin().read_line(&mut guess).expect("cannot read line");
         // io::Result Ok,Err

         // shadow: when used guess after there, rust will used new guess, and the last guess was shadowed.
         // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // \n
         // trim: delete the space of guess
         // parse: based 'u32' change the guess, change to integer
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                println!("the secret number is {}", secret_number);
                println!("what you guess is: {}", guess);
                break;
            },
        };
    };


}