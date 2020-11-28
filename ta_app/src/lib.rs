#![no_std]

use core::cell::{Ref, RefCell, RefMut};

use optee_common::{CommandId, HandleTaCommand, TeeErrorCode as Error};
use zondee_utee::wrapper::Trace;

#[derive(Default)]
pub struct TaApp {}

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

impl HandleTaCommand for TaApp {
    fn process_command(
        &mut self,
        cmd_id: CommandId,
        input: &[u8],
        output: &mut [u8],
    ) -> Result<(), Error> {
        if input.len() < core::mem::size_of::<u64>() || output.len() < core::mem::size_of::<u64>() {
            return Err(Error::OutOfMEmory);
        }

        let mut temp = [0u8; core::mem::size_of::<u64>()];
        temp.copy_from_slice(input);
        let value = u64::from_le_bytes(temp);
        let result = match cmd_id {
            CommandId::Inc => value.checked_add(1).ok_or(Error::Generic)?,
            CommandId::Dec => value.checked_sub(1).ok_or(Error::Generic)?,
            _ => return Err(Error::NotSupported),
        };

        output.copy_from_slice(result.to_le_bytes().as_ref());

        Ok(())
    }
}

pub fn open_session() -> Result<(), ()> {
    // At this point no handler should have been created
    // Only one instance is allowed, so we create our command handler on each new session.
    TA_HANDLER.0.borrow_mut().replace(TaApp::default());
    Ok(())
}

pub fn close_session() {
    // Once the client is done, the TA session is closed, dropping our handler
    let _ = TA_HANDLER.0.borrow_mut().take();
}

pub fn borrow_mut_app<'a>() -> RefMut<'a, Option<impl HandleTaCommand + 'static>> {
    Trace::msg(format_args!("Getting TA_app mut handler\n"));
    TA_HANDLER.0.borrow_mut()
}

pub fn borrow_app<'a>() -> Ref<'a, Option<impl HandleTaCommand + 'static>> {
    Trace::msg(format_args!("Getting TA_app handler\n"));
    TA_HANDLER.0.borrow()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
