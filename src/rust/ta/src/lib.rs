#![no_std]
#![no_builtins]

#[cfg(test)]
#[macro_use]
extern crate log;

use ta_app::TaApp;
use zondee_utee::wrapper::Parameters;
use zondee_utee::{wrapper::raw::TEE_Param, wrapper::{TaErrorCode as Error, Trace}, HandleTaCommand};

mod optee;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Trace::msg(format_args!("{:?}\n", info));
    loop {}
}

#[no_mangle]
pub extern "C" fn invoke_command(
    cmd_id: u32,
    param_types: u32,
    parameters: &mut [TEE_Param; 4],
) -> u32 {
    Trace::msg(format_args!("Receiving command {}\n", cmd_id));
    let mut params = Parameters::from_raw(parameters, param_types);
    Trace::msg(format_args!("Got parameters\n"));
    if let Err(e) = TaApp::handle_command(cmd_id, param_types, &mut params) {
        e as _
    } else {
        0
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
