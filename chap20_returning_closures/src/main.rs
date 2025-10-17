fn main() {
    println!("returning closures");
    // 1. Masalah: Closure tidak punya “tipe konkret”
    /*
        Closure di Rust tidak punya satu tipe yang bisa kamu tulis eksplisit —
        setiap closure yang kamu buat akan punya tipe unik,
        yang disembunyikan oleh compiler (disebut opaque type).
     */

    /*

    // 2. Masalah saat ada beberapa closure berbeda
    Ini tidak bisa dikompilasi, karena:
    - Meskipun keduanya punya signature sama (Fn(i32) -> i32),
    - Setiap impl Fn(...) menciptakan tipe tersembunyi (opaque type) yang berbeda.
    - Jadi impl Trait di dua fungsi berbeda ≠ tipe yang sama.
     */
    // let handlers = vec![returns_closure(), returns_initialized_closure(3)];
    // for handler in handlers {
    //     println!("handling {}", handler(5));
    // }

    // 3. Solusi: Gunakan trait object
    // Untuk menyatukan berbagai closure (dengan tipe berbeda tapi trait sama),
    // kita perlu erasure tipe, yaitu lewat trait object (dyn Fn):
    let handlers = vec![
        returns_closure(),
        returns_initialized_closure(134),
    ];

    for handler in handlers {
        println!("{}", handler(5));
    }

    // Box<dyn Fn(i32) -> i32> = pointer dinamis ke nilai apa pun yang mengimplementasikan Fn(i32) -> i32
    // Ini disebut trait object
    // Artinya: tipe konkretnya boleh berbeda, asalkan implementasi trait-nya sama.

    /*
        Bayangkan impl Trait seperti kotak transparan berlabel tipe yang sama,
        tapi tipe isi di dalamnya harus sama.
        Sedangkan Box<dyn Trait> seperti kotak hitam universal — kamu tidak tahu apa di dalamnya,
        tapi kamu tahu semua isinya bisa melakukan hal yang sama (implementasi trait yang sama).
     */
}

// Fungsi ini berhasil karena cuma mengembalikan satu closure —
// compiler tahu tipe spesifiknya di balik layar.
// fn returns_closure() -> impl Fn(i32) -> i32 {
//     |x| x + 1
// }
//
// fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
//     |x| x + init
// }

// solusi trait object
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}