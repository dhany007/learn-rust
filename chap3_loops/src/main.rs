fn main() {
    println!("Repetition with Loops");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of counter is: {result}"); // counter = 20

    println!("loop labels");

    let mut count = 0;

    'counting_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break; // break the inner loop
            }
            if count == 2 {
                break 'counting_loop; // immediately break counting_loop from inner
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("conditional loop with while");

    let mut number = 3;

    while number != 0 {
        println!("number: {number}");
        number -= 1;
    }

    println!("liftoff");

    println!("looping through collection with for");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    println!("reversed loop in collection");
    for number in (1..4).rev() { // [3, 2, 1]
        println!("the value is: {number}");
    }
}
