extern crate piston_window;

use piston_window::*;
use std::net::TcpListener;

pub fn run() {
    
    test_stdlib_webserver();

    test_types();
    
    test_functions("chris");
    
    add(5, 5);

    test_dependency();
}

fn test_stdlib_webserver() {
    let addr = "127.0.0.1:10086";
    let listener = TcpListener::bind(addr).unwrap();
    println!("the webserver is listening at {} ...", addr);
    for stream in listener.incoming() {
        let stream  = stream.unwrap();

        println!("a connection was made !");
    }
}

fn test_dependency() {
    let mut window: PistonWindow = WindowSettings::new("hello piston!", [640, 480])
                                    .exit_on_esc(true).build().unwrap();
    
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0;4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], 
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}


fn test_types() {
    let x = 1;      // integer
    let y = 2.1;    // float number
    let chris = "chris is a string value";      // string
    let boolean = true;     // bool type
    let dynamic_math = 8 * 8;

    let my_array = [1, 2, 3, 4, 5, 6, 7];
    let my_tuple = ("test", 1.999, [3,4,5]);

    let (dynamic_x, dynamic_y, dynamic_z) = my_tuple;

    println!("the values are {}, {}, {}, {}, {}", x, y, chris, boolean, dynamic_z[2]);
}


fn test_functions(name:&str) {
    println!("hello there {}!", name)
}


fn add(x:i8, y:i8) {
    println!("{}", x+y);
}