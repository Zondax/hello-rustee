#![no_std]
#![no_builtins]

#[cfg(test)]
#[macro_use]
extern crate log;

use optee_common::{CommandId, HandleTaCommand};
use ta_app::borrow_mut_app;
use zondee_utee::wrapper::{
    raw::{TEE_Param, TEE_PARAM_TYPES},
    ParamType, Parameters, TaErrorCode as Error, Trace,
};

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
    let mut params = Parameters::from_raw(parameters, param_types);

    // This check would depend on the opretion defined by cmd_id
    let expected_param_types = TEE_PARAM_TYPES(
        ParamType::MemRefInput as u32,
        ParamType::MemRefOutput as u32,
        ParamType::None as u32,
        ParamType::None as u32,
    );
    if param_types != expected_param_types {
        Trace::msg(format_args!("{}", "Bad parameters for Encoding request"));
        return Error::BadParameters as _;
    }

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

    let cmd = CommandId::from(cmd_id);

    // The inner handler could have persistance data or state that is required along the execution of the program
    // so instead of creating a handler on every command_invocation, we created the handler when the session is opened.
    // Such session remains open until the TEEC closes it.
    if let Some(ta_handler) = borrow_mut_app().as_mut() {
        if let Err(e) = ta_handler.process_command(cmd, imemref.buffer(), omemref.buffer()) {
            return e as _;
        }
        0
    } else {
        Error::ItemNotFound as _
    }
}
