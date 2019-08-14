
use std::collections::HashMap;

fn test_Vec() {
     let mut v = vec![1,2,3];

    println!("create with initial values: {:#?}", v);

    //println!("value at index 4 is: {:#?}", v[4]);
    println!("value at index 4 is: {:#?}", v.get(4));

    let mut third_r: &i32 = &v[2];
    third_r = &0;
    println!("third_r is : {:#?}, vec of 3th is : {:#?}", third_r, v.get(2));

    let  mut third_v: i32 = v[2];
    third_v = 0;
    println!("third_v is : {:#?}, vec of 3th is : {:#?}", third_v, v.get(2));

    v[2] = -1;
    println!("vec of 3th is : {:#?}", v.get(2));   
}

fn test_HashMap() { 
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("hashmap content: {:#?}", map);
    //println!("invalid vars: {:#?}, {:#?}", field_name, field_value);
}

pub fn run() {

    test_Vec();

    test_HashMap();
}