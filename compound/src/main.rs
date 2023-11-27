fn main() {
    // some trying about tuple
    let tup: (i32, f64, u8) = (500, 1.2, 156);

    let (x, y, z) = tup;

    println!("{} {} {}", x, y, z);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // some trying about array
    let months = [
        "January",
        "February",
        "March",
        "Apirl",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let index: i32 = 12;
    let month = months[index];
     /*
        let index :i32 = 15;
        let month = months[index];
                           ^^^^^
                           there will be error because index out of bounds :15 > 12
       if you write this:
        let index = [12,13,14,15];
        let month = months[index[1]];
       it can be build accessed but cannot be run and call :(error: index out of bounds)
     */
    println!("{}", month);
}
