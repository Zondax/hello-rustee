#![no_std]
#![no_builtins]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn host_test() -> u32 {
    12345
}

#[cfg(test)]
mod tests {
  use crate::{host_test};

  #[test]
  fn test_local() {
    let v = host_test();
    assert_eq!(v, 12345);
  }
}
