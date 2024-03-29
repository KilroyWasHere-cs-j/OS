#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
extern crate alloc;

use bootloader::entry_point;
use bootloader::BootInfo;

use blog_os::memory;
use blog_os::memory::BootInfoFrameAllocator;
use core::panic::PanicInfo;

use crate::kernel::display::{println, reset_screen};

mod kernel;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::allocator;
    use x86_64::VirtAddr;

    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    println("Hello World!");
    println("Booting into Gabeian...");

    println("Running pre_boot tests");

    println("Reseting screen");
    kernel::delay::delay(10000000);
    reset_screen();

    loop {}
}

pub fn dummy() {
    println("Hello World");
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    let hold = 1;
    assert_eq!(hold, 1);
}
