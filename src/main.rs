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

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    build_os_rust::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    build_os_rust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    build_os_rust::test_panic_handler(info)
}
