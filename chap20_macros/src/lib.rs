#[macro_export]
// Contoh definisi macro vec! yang sederhana:
macro_rules! vec {
    ($($x:expr), *) => { // ambil nol atau lebih ekspresi (expr) yang dipisahkan koma
        {
            let mut temp_vec = Vec::new(); // diulang untuk setiap argumen yang diberikan
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

use proc_macro;

#[some_attribute]
// The function that defines a procedural macro takes a TokenStream as an input and produces a TokenStream as an output
//  The TokenStream type is defined by the proc_macro crate that is included with Rust and represents a sequence of tokens
pub fn some_name(input: TokenStream) -> TokenStream {
}



pub trait HelloMacro {
    fn hello_macro();
}