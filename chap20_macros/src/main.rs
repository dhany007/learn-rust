use chap20_macros::HelloMacro;
use chap20_macros_derive::hello_macro;

fn main() {
    println!("Macros");
    /*
    - macros are a way of writing code that writes other code, which is known as metaprogramming
    - Macro dieksekusi saat compile-time, bukan saat runtime seperti fungsi biasa
     */

    let v = vec![1,2,3,4];

    // code diatas akan diekspansi seperti berikut ketika di compile
    let temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec.push(4);


    println!("Procedural Macro");
    // Macro jenis ini bekerja seperti “fungsi” yang menerima kode Rust (TokenStream) sebagai input,
    // memodifikasinya, lalu menghasilkan kode baru sebagai output.
    // Cargo.toml:
    // [lib]
    // proc-macro = true

    println!("Write a Custom derive Macro");
    Pancakes::hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}
