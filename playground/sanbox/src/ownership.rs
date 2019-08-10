
fn move_sementics() {
    /*
        move 语义， 只作用于在编译期间不能知道数据大小的object，
        像 i64 这些基础类型，则会执行copy
        copy和drop在一个对象中是互斥的
        If a type has the Copy trait, an older variable is still usable after assignment. 
        Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, 
        has implemented the Drop trait.
    */

    let s1 = String::from("hello");
    // 这里的object 赋值语句类似于 c++中的 std::move
    // 把 move的功能放在语言层面了
    let s2 = s1;

    /* 
        下边这个语句编译不过，是为了防止多次释放内存
        In addition, there’s a design choice that’s implied by this: 
            Rust will never automatically create “deep” copies of your data. 
            Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
    */
    // println!("s1 is {}", s1);
}

fn deepcopy() {
    // 若果确实需要一个对象的副本， 可以用clone方法
    let s1 = String::from("world");

    // 包括分配空间和内容复制, 所以这种方式会比较expensive
    let _s2 = s1.clone();

    println!("s1:{}, s2:{}", s1, _s2);
}

fn takes_ownership(some_object: String) {
    println!("the string moved here is:{}", some_object);
}// some_object 在这里调用的drop方法，自此便不复存在. 
// 其流程是： run() 函数创建， move到tabke_ownership, 在take_ownership出栈的时候调用drop方法释放对象

fn make_copy(some_basic_type: i32) {
    println!("the basic type copied here is:{}", some_basic_type);
} // 因为是基础类型，不会调用drop方法，直接在函数调用栈pop就完了


fn takes_and_give_back(a_string: String) -> String {
    a_string
}


fn get_len(s: &String) -> usize {
    // 根据rust文档， 引用就是对象的指针
    s.len()
} // 这里不会调用drop方法，因为drop方法是属于对象的，而这里的s并不是一个对象，而是指向对象的指针

fn append(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn run() {
    move_sementics();
    deepcopy();

    {
        let s = String::from("hello");
        // "move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait"
        takes_ownership(s);
        // value borrowed here after move
        //println!("now s:{} is invalid", s);

        let x = 6;
        make_copy(x);
        println!("now x:{} is still valid", x);
    }
    // println!("now x:{} is out of scope", x);

    {
        let s = String::from("hello");
        let s1 = takes_and_give_back(s);

        // 注意这里的写法和c++中一样， 传递参数的时候也要有&前缀
        let len = get_len(&s1);
        println!("string length is:{}", len);

        /*
            1. At any given time, you can have either one mutable reference or any number of immutable references.
            2. References must always be valid.
        */

        let mut s2 = s1;
        println!("before mut the string is:{}", s2);
        append(&mut s2);
        println!("after mut the string is:{}", s2);

        let word = first_word(&s2);
        println!("slice find first word is:{}", word);
        s2.clear();
        println!("after clear s2 is:{}", s2);
    }
}
