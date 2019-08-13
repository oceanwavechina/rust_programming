#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/* rust 的访问权限比计严格 
    1. struct的每一个元素, 所有的默认都是private的，这个也符合数据隐藏的原则
    2. 需要mod和function都声明为pub的才可以
    3. 枚举的每个元素都是public的
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { }
        fn seat_at_table() { }
    }

   pub mod serving {
        fn take_order() { }
        pub fn serve_order() { }
        fn take_payemnt() { }
    }
}

fn serve_order() {}

mod back_of_house {

    // 结合了 c++的命名空间和python的as
    use crate::front_of_house::serving::serve_order as fserver_order;
    
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        super::front_of_house::serving::serve_order();
        fserver_order();
    }

    fn cook_order() { }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 绝对地址
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对地址
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // it;s private
    // meal.seasonal_fruit = String::from("blueberries");
}