fn main() {
    println!("Struct!");
    // A struct, or structure, is a custom data type that lets you package together
    // and name multiple related values that make up a meaningful group

    println!("Defining and Instantiating Structs");

    let mut user1 = User{
        active: true,
        username: String::from("some.username123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("email = {}", user1.email); // someone@example.com

    // If the instance is mutable,
    // we can change a value by using the dot notation and assigning into a particular field
    user1.email = String::from("new.email@example.com");
    println!("new email = {}", user1.email); // new.email@example.com


    let email = String::from("dhany@example.com");
    let username = String::from("dhany.ganteng");
    let user2 = build_user(email, username);
    println!("user2 email = {}", user2.username);

    println!("Creating Instances from Other Instances with Struct Update Syntax");
    // It’s often useful to create a new instance of a struct
    // that includes most of the values from another instance, but changes some.
    // You can do this using struct update syntax.

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("user3@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("user3 username = {}", user3.username); // some.username123

    // The syntax .. specifies that the remaining fields not explicitly
    // set should have the same value as the fields in the given instance.
    // Note that the struct update syntax uses = like an assignment; this is because it moves the data
    let user4 = User {
        email: String::from("user4@example.com"),
            ..user3
    };
    println!("user4 email = {}", user4.email); // user4@example.com

    println!("Using Tuplse Structs Without Named Fields to Create Different Types");
    // Tuple structs have the added meaning the struct name provides
    // but don’t have names associated with their fields;
    // rather, they just have the types of the fields.

    // Tuple structs are useful when you want to give the whole tuple a name
    // and make the tuple a different type from other tuples,
    // and when naming each field as in a regular struct would be verbose or redundant.

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // the black and origin values are different types because they’re instances of different tuple structs
    // Tuple struct instances are similar to tuples in that you can destructure them into their individual pieces,
    // and you can use a . followed by the index to access an individual value
    let Color(x, y, z) = black;
    println!("x = {}, y = {}, z = {}", x, y, z);
    let Point(r, g, b) = origin;
    println!("r = {}, g = {}, b = {}", r, g, b);

    println!("Unit-Like Structs Without Any Fields");
    // unit-like structs is structs that don’t have any fields
    // Unit-like structs can be useful when you need to implement a trait on some type
    // but don’t have any data that you want to store in the type itself
    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

struct Color(i32, i32, i32); // RGB
struct Point(i32, i32, i32); // xyz

// we use the struct keyword, the name we want, and then a semicolon.
// No need for curly brackets or parentheses!
struct AlwaysEqual;
