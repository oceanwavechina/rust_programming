// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Yanan";
    println!("my name is {}", name);

    // mut的使用
    let age = 32;
    // ERROR: cannot assign twice to immutable variable
    // 变量默认是immutable的
    // age = 31;

    let mut the_age = 32;
    println!("I am {} years old", the_age);
    the_age = 31;
    println!("I am {} years old", the_age);

    // Define constants, 定义const的时候要指定类型 
    const QQ: i32 = 445716910;
    println!("my QQ No. is {}", QQ);

    // Assign multiple vars
    let (my_name, my_age ) = ("Yanan", 31);
    println!("{} is {}", my_name, my_age);
}