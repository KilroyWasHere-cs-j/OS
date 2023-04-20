#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

/*https://os.phil-opp.com/ */

/* Build command:  
 * cargo build --target thumbv7em-none-eabihf  
 */

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}
