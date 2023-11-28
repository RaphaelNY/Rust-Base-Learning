fn main() {
    println!("Hello, world!");
    let number :u32 = 3;
     // first using way
    if number > 5{
        println!("Hello");
    } else {
        println!("World");
    }

    // second using way
    if number > 2 {
        println!("Hello");
    } else if number % 3 == 0 {
        println!("world");
    } else if number % 2 == 0 {
        println!("Hello World!");
    }

     // if we used more than one else if,we always used match to rewrite it.
     // if is a expression,can put in right of =
    let condition :bool = false;
    let number :i32 = if condition { 5 } else { 6 };
    println!("the value of number is {}", number);

     // circle
     /*
        loop {
            pirntln!("time");
        }
        this is a circle endless
     */
     // you can use break to stop the loop
    let mut counter :i32 = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }; // break + (return value)

    println!("the result is: {}", result);

     // while circle
    let mut number :u32 = 3;

    while number != 0 {
        println!("{}!!", number);
        number -= 1;
    }

    println!("Liftoff!!");

     // in rust, we always used for circle to go through like a=[10,20,30,40...]
    let a :[i32;5] =[10,20,30,40,50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for (index,value) in a.iter().enumerate() {
        println!("in index: {} is value: {}", index, value);
    }

     // range: create number in  start to end without end number
    for number in (1..4).rev() {
        println!("{}!", number);
    } // this output is 3!\n2!\n1!
    println!("Liftoff!!");
     /* in follow code, the output is 1!\n2!\n3!\n
             for number in 1..4 {
                println!("{}!", number);
             }
             println!("Liftoff!!");
         you can also used follow to make output 3 2 1 :
             for number in 4..1 {
                 println!("{}!", number);
             }
             println!("Liftoff!!");
      */
}
