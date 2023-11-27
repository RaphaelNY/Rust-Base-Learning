fn main() {
    println!("Hello, world!");
    another_function(20, "rust");
     /*
        a statement cannot be a expression,such as:
        let x = (let y = 6);
                 ^^^^^^^^^----------------------this is an error
        but this is available:
        let y = {
            let x = 1;
            x + 3
        };
        in this example, x + 3 without ";" means it will have a return value,and y will set as x + 3
        if you add ";"after x + 3, it will be an error
        attention:
         if you write x + 3; it will return a (), which is an empty tuple
     */
    let y = {
        let x = 1;
        x + 3;
    };
    println!("the value of new y is {:?} 😎", y);

    println!("the value of new five is {} 😎", five());
    println!("the value of new five is {} 😎", five_2(20));
     // you can used return key word witch is right too.
}
fn another_function(x :u32, y :&str){
    println!("this is a function but not main function in main.rs file!😊");
    println!("the value of x you put to this function is {}", x);
    println!("the value of y you put to this function is {}", y);
}

fn five() -> i32 (){
    5
}
fn five_2(x :i32) -> i32(){
    x + 5
}