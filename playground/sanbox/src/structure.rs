

/*
    https://doc.rust-lang.org/nomicon/repr-rust.html
    
    关于struct的数据对齐
        1. rust会通过插入padding的方式保证数据对齐，但是我们并不能假设这些padding真正会插入到那些地方
        2. 为了使得对齐时尽可能少的浪费内存空间，rust可能会改变成员的顺序
*/

// 加上这个指令为了终端打印输出 
#[derive(Debug)]
struct User {
    username: String, 
    email: String,
    sign_in_count: u64,
    sign_in_duration: u64,
    active: bool,
}

impl User {
    fn duration_per_signin(&self) -> u64 {
        self.sign_in_duration / self.sign_in_count
    }
}

impl User {
    // 这个类似于是静态方法，因为参数中没有self
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active:true,
            sign_in_count:8,
            sign_in_duration:200,
        }
    }
}

pub fn run() {
    let user = User::build_user("yanan@kuwo.cn".to_string(), "oceanwavechina".to_string());
    println!("the user struct is {:#?}", user);
    println!("duration per signin is {:#?}", user.duration_per_signin());
}