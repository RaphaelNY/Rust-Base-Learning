#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("Value in cents: {}", value_in_cents(c));

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six: {:?}, None: {:?}", six, none);
     // match must include all possible cases
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // you can use _ to match all other cases, _ need to be at the end
    let v = 0u8;
    match v {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        7 => println!("Seven"),
        _ => println!("Other"), // _ match all other cases
    }
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // if let (match's syntactic sugar)
    let v = Some(0u8);
    match v {
        Some(3) => println!("Three"),
        _ => (), // _ match all other cases
    }

    if let Some(3) = v {
        println!("Three");
    } else {
        println!("Other");
    }
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1 // return value
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // return value
        Some(i) => Some(i + 1), // return value
    }
}