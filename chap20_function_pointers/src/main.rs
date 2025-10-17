fn main() {
    println!("function pointers");

    let answer = do_twice(add_one, 5);
    println!("The answer is {answer}");

    let list_of_numbers = vec![1,2,3];
    // Dengan closure
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Dengan function pointer (lebih eksplisit)
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string()).collect();
    // Kedua cara menghasilkan hasil yang sama,
    // tetapi contoh kedua lebih idiomatis kalau kamu ingin langsung memakai fungsi yang sudah ada.


    // Status::Value di sini adalah function pointer
    // yang otomatis membuat instance Status::Value(x) untuk setiap x dari 0 hingga 19.
    let list_of_statutes: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// fn(i32) i32
fn add_one(x: i32) -> i32 {
    x+1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}
