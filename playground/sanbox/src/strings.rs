// Primitive str = Immutable fixed-length string  somewhere in memory
// String Growbale, heap-allocated data structure

pub fn run() {
    // 常量字符串，存在常量区
    let hello_immu = "Hello";
    println!("{} length is {}", hello_immu, hello_immu.len());

    // heap上的
    let mut hello = String::from("Hello");
 
    hello.push('大'); // 说明一个汉字占3个字节
    hello.push_str(" 世 界");

    // capacity
    println!("Capacity: {}", hello.capacity());

    println!("{} contains {}: {}", hello, '世', hello.contains('世'));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    println!("{} length is {}", hello, hello.len());
}