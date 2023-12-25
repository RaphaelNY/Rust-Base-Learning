fn main() {
    let v :Vec<i32> = Vec::new(); // empty vector
    let ve :Vec<i32> = vec![1, 2, 3]; // in nomal, we use this way to create a vector:initial
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let mut vee = Vec::new();
    vee.push(5);
    vee.push(6);
    vee.push(7);
    vee.push(8);
    println!("{:?}\n", vee);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let veee = vec![5, 6, 7, 8, 9];
    let third: &i32 = &veee[2];
    println!("The third element is {}", third);

    match veee.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
}
