fn main() {
    println!("Common Collections");

    /*
        - A vector allows you to store a variable number of values next to each other.
        - A string is a collection of characters. We’ve mentioned the String type previously,
          but in this chapter we’ll talk about it in depth.
        - A hash map allows you to associate a value with a specific key.
          It’s a particular implementation of the more general data structure called a map.
     */

    println!("Vectors");

    // create a new empty vector
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // create a new vector that holds the values you give it
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    //  if we want to be able to change its value, we need to make it mutable using the mut keyword
    let mut v3 = Vec::new();
    v3.push(5); // add elements
    v3.push(6);
    v3.push(7);
    println!("{:?}", v3);

    println!("Reading Elements of Vectors");
    // via indexing or by using the get method.

    let third = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    println!("Iterating over the values in a vector");
    for i in &v3 {
        println!("{}", i);
    }

    let mut v4 = vec![1,2,3,4];
    for i in &mut v4 {
        *i += 50;
    }
    println!("{:?}", v4);

    // We can define an enum whose variants will hold the different value types,
    // and all the enum variants will be considered the same type: that of the enum.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{row :?}");
}

 #[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
