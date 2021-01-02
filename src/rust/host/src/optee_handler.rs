//! Handler that implements the HandleRequest trait that is used by the host service
//! to call to optee for the execution of commands using an especific TA instance.
//! This Handler is implemented here because this module has access to private functions that
//! do the weight lifting of performing invocations to OPTEE through the TEEC api, which is unsafe
//! Also by doing this the host client doesnt need to depend on obscure OPTEE bindings
use zkms_common::{HandleRequest, RequestMethod, RequestResponse};

use optee_common::CommandId;

use crate::invoke_command;

use zondee_teec::wrapper::{Operation, ParamNone, ParamTmpRef};

#[derive(Default)]
pub struct Handler {}

impl HandleRequest for Handler {
    fn process_request(&self, request: RequestMethod) -> Result<RequestResponse, String> {
        match request {
            RequestMethod::Inc(msg) => {
                let mut msg = msg.to_le_bytes();
                let mut out = 0u64.to_le_bytes();
                let msg_slice = msg.as_mut();
                let out_slice = out.as_mut();
                let p0 = ParamTmpRef::new_input(msg_slice);
                let p1 = ParamTmpRef::new_output(out_slice);
                let mut operation = Operation::new(p0, p1, ParamNone, ParamNone);

                invoke_command(CommandId::Inc as _, &mut operation).map_err(|e| e.to_string())?;
                let out = u64::from_le_bytes(out);
                Ok(RequestResponse::Inc(out))
            }
            RequestMethod::Dec(msg) => {
                let mut msg = msg.to_le_bytes();
                let mut out = 0u64.to_le_bytes();
                let msg_slice = msg.as_mut();
                let out_slice = out.as_mut();
                let p0 = ParamTmpRef::new_input(msg_slice);
                let p1 = ParamTmpRef::new_output(out_slice);
                let mut operation = Operation::new(p0, p1, ParamNone, ParamNone);

                invoke_command(CommandId::Dec as _, &mut operation).map_err(|e| e.to_string())?;
                let out = u64::from_le_bytes(out);
                Ok(RequestResponse::Dec(out))
            }
        }
    }
}
