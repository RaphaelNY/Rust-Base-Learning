enum IpAddrKind1 {
    V4,
    V6,
}

enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

 /* in standard library:
    struct Ipv4Addr {
        // --snip--
    }
    struct Ipv6Addr {
        // --snip--
    }
    enum IpAddrKind {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
 */

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}impl Message  {
    fn call(&self) {
        // method body would be defined here
    }
}

struct IpAddr {
    kind: IpAddrKind1,
    address: String,
}

fn main() {
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let four = IpAddrKind1::V4;
    let six = IpAddrKind1::V6;
    route(four);
    route(six);
    route(IpAddrKind1::V4);
    route(IpAddrKind1::V6);
     // error: route(IpAddrKind);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let home = IpAddr {
        kind: IpAddrKind1::V4,
        address: String::from("223.5.5.5"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind1::V6,
        address: String::from("::1"),
    };
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let home = IpAddrKind2::V4(223,5,5,5);
    let loopback = IpAddrKind2::V6(String::from("::1"));
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let q = Message::Quit;
    let m = Message::Move { x: 1, y: 2 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(157, 255, 163);
    m.call();
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // Option<T> enum
   /*
        rust don’t have the null feature that many other languages have.
        Option<T> enum is included in the prelude, along with its variants:
            enum Option<T> {
                Some(T),
                None,
            }
   */
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    let x :i8 = 5;
    let y :Option<i8> = Some(5);
    let m = x + y.unwrap(); // error: let m = x + y; x is i8, y is Option<i8>,its different
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
}

fn route(ip_kind: IpAddrKind1) {}