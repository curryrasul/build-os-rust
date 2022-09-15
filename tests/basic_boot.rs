#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(build_os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use build_os_rust::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    build_os_rust::test_panic_handler(info)
}
