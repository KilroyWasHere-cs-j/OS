// General purpose delay function
pub fn delay() {
    for i in 0..10000000 {
        let _ = i;
    }
}

pub fn delay_s(secs: i32) {
    for _i in 0..secs {
        delay();
    }
}
