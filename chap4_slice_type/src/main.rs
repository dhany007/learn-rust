fn main() {
    println!("The Slice Type");
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    // A slice is a kind of reference, so it does not have ownership.

    // write a function that takes a string of words separated by spaces
    // and returns the first word it finds in that string.
    // If the function doesn’t find a space in the string,
    // the whole string must be one word, so the entire string should be returned.
    let mut s = String::from("hello world");
    let word = first_word(&s); // // word will get the value 5
    println!("The word is {}", word);
    s.clear();                  // this empties the String, making it equal to ""
    println!("The s is {}", s); // `word` still has the value `5` here, but `s` no longer has any content
                                // that we could meaningfully use with the value `5`, so `word` is now
                                // totally invalid!


    println!("String Slices");
    let str = String::from("hello world");
    let hello = &str[0..5];  // hello
    let hello2 = &str[..5]; // hello, same as above
    println!("The string hello = {hello}" );
    println!("The string hello2 = {hello2}" );

    let world = &str[6..11]; // world
    let len = str.len();
    let world3 = &str[6..len]; // world
    let world4 = &str[..]; // hello world
    println!("The string world = {world}");
    println!("The string world3 = {world3}");
    println!("The string world4 = {world4}");

    let word = first_word_slices(&str);
    println!("The word is {}", word);
    // str.clear(); // error because clear needs to truncate the String, it needs to get a mutable reference.
    // println!("The word is {}", word);

    println!("String Slices as Parameters");
    println!("Other slices");
    let a = [1,2,3,4,5];
    let slice = &a[..3];
    assert_eq!(slice, &[1,2,3]);

}


// without using slices
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert our String to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iter is a method that returns each element in a collection
                                                        // enumerate wraps the result of iter and returns each element as part of a tuple instead
        println!("{} {}", i, item);                     // first element of the tuple returned from enumerate is the index,
                                                        // and the second element is a reference to the element
        if item == b' ' { // search for the byte that represents the space by using the byte literal syntax
            return i;   // If we find a space, we return the position
        }
    }

    s.len() // Otherwise, we return the length of the string
}

// The type that signifies “string slice” is written as &str
fn first_word_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return a string slice using the start of the string
                             // and the index of the space as the starting and ending indices.
        }
    }

    &s[..] // return all string slice
}
