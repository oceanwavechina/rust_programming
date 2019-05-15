pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    numbers.push(50);
    numbers.pop();
    println!("Vector {:?}", numbers);

    // loop through vector values
    for x in numbers.iter() {
        println!("number: {}", x);
    }

    // loop and mutate value
    for x in numbers.iter_mut() {
        *x *=2;
    }
    println!("Vector: {:?}", numbers);
}