fn main() {
    println!("Strings!");

    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents".to_string();
    println!("{}", s);

    // appending to a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    // appending to string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {}", s );

    // indexing into strings
    let s1 = String::from("Hello");
    let h = &s1[0..2];
    println!("h is {}", h);

    // iterating over strings to char
    for c in s1.chars() {
        println!("{c}");
    }

    // iterating over strings to byte
    for b in s1.bytes() {
        println!("{b}");
    }
}
