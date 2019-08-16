/*
    generic-type
        这个和c++的模板几乎是一模一样
    
    Traits:
        这个和c++中的接口类似
*/

// 这个是函数模板
fn largest<T:PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

fn test_generic_types() {
    let number_list = vec![34, 50, 28, 19, 100];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p = Point{ x:1, y:2.0};
    println!("The p Point is ({}, {})", p.x, p.y);
}

pub fn run() {
    test_generic_types();
}