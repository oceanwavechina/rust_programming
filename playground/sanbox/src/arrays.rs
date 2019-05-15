
use std::mem;

pub fn run() {
    // rust 中的数组初始化一定要和声明的大小一样
    // 类型也必须一样
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // reassign
    numbers[2]=20;
    
    println!("number of array: {:?}", numbers);

    println!("get by index: {}", numbers[0]);

    // array are stack allocated
    println!("Array occupies  {}  bytes", mem::size_of_val(&numbers));

    // get slice
    let slice:&[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);
}