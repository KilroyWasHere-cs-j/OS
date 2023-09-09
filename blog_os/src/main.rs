#![no_std]
#![no_main]

//  qemu-system-x86_64 -drive format=raw,file=/home/gabrielt/Git/OS/blog_os/target/x86_64-blog_os/debug/bootimage-blog_os.bin

// ? To deploy 
// ? cargo bootimage
// ? qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin

// https://os.phil-opp.com/

mod display;

use display::vga_buffer::print;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print(b"Hello World!", display::vga_buffer::Color::Green);
    loop {}
}