#![no_std]
#![no_builtins]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn ta_test() -> u8 {
    42
}

#[cfg(test)]
mod tests {
    use crate::ta_test;

    #[test]
    fn test_local() {
        let v = ta_test();
        assert_eq!(v, 42);
    }
}
