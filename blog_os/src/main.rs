
// 不用链接标准库
#![no_std]
// To tell the Rust compiler that we don't want to use the normal entry point chain
#![no_main]

// 自定义我们的测试框架
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[warn(dead_code)]
static HELLO: &[u8] = b"Hello World!";

//
// diverging function
// https://doc.rust-lang.org/stable/rust-by-example/fn/diverging.html
//
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// test框架的入口函数
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    // println!("Running {} tests", tests.len());
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    // print!("trivial assertion ...");
    serial_println!("trivial assertion ...");
    // assert_eq!(1, 1);
    assert_eq!(1, 2);
    // println!("[ok]");
    serial_println!("[ok]");
}

// 退出qemu虚拟机的函数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]    //  每个成员按32位的无符号类型存储
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}



// disable the name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop{}
}