#![no_std]
#![no_builtins]

#[cfg(test)]
#[macro_use]
extern crate log;

use optee_common::CommandId;
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

    // The idea here is using a more generic approach
    // for instance, we might use Serialization/Deserialization
    // so that the inner type is recovered from the byte array depending on the operation
    // described by the cmd_id argument
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

    if imemref.buffer().len() != core::mem::size_of::<u64>()
        || omemref.buffer().len() != core::mem::size_of::<u64>()
    {
        return Error::OutOfMEmory as _;
    }

    let mut tmp = [0u8; core::mem::size_of::<u64>()];

    tmp.as_mut().copy_from_slice(imemref.buffer());
    let passed_value = u64::from_le_bytes(tmp);
    let mut ret_code = 0;

    let cmd = CommandId::from(cmd_id);

    // The inner handler could have persistance data or state that is required along the execution of the program
    // so instead of creating a handler on every command_invocation, we created the handler when the session is opened.
    // Such session remains open until the TEEC closes it.
    if let Some(ta_handler) = borrow_mut_app().as_mut() {
        let _ = ta_handler
            .process_value(cmd, passed_value)
            .map(|res| {
                omemref.buffer().copy_from_slice(res.to_le_bytes().as_ref());
            })
            .map_err(|e| ret_code = e as _);
    }
    ret_code
}
