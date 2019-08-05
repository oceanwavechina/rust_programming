// Tuples have a fixed length: once declared, they cannot grow or shrink in size.

pub fn run() {
    let person: (&str, &str, i8) = ("Yanan", "China", 32);
    // 这种写法类似于c++11 get<0>()
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    // 这种写法类似于c++11 中的tie
    let (name, country, age) = person;
    println!("name is {}", name);

    // 好像没有长度限制
    let (a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15) 
        = (0, 0, 0, 0, 0, 0, 0, 0 ,0, 0, 0, 0, 0, 0, 0);
}