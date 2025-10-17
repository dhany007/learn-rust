use std::ops::Add;

fn main() {
    println!("Operator overloading");

    let length = Millimeters(20) + Meters(10);
    println!("length {:?}", length);

}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


// ini adalah default dari Add
// Rhs singkatan dari Right-Hand Side — yaitu nilai di sebelah kanan operator +.
// Rhs = Self artinya, kalau tidak ditentukan,
// Rhs akan sama dengan tipe yang mengimplementasikan trait Add.
// contoh diatas kita mengganti rhs nya


// trait Add<Rhs=Self> {
//     type Output;
//
//     fn add(self, rhs: Rhs) -> Self::Output;
// }
