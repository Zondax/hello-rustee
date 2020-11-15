use zkms_common::{HandleRequest, KeystoreRequest, KeystoreResponse, RequestMethod};

use optee_common::CommandId;

use crate::invoke_command;

use optee_teec::{Operation, ParamNone, ParamTmpRef};

#[derive(Default)]
pub struct Handler {}

impl HandleRequest for Handler {
    fn process_request(&self, request: KeystoreRequest) -> Result<(), String> {
        let sender = request.sender;
        match request.method {
            RequestMethod::Encode(msg) => {
                let mut msg = msg.into_bytes();
                let mut out = vec![0u8; msg.len()];
                let p0 = ParamTmpRef::new_input(msg.as_mut());
                let p1 = ParamTmpRef::new_output(out.as_mut());
                let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);

                invoke_command(CommandId::Encode as _, &mut operation)
                    .map_err(|e| e.to_string())?;
                let out = String::from_utf8(out).map_err(|_| "Utf8 error".to_string())?;
                let _ = sender.send(KeystoreResponse::Encode(out));
            }
            RequestMethod::Decode(msg) => {
                let mut msg = msg.into_bytes();
                let mut out = vec![0u8; msg.len() * 2];
                let p0 = ParamTmpRef::new_input(msg.as_mut());
                let p1 = ParamTmpRef::new_output(out.as_mut());
                let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);

                invoke_command(CommandId::Decode as _, &mut operation)
                    .map_err(|e| e.to_string())?;
                let out = String::from_utf8(out).map_err(|_| "Utf8 error".to_string())?;
                let _ = sender.send(KeystoreResponse::Decode(out));
            }
        }
        Ok(())
    }
}
