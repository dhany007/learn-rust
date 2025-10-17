use std::fmt;

fn main() {
    println!("Supertraits");

    // Supertrait = trait yang menjadi syarat dasar bagi trait lain.
    // alau kamu ingin sebuah type (struct, enum) mengimplementasikan trait A,
    // maka type itu juga harus sudah mengimplementasikan trait B, karena A bergantung pada B.


    let p = Point { x: 1, y: 3 };
    p.outline_print();
}

/*

**********
*        *
* (1, 3) *
*        *
**********

 */

// trait OutlinePrint: fmt::Display artinya:
// agar suatu type bisa mengimplementasikan OutlinePrint,type itu wajib juga mengimplementasikan Display.
//
// Di dalam outline_print, kita bebas memanggil .to_string() karena Display otomatis menyediakan method itu.

// Supaya bisa mencetak bingkai, tentu harus bisa menulis teks dulu.
// Jadi OutlinePrint bergantung pada Display.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

// adalah implementasi trait bawaan Rust bernama std::fmt::Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


trait Readable {
    fn read(&self);
}

// Loggable tidak bisa diimplementasikan tanpa Readable,
// karena Loggable memanggil self.read().
trait Loggable: Readable {
    fn log(&self) {
        println!("*** Loggable ***");
        self.read();
    }
}