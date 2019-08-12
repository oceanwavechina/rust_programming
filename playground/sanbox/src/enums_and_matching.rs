
#[derive(Debug)]

/* rust 的enume中的元素可以是不同的类型 */
enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Write (String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn reset_pos(&mut self) {
        println!("self of enume Message is {:#?}", self);
        match self {
            Message::ChangeColor(x, y, z) => {
                println!("x={}", x);
                // enum中的值是不可以修改吗??
                // x = &mut (0);
            },
            _ => {},
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match  x {
        None => None,
        Some(i) => Some(i+1),
    }
}

pub fn run() {

    // 结构体的初始化用{}
    let msg = Message::Move{x:32, y:32};
    println!("enum msg with anonymous struct is: {:#?}", msg);

    // 其他的初始化用()
    let mut msg = Message::ChangeColor(0, 0, 0);
    println!("enum msg with three vars is: {:#?}", msg);

    msg.reset_pos();

    // 不能是 3, 因为option中对应的是Some
    plus_one(Some(3));

    // 用if let 判断多参数的枚举比较特殊
    let b = Message::ChangeColor(0, 0, 0);
    if let Message::ChangeColor(x,y,z) = b {
        println!("if let match: {},{},{}", x, y, z);
    }

    // 注意一下带参数的struct的用法
    let b = Message::Move{x:-1, y:-1};
    if let Message::Move{x:p1, y:p2} = b {
        println!("if let match: {},{}", p1, p2);
    }
}