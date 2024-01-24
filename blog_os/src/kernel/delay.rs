use core::arch::asm;

/// Delays for a while
pub fn delay() {
    for _ in 0..100000000 {
        unsafe {
            asm!("nop");
        }
    }
}