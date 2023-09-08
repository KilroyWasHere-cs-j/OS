#![no_std]
#![no_main]

//  qemu-system-x86_64 -drive format=raw,file=/home/gabrielt/Git/OS/blog_os/target/x86_64-blog_os/debug/bootimage-blog_os.bin

// ? To deploy 
// ? cargo bootimage
// ? qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin

// https://os.phil-opp.com/

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn print(hold: &[u8]){
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in hold.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x8;
        }
    }

}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print(b"Hello World!");
    loop {}
}