#![no_std]
#![no_builtins]

#[cfg(test)]
#[macro_use]
extern crate log;

use core::fmt::Write;
use heapless::consts::*;
use heapless::String;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// convert array to hexstring
pub fn to_hex_string(data: &[u8]) -> Result<String<U512>, String<U512>> {
    if data.len() * 2 >= 512 {
        // FIXME: Improve error types
        return Err(String::from("data should be less than 256 bytes"));
    }

    let mut buf = String::<U512>::new();

    for &byte in data {
        // FIXME: Improve error types
        write!(&mut buf, "{:02x}", byte)
            .map_err(|_e| String::<U512>::from("Error formatting bytes"))?;
    }

    Ok(buf)
}

#[no_mangle]
pub extern "C" fn ta_version() -> u8 {
    42
}

#[cfg(test)]
mod tests {
    extern crate simple_logger;
    use crate::{ta_version, to_hex_string};

    #[test]
    fn test_local() {
        simple_logger::init().unwrap();

        let v = ta_version();
        info!("Version {}", v);

        assert_eq!(v, 42);
    }

    #[test]
    fn test_logging() {
        simple_logger::init().unwrap();

        let dummy = [0; 10];

        info!("Dummy {}", to_hex_string(&dummy).expect("error"));
    }
}
