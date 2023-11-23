#![no_std]
#![no_main]

//  qemu-system-x86_64 -drive format=raw,file=/home/gabrielt/Git/OS/blog_os/target/x86_64-blog_os/debug/bootimage-blog_os.bin

// ? To deploy
// ? cargo bootimage
// ? qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin

// https://os.phil-opp.com/
// https://blog.stephenmarz.com/2020/11/11/risc-v-os-using-rust-graphics/
// https://crates.io/crates/embedded-graphics
// https://github.com/drogue-iot/reqwless

//https://en.wikipedia.org/wiki/Code_page_437 for printing characters

mod display;
mod tests;
mod utils;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Let user know that the OS is booting
    println!("Booting GabeOS {}", "...");
    println!("Running Pre-boot Tests {}", "...");
    // Run a the on boot systems tests
    tests::test_runner::run_all_tests();
    println!("Pre-boot Tests Passed {}", "...");

    loop {
        utils::delay::delay_s(10);
        println!("GabeOS says hello...")
    }
}
