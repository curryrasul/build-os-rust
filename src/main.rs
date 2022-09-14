#![no_std]
#![no_main]

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop {}
}
