
fn loop_break_with_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("loop break with value: {}", result);
}

fn loop_iter() {
    let a = [10, 20, 30, 40, 50];

    for ele in a.iter() {
        println!("value is: {}", ele);
    }
}

fn countdown() {
    // 1,2,3 不包括4
    //for number in (1..4) {
    for number in (1..4).rev() {
        println!("{}", number);
    }
}

pub fn run() {
    let number = 3;

    // rust中 判断的条件一定要是显示的 bool类型, 一下的写法不支持
    // if number  {
    if number > 1 {
        println!("number is non-zero");
    };

    // 类似python中那样的语法糖
    let cond:bool = true;
    let number = if cond {
        10
    } else {
        -10
    };
    println!("number after let-if: {}", number);

    // loop返回值
    loop_break_with_value();

    loop_iter();

    countdown();
}