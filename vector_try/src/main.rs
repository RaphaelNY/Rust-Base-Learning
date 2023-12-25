// use enum to save the many kind of data. following is the example
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new(); // empty vector
    let ve: Vec<i32> = vec![1, 2, 3]; // in nomal, we use this way to create a vector:initial
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
     /*
        if  "let third: &i32 = &veee[100];"
        this will cause the panic and exit the program. the index is out of bounds of this vector
     */

    match veee.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    println!("vector iteration:{:?}\n", vee);
    let first = &vee[0];
     // vee.push(6); // error: cannot borrow `vee` as mutable because it is also borrowed as immutable.
    println!("The first element is: {}", first);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // use enum to save the many kind of data. following is the example
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("enum vector:{:?}\n", row);
     /*
        we can use the enum to save the many kind of data. but the type of data must be the same.
        but, if the data is too much kind that you cant enum all, we can use the trait, it will learned in the next chapter.
     */
}
