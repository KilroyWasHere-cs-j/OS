use core::arch::asm;

/// Delays for a while
pub fn delay() {
    for _ in 0..100000000 {
        unsafe {
            asm!("nop");
        }
    }
}

pub fn s_delay() {
    for _ in 0..10000000 {
        unsafe {
            asm!("nop");
        }
    }
}