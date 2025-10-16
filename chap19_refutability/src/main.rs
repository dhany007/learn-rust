fn main() {
    println!("Refutability: Whether a Pattern Might Fail to Match");

    // error because None is not recovered
    // let Some(x) = some_option_value;

    let x = 5 else {
        return;
    };
    // else akan unused_variables
}
