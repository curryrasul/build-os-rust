#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(build_os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use build_os_rust::println;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    build_os_rust::init();

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    for i in 1..100 {
        println!("-"); // new
    }

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    build_os_rust::test_panic_handler(info)
}
