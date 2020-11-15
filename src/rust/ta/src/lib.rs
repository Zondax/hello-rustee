#![no_std]
//#![no_builtins]

#[cfg(test)]
#[macro_use]
extern crate log;

use optee_common::{HandleTaCommand, TeeErrorCode as Error};
use zondee_utee::wrapper::{raw::TEE_Param, Parameters};

mod optee;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn invoke_command(
    cmd_id: u32,
    param_types: u32,
    parameters: &mut [TEE_Param; 4],
) -> u32 {
    let mut params = Parameters::from_raw(parameters, param_types);
    if let Some(ta_handler) = optee::get_ta_handler() {
        match ta_handler.handle_command(cmd_id, param_types, &mut params) {
            Err(code) => code as u32,
            _ => 0u32,
        }
    } else {
        Error::BadState as u32
    }
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
