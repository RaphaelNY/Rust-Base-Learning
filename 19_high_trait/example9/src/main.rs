trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
     fn baby_name() -> String;
}


struct Dog;

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

impl Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
