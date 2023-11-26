fn main() {
    let mut x = 5;
     /* mut can change the unavailable changed variable to available changed variable
        a constant used const key word, and it need to be capitalization
     */
    println!("the value of x is {}", x);

    let mut x = "sc";
    println!("the new x is {}", x);
     // shadowing
    let x = "sdsc";
    println!("the new x is {}", x);
}
