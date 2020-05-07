
// 不用链接标准库
#![no_std]
// To tell the Rust compiler that we don't want to use the normal entry point chain
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[warn(dead_code)]
static HELLO: &[u8] = b"Hello World!";

//
// diverging function
// https://doc.rust-lang.org/stable/rust-by-example/fn/diverging.html
//
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

// disable the name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {

    //vga_buffer::print_something();

    println!("Hello World{}", "!");
    panic!("Some panic message");

    // let vga_buffer = 0xb8000 as *mut u8;

    // for(i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some number: {}, {}", 42, 1.37).unwrap();
        
    loop{}
}