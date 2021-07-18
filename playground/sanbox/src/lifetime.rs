
/* 声明周期
    that every reference in Rust has a lifetime,
        which is the scope for which that reference is valid

     In practice, it means that the lifetime of the reference returned by the longest function 
        is the same as the smaller of the lifetimes of the references passed in.
    
    加上lifetime的标志，并不能改变任何变量的生命周期，只是让编译器检查声明周期是不是合法的
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


pub fn run() {
    /*
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    */

    /*
    let string1  = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    
    println!("The longest string is {}", result);
    */
}