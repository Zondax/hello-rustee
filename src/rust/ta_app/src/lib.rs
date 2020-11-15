#![no_std]

use core::cell::{Ref, RefCell, RefMut};

use optee_common::CommandId;
use zondee_utee::{
    wrapper::{raw::TEE_PARAM_TYPES, ParamType, Parameters, TaErrorCode as Error, Trace},
    HandleTaCommand,
};

#[derive(Default)]
struct TaApp {}

// This is safe because all request are serialized by the TA framework
unsafe impl Sync for TaApp {}

type InnerHandler<T> = RefCell<Option<T>>;

/// Main TA request handler which wrapps any type that implements the HandleTaCommand Trait
struct TaHandler<T>(InnerHandler<T>);

// This is safe because the ta framework serializes all of the incoming requests so that only one is
// processed at time
unsafe impl<T: HandleTaCommand + Sync + Default> Sync for TaHandler<T> {}

// The privite handler for processing client commands
static TA_HANDLER: TaHandler<TaApp> = TaHandler(RefCell::new(None));

pub fn open_session() -> Result<(), ()> {
    // At this point no handler should have been created
    // Only one instance is allowed, so we create our command handler on each new session.
    if TA_HANDLER.0.borrow().is_some() {
        Err(())
    } else {
        TA_HANDLER.0.borrow_mut().replace(TaApp::default());
        Ok(())
    }
}

pub fn close_session() {
    // Once the client is done, the TA session is closed, dropping our handler
    let _ = TA_HANDLER.0.borrow_mut().take();
}

pub fn borrow_mut_app<'a>() -> RefMut<'a, Option<impl HandleTaCommand>> {
    TA_HANDLER.0.borrow_mut()
}

pub fn borrow_app<'a>() -> Ref<'a, Option<impl HandleTaCommand>> {
    TA_HANDLER.0.borrow()
}

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
