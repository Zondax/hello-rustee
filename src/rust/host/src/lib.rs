#![no_builtins]

use libc::c_void;
mod optee_handler;

use optee_common::{CommandId, TeeError};
//use optee_teec::{Operation, Param};
use zondee_teec::wrapper::{Operation, Param};

use host_app;

extern "C" {
    fn invoke_optee_command(command_id: u32, op: *mut c_void) -> u32;
}

pub(crate) fn invoke_command<A: Param, B: Param, C: Param, D: Param>(
    id: CommandId,
    op: &mut Operation<A, B, C, D>,
) -> Result<(), TeeError> {
    let res = unsafe { invoke_optee_command(id as u32, op.as_mut_ptr() as _) };
    if res == 0 {
        Ok(())
    } else {
        Err(TeeError::from_raw_error(res))
    }
}

#[no_mangle]
pub extern "C" fn host_test() -> u32 {
    12345
}

#[no_mangle]
pub extern "C" fn run() -> u32 {
    // Creates and runs the server instance by passing a request handler to be used
    host_app::start_service(optee_handler::Handler::default());

    12345
}

#[cfg(test)]
mod tests {
    use crate::host_test;

    #[test]
    fn test_local() {
        let v = host_test();
        assert_eq!(v, 12345);
    }
}
