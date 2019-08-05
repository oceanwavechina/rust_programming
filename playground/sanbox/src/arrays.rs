/* 
    固定长度: arrays in Rust have a fixed length

    Arrays are useful when you want your data allocated on the stack rather than the heap
    or when you want to ensure you always have a fixed number of elements
*/

use std::mem;

pub fn run() {
    // rust 中的数组初始化一定要和声明的大小一样
    // 类型也必须一样
    // 分号后边的是数组中元素的个数
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


    // 可以把数组初始化成同一个元素
    // 分号前边是初始值，后边是个数
    let a = [3; 6];
    println!("array with same values: {:?}", a);
}