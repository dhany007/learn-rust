use::std::collection::Hashmap;

fn main() {
    println!("Newtype Pattern for type safety and abstraction");

    /*
    - Type safety: mencegah kamu salah kirim type.
    - Abstraction: menyembunyikan detail internal.
     */

    let mm = Meters(2);
    // print_mm(mm) => tidak bisa — beda type
    // mencegah bug logika seperti mencampur meter dan milimeter.


    println!("alias type");
    // Type alias = nama lain untuk type yang sama, bukan type baru.
    let x: i32 = 5;
    let y: Kilometers = 10;
    // Kilometers dan i32 adalah type yang sama — compiler menganggap identik.
    // Perbedaan utama: Newtype → type baru,
    // Type alias → hanya nama lain dari type yang sama.

    println!("never type");

    /*
    Kok bisa continue (yang tidak mengembalikan nilai) disamakan dengan u32?
    Karena continue memiliki type !.
    Rust tahu: ! bisa dikonversi ke type apa pun, karena tidak akan pernah terjadi nilai nyata dari !.
    Makanya panic!, break, return, loop {}, semuanya punya type !.
     */
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Dynamically Sized Types (DST) dan Sized Trait")
    // let s: str = "andi"; //tidak bisa
    /*
    str sendiri adalah unsized type. Yang bisa kamu pegang adalah &str, karena:
    &str berisi dua informasi: pointer + length.
    Jadi ukurannya tetap diketahui (dua usize).
    Semua DST harus dibungkus pointer (&, Box, Rc, dsb):
    - &str
    - Box<str>
    - &dyn Trait
    - Box<dyn Trait>
     */
}


struct Meters(u32);
struct Millimeters(u32);

fn print_mm(mm: Millimeters) {
    println!("mm: {}", mm.0);
}

/*
Kelas luar cuma tahu People::add, tapi nggak tahu kalau di dalamnya pakai HashMap.
Jadi ini bentuk encapsulation tanpa class — khas Rust.
 */
pub struct People(Hashmap<i32, String>);

impl People {
    pub fn add(&mut self, name: string) {
        let id = self.0.len() as i32;
        self.0.insert(id, name);
    }
}

// Type alias = nama lain untuk type yang sama, bukan type baru.
type Kilometers = i32;

/*
! disebut never type (atau empty type) karena:
- Tidak punya nilai apa pun.
- Digunakan untuk menunjukkan bahwa fungsi tidak pernah kembali.
 */
fn bar() -> ! {
    panic!("never returns");
}

// sized
fn generic<T>(t: T) -> T {
    // snip
}

// sebenarnya rust menganggapnya seperti ini
// yang artinya hanya menerima type yang sized.
fn generic<T: sized>(t: T) -> T {
    // snip
}

// ?Sized berarti “T boleh sized, boleh tidak sized”.
// Tapi perhatikan: kamu harus pakai pointer (&T), karena type-nya bisa tidak berukuran tetap.
fn generic<T: ?sized>(t: &T) -> T {
    // snip
}