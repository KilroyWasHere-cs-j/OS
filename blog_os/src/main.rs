#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use bootloader::entry_point;
use bootloader::BootInfo;

use alloc::{rc::Rc, vec};
use blog_os::memory;
use blog_os::memory::BootInfoFrameAllocator;
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::kernel::display::{force_new_line, print, print_s, println, println_s, reset_screen};
use crate::kernel::scheduler::JobPool;

mod System69;
mod kernel;

lazy_static! {
    pub static ref JOBPOOL: Arc<Mutex<JobPool>> = Arc::new(Mutex::new(JobPool::new()));
}

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

    // Run memory tests
    memory_tests();

    kernel::delay::delay();
    reset_screen();

    loop {}
}

pub fn memory_tests() {
    // ! These should actually be in a test suite, but I'm too lazy to do that right now
    force_new_line();
    println("Heap allocation tests: ");

    force_new_line();

    // // allocate a number on the heap
    let heap_value = Box::new(41);
    print("heap_value at: ");
    println_s(heap_value.to_string());
    //
    // // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..20 {
        vec.push(i);
    }
    print("vec: ");
    println_s(vec.iter().map(|&x| x.to_string()).collect::<String>());

    print("Heap allocation tests done!");

    force_new_line();

    println("Reference counting tests: ");

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();

    // Memory tests
    println("current reference count is: ");
    print_s(Rc::strong_count(&cloned_reference).to_string());

    core::mem::drop(reference_counted);

    println("reference count is: ");
    print_s(Rc::strong_count(&cloned_reference).to_string());

    println("Reference counting tests done!");

    force_new_line();

    println("Gabeian booted successfully!");
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
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
