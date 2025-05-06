fn main() {
    println!("Reference and Borrowing");
    // Unlike a pointer, a reference is guaranteed to point to a valid value
    // of a particular type for the life of that reference

    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    // So, what happens if we try to modify something we’re borrowing?
    // let s2 = String::from("hello");
    // change(&s2); // error: `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

    // change s2 to be mut.
    // Then we create a mutable reference with &mut s2 where we call the change function,
    // and update the function signature to accept a mutable reference with some_string: &mut String
    let mut s2 = String::from("hello");
    change_mut(&mut s2);
    println!("s2 = {s2}"); // s2 = hello world

    // if you have a mutable reference to a value,
    // you can have no other references to that value
    let mut s3 = String::from("hello again");
    let r1 = &mut s3; // first mutable borrow occurs here
    let r2 = &mut s3; // error second mutable borrow occurs here
    // println!("{}, {}", r1, r2); // This error says that this code is invalid because we cannot borrow s as mutable more than once at a time

    // As always, we can use curly brackets to create a new scope,
    // allowing for multiple mutable references, just not simultaneous ones:
    let mut s4 = String::from("hello");
    {
        let r3 = &mut s4;
        println!("{}", r3);
    } // r3 goes out of scope here, so we can make a new reference with no problems.
    let r5 = &mut s4;
    println!("{}", r5);

    // We also cannot have a mutable reference while we have an immutable one to the same value.
    let mut s5 = String::from("hello");
    let r7 = &s5; // no problem
    let r8 = &s5; // no problem
    let r9 = &mut s5; // BIG PROBLEM
    // println!("r7 = {r7}, r8 = {r8}, r9 = {r9}"); // cannot borrow `s5` as mutable because it is also borrowed as immutable

    // Note that a reference’s scope starts from where it is introduced
    // and continues through the last time that reference is used
    // artinya setelah selesai dipake, maka bisa lanjut
    let mut s6 = String::from("hello");
    let r10 = &s6; // no problem
    let r11 = &s6; // no problem
    println!("r10 = {r10}, r11 = {r11}"); // sudah dipakai disini
    let r12 = &mut s6; // no problem karena sudah dipakai diatas
    println!("r12 = {r12}"); // the compiler can tell that the reference is no longer being used at a point before the end of the scope.

    println!("dangling references");
    // a pointer that references a location in memory that may have been given to someone else—
    // by freeing some memory while preserving a pointer to that memory
    // gak ada reference nya

    // let reference_to_nothing = dangle_reference(); // error karena tidak ada reference yang direturn
    let reference_to_nothing = dangle(); // sukses karena hanya return string
    println!("reference_to_nothing = {reference_to_nothing}");

    // The Rules of References
    // Let’s recap what we’ve discussed about references:
    // - At any given time, you can have either one mutable reference or any number of immutable references.
    // - References must always be valid.
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

// fn dangle_reference() -> &String { // fungsi ini mereturn reference
//     let s = String::from("hello");
//
//     &s // return reference s
// } // reference s tidak bisa keluar karena dalam scope ini, jadi fungsi ini invalid

fn calculate_length(s: &String) -> usize {
    s.len()
}

// error: `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// fn change(some_string: &String) {
//     some_string.push_str(" world");
// }

fn change_mut(some_string: &mut String) {
    some_string.push_str(" world");
}