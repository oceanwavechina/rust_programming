/*
Primitive Types
Interger u/i 8 - 128
Float f32 f64
Boolean
Characters char
Tuples
Arrays
*/

/* 
    Rus is a statically typed language, 
        which means that it must know the type of all variables at compile time，
        however, the compiler can usually infer what type we want to use base on the value
*/

// TODO: 如何输出变量的类型(不用 experimental 的方式)

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 1.2;

    // Add explicit type
    let z: i64 = 3243242342;

    // find max size
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    // boolean
    let is_active = true;
    println!("{:?}", (x, y, z, is_active));

    let is_greater = 10 > 8;
    println!("is_greater: {}", is_greater);

    // Unicode Char
    let a1 = '好';
    let face = '\u{1F600}';
    println!("a1: {}, {}", a1, face);
}