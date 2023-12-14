 // first sample struct
struct User{
    username : String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
impl User {
    fn new(username: String, email: String) -> User {
        User {
            email,
            username,
            sign_in_count: 0,
            active: false,
        }
    }
 } // When the field name is the same as the corresponding variable name, the field initialization abbreviation can be used

fn main() {
    println!("Hello, world!");
     // create instance of struct
    let mut user1 = User { // empty string
        email: String::from(""),
        username: "".to_string(),
        active: false,
        sign_in_count: 0,
    }; // This method does not require the order of assignment
    user1.username = String::from("user1");
     // update struct
    let user2 = User {
        email: String::from("114514@gmail.com"),
        username: String::from("user2"),
        ..user1
    }; // user2's active and sign_in_count are the same as user1

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // unit-like struct
    struct UnitLikeStruct;
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
}

