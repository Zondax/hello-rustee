use zkms_common::{HandleRequest, KeystoreResponse, RequestMethod};

use optee_common::CommandId;

use crate::invoke_command;

use zondee_teec::wrapper::{Operation, ParamNone, ParamTmpRef};

#[derive(Default)]
pub struct Handler {}

impl HandleRequest for Handler {
    fn process_request(&self, request: RequestMethod) -> Result<KeystoreResponse, String> {
        println!("Request {:?}", request);
        match request {
            RequestMethod::Inc(msg) => {
                println!("inc: {}", msg);
                let mut msg = msg.to_le_bytes();
                let mut out = 0u64.to_le_bytes();
                let msg_slice = msg.as_mut();
                let out_slice = out.as_mut();
                let p0 = ParamTmpRef::new_input(msg_slice);
                let p1 = ParamTmpRef::new_output(out_slice);
                println!("params created");
                let mut operation = Operation::new(p0, p1, ParamNone, ParamNone);
                println!("Operation created - Invoking command");

                invoke_command(CommandId::Inc as _, &mut operation).map_err(|e| e.to_string())?;
                let out = u64::from_le_bytes(out);
                println!("Result {}", out);
                Ok(KeystoreResponse::Inc(out))
            }
            RequestMethod::Dec(msg) => {
                println!("inc: {}", msg);
                let mut msg = msg.to_le_bytes();
                let mut out = 0u64.to_le_bytes();
                let msg_slice = msg.as_mut();
                let out_slice = out.as_mut();
                let p0 = ParamTmpRef::new_input(msg_slice);
                let p1 = ParamTmpRef::new_output(out_slice);
                println!("params created");
                let mut operation = Operation::new(p0, p1, ParamNone, ParamNone);
                println!("Operation created - Invoking command");

                invoke_command(CommandId::Dec as _, &mut operation).map_err(|e| e.to_string())?;
                let out = u64::from_le_bytes(out);
                println!("Result {}", out);
                Ok(KeystoreResponse::Dec(out))
            }
        }
    }
}
