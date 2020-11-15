#![no_std]

use optee_common::{CommandId, HandleTaCommand, TeeErrorCode as Error};
use zondee_utee::wrapper::{raw::TEE_PARAM_TYPES, ParamType, Parameters, Trace};

#[derive(Default)]
pub struct TaApp {}

// This is safe because all request are serialized by the TA framework
unsafe impl Sync for TaApp {}

impl HandleTaCommand for TaApp {
    fn handle_command(
        &mut self,
        cmd_id: u32,
        param_types: u32,
        params: &mut Parameters,
    ) -> Result<(), Error> {
        match CommandId::from(cmd_id) {
            CommandId::Encode => {
                let expected_param_types = TEE_PARAM_TYPES(
                    ParamType::MemRefInput as u32,
                    ParamType::MemRefOutput as u32,
                    ParamType::None as u32,
                    ParamType::None as u32,
                );
                if param_types != expected_param_types {
                    Trace::msg(format_args!("{}", "Bad parameters for Encoding request"));
                    return Err(Error::BadParameters);
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
                if omemref.buffer().len() < imemref.buffer().len() * 2 {
                    return Err(Error::ShortBuffer);
                }
                omemref.buffer().copy_from_slice(imemref.buffer());
                //if hex::encode_to_slice(imemref.buffer(), omemref.buffer()).is_err() {
                //    return Error::BadFormat as _;
                //}
                return Ok(());
            }
            _ => {}
        }

        Err(Error::NotSupported)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
