#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
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