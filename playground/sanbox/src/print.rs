pub fn run() {
    // print to console
    println!("hello from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "yanan", "China");

    // Positional Argument, 注意是从0开始的
    println!("{0} is from {1} and {0} likes to {2}", "yanan", "China", "code");

    // Named Arguments
    println!("{name} like to play {activity}", name="John", activity="Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder fro debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("1+1={}", 1+1);
}