fn main() {
    println!("Advanced Traits");

    println!("Associated Types");
}

// Associated types
// we can define a trait that uses some types without needing
// to know exactly what those types are until the trait is implemented.
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

// Associated type Item diisi dengan u32.
// Jadi Self::Item di dalam trait berarti u32 untuk tipe Counter
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
