use core::arch::asm;

/// Delays for a while
pub fn delay(delay: i64) {
    for _ in 0..delay {
        unsafe {
            asm!("nop");
        }
    }
}
