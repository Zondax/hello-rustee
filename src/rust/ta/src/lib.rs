#![no_std]
//#![no_builtins]

#[cfg(test)]
#[macro_use]
extern crate log;

use optee_common::CommandId;
use zondee_utee::wrapper::{
    raw::{TEE_Param, TEE_PARAM_TYPES},
    ParamType, Parameters, TeeErrorCode as Error, Trace,
};

mod optee;

//use heapless::consts::*;
//use heapless::String;

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
/*pub fn to_hex_string(data: &[u8]) -> Result<String<U512>, String<U512>> {
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
*/

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
    signature_out.iter_mut().for_each(|i| *i = 'a' as u8);

    3
}

#[no_mangle]
pub extern "C" fn invoke_command(
    cmd_id: u32,
    param_types: u32,
    parameters: &mut [TEE_Param; 4],
) -> u32 {
    let mut params = Parameters::from_raw(parameters, param_types);

    match CommandId::from(cmd_id) {
        CommandId::SignWith => Trace::msg(format_args!("{}", "Signing with")),
        CommandId::SignWithAny => Trace::msg(format_args!("{}", "Signing with Any")),
        CommandId::EncryptPhrase => {
            let expected_param_types = TEE_PARAM_TYPES(
                ParamType::MemRefInput as u32,
                ParamType::MemRefOutput as u32,
                ParamType::None as u32,
                ParamType::None as u32,
            );
            if param_types != expected_param_types {
                Trace::msg(format_args!("{}", "Bad parameters for Encryption request"));
                return Error::BadParameters as _;
            }
            // for now just copy the data into the output buffer
            let mut imemref = unsafe {
                params
                    .0
                    .as_memref()
                    .expect("this is safe, the type was previously check")
            };
            let mut omemref = unsafe {
                params
                    .1
                    .as_memref()
                    .expect("this is safe, the type was previously check")
            };
            if omemref.buffer().len() == imemref.buffer().len() {
                omemref.buffer().copy_from_slice(imemref.buffer());
                Trace::msg(format_args!(
                    "{}",
                    "Encrypted phrase copied into output buffer"
                ));
                return 0;
            }
        }
        _ => {}
    }

    Error::NotSupported as u32
}

/*
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
}*/
