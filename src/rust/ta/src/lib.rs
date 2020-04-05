#![no_std]
#![no_builtins]

mod optee;

#[cfg(test)]
#[macro_use]
extern crate log;

use core::fmt::Write;
use heapless::consts::*;
use heapless::String;

//use schnorrkel;
//use schnorrkel::SIGNATURE_LENGTH;

const SIGNATURE_LENGTH: usize = 64;

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

#[no_mangle]
pub extern "C" fn ta_sign(
    message_ptr: *const u8,
    message_len: u32,
    signature_ptr: *mut u8,
    signature_length: u32,
) -> u8 {
    let message = unsafe {
        // FIXME: panic results in undefined behavior
        assert!(!message_ptr.is_null());
        core::slice::from_raw_parts(message_ptr, message_len as usize)
    };

    let signature_out = unsafe {
        // FIXME: panic results in undefined behavior
        assert!(!signature_ptr.is_null());
        assert_eq!(signature_length as usize, SIGNATURE_LENGTH);
        core::slice::from_raw_parts_mut(signature_ptr, signature_length as usize)
    };

    // let keypair = Keypair::generate_with(::rand_core::OsRng);
    //
    // let context = signing_context(b"this signature does this thing");
    // let signature = keypair.sign(context.bytes(message));
    //
    // signature_out.copy_from_slice(&signature.to_bytes());

    3
}

#[cfg(test)]
mod tests {
    extern crate simple_logger;

    use crate::{ta_version, to_hex_string};

    #[test]
    fn test_version() {
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
