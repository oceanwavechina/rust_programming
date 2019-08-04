use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is :{}", secret_number);

    loop {
        println!("Please input your guess.");

        // rust 中所有的变量默认都是 immutable;
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Faild to read line");

        // rust中可以在把之前声明的名字绑定到一个新的变量上
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // parse 返回的是Result类型的结果，是一个enum，所以用match来匹配其中的内容
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your Guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
