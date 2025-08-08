fn main() {
    println!("Processing a Series of Items With Iterators");
    /*
        The iterator pattern allows you to perform some task on a sequence of items in turn.
        An iterator is responsible for the logic of iterating over each item
        and determining when the sequence has finished.
        When you use iterators, you don’t have to reimplement that logic yourself.
     */

    let v1 = vec![1,2,3,4,5];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("got: {val}");
    }

    let v2: Vec<i32> = vec![1,2,3];

    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
    assert_eq!(v3, vec![2, 3, 4]);
}
