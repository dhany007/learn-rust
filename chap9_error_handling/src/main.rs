fn main() {
    println!("Error Handling");

    // unrecoverable errors with panic!
    // panic!("crash and burn")
    let  v = vec![1,2,3];
    v[99]; // index out of bounds: the len is 3 but the index is 99
}

