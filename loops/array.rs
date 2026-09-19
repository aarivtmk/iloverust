fn main() {
    let arr = [2, 5, 6, 8, 10];
    //  a simple for loop
    for x in arr {
        println!("{}", x + 1);
    }

    //  iter
    // "Give me a way to go through the elements one by one without taking ownership of them."
    for x in arr.iter() {
        println!("{}", x + 1);
    }

    /*
    * for n in arr {
        println!("{}", n);
    }

    You absolutely can.

    The difference is:

    arr.iter()

    gives you an Iterator, which lets you use things like:

    .iter()
    .map(...)
    .filter(...)
    .find(...)
    *
    */
}
