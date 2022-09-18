#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(build_os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use build_os_rust::println;
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use build_os_rust::memory;
    use build_os_rust::memory::translate_addr;
    use build_os_rust::memory::BootInfoFrameAllocator;
    use x86_64::structures::paging::Translate;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    build_os_rust::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let x = Box::new(41);

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
