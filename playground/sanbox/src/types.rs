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
    /*  Default is "i32"
        So how do you know which type of integer to use? 
        If you’re unsure, Rust’s defaults are generally good choices, 
        and integer types default to i32: this type is generally the fastest, even on 64-bit systems. 
        The primary situation in which you’d use isize or usize is when indexing some sort of collection.
    */
    let x = 1;

    /*  Default is "f64"
        because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
    */
    let y = 1.2;

    /*
        在release中检查integer overflow 可以用内置的函数
        下面的方式在 cargo run --release下也会提示
    */
    let p: u8 = 255;
    p.checked_add(1).expect("integer overflow");

    /*
        rust 中的整形溢出
        以下这个 用cargo run --release 就不会提示错误
        Let’s say you have a variable of type u8 that can hold values between 0 and 255. 
        If you try to change the variable to a value outside of that range, such as 256, 
        integer overflow will occur. Rust has some interesting rules involving this behavior. 
        When you’re compiling in debug mode, Rust includes checks for integer overflow 
        that cause your program to panic at runtime if this behavior occurs. 
        Rust uses the term panicking when a program exits with an error; 
        we’ll discuss panics in more depth in the “Unrecoverable Errors with panic!” section in Chapter 9.

        When you’re compiling in release mode with the --release flag, 
        Rust does not include checks for integer overflow that cause panics. 
        Instead, if overflow occurs, Rust performs two’s complement wrapping. 
        In short, values greater than the maximum value the type can hold “wrap around” to the 
        minimum of the values the type can hold. 
        In the case of a u8, 256 becomes 0, 257 becomes 1, and so on. 
        The program won’t panic, but the variable will have a value 
        that probably isn’t what you were expecting it to have. 
        Relying on integer overflow’s wrapping behavior is considered an error. 
        If you want to wrap explicitly, you can use the standard library type Wrapping.
    */
    let mut min: u8 = 255;
    min = min + 1;
    println!("integer overflow(256 for u8):{}", min);

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