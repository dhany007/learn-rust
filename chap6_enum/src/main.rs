fn main() {
    println!("Enum and Pattern Matching");

    println!("Defining an Enum");
    // enum value can only be one of its variants

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrWithoutStruct::V4(String::from("127.0.0.1"));
    let loopback = IpAddrWithoutStruct::V6(String::from("::1"));

    let home = IpAddrVariants::V4(127, 0, 0, 1);
    let loopback = IpAddrVariants::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));

    println!("The Option Enum");
    // The Option type encodes the very common scenario in which a value could be something,
    // or it could be nothing.

}

fn route(ip_kind: IpAddrKind) {

}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// attach data to each variant of the enum directly,
// so there is no need for an extra struct
enum IpAddrWithoutStruct {
    V4(String),
    V6(String),
}

// each variant can have different types and amounts of associated data
enum IpAddrVariants {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

enum Option<T>{
    None,
    Some(T),
}