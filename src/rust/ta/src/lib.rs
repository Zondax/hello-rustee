#![no_std]
#![no_builtins]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn ta_test() -> u8 {
    42
}
