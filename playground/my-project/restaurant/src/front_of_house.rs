/* rust 的访问权限比计严格 
    1. struct的每一个元素, 所有的默认都是private的，这个也符合数据隐藏的原则
    2. 需要mod和function都声明为pub的才可以
    3. 枚举的每个元素都是public的
*/


mod front_of_house {

    /*
        相当于前置声明，实际会从同名文件中加载对应的方法
        Using a semicolon after mod front_of_house 
        rather than using a block tells Rust to load the contents of the module 
        from another file with the same name as the module.
    */
    pub mod hosting;

   pub mod serving {
        fn take_order() { }
        pub fn serve_order() { }
        fn take_payemnt() { }
    }
}