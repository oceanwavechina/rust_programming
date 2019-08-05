
/*
    函数声明时的 -> 代表的是返回值
    返回值有两个写法，可以是最后的一个expression 或是 用expcitly 用return
    the return value of the function is 
        1. synonymous with the value of the final expression in the block of the body of a function. 
        2. You can return early from a function by using the return keyword and specifying a value, 
            but most functions return the last expression implicitly. 
*/
fn five() -> i32 {
    // return 5;
    5
}

// 函数默认的返回值是 empty tuple
fn danger_return() -> () {
    5;
    ()
}

pub fn run() {
    // rust 不支持这种写法
    // Statements do not return values. 
    // Therefore, you can’t assign a let statement to another variable, 
    // as the following code tries to do;
    //let x = (let y = 6);

    /*
        注意大括号里边是创建的一个expression, 最后一句没有加分号
        下边是expression和statement的区别 
        * Expressions do not include ending semicolons
        * Statemen If you add a semicolon to the end of an expression, 
            you turn it into a statement, which will then not return a value
    */
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {}", y);

    let z = five();
    println!("five returns: {}", z);

    println!("danger_return: {:?}", danger_return());
}