
// 不用链接标准库
#![no_std]
// To tell the Rust compiler that we don't want to use the normal entry point chain
#![no_main]

use core::panic::PanicInfo;

//
// diverging function
// https://doc.rust-lang.org/stable/rust-by-example/fn/diverging.html
//
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// disable the name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}