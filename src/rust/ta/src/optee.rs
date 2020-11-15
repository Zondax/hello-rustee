use optee_common::{HandleTaCommand, TeeErrorCode as Error};
// This type encapsulates our application which aims to process ta commands
use ta_app::TaApp;

#[derive(Default)]
/// Main TA request handler which wrapps any type that implements the HandleTaCommand Trait
pub(crate) struct TaHandler<T: HandleTaCommand + Sync + Default>(T);

// This is safe because the ta framework serializes all of the incoming requests so that only one is
// processed at time
unsafe impl<T: HandleTaCommand + Sync + Default> Sync for TaHandler<T> {}

// This is ugly but, It is the only way, because once_cell and lazy_static generate a compilation error
// It is because those crates uses an internal Alloc which is no_std compatible for normal Rust libs, but here we are
// compiling this crate as a static lib, breaking many things that are usually no_std compatible
static mut TA_HANDLER: Option<TaHandler<TaApp>> = None;

pub(crate) fn get_ta_handler() -> Option<&'static mut impl HandleTaCommand> {
    unsafe {
        if let Some(ref mut handler) = TA_HANDLER {
            Some(&mut handler.0)
        } else {
            None
        }
    }
}

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> u32 {
    unsafe {
        // At this point no handler should have been created
        // Only one instance is allowed, so we create our command handler on each new session.
        if TA_HANDLER.is_some() {
            return Error::AccessDenied as u32;
        }
        TA_HANDLER.replace(TaHandler(TaApp::default()));
    }
    0
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() -> () {}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_session_context: *const u8) -> () {
    unsafe {
        // Once the client is done, the TA session is closed, dropping our handler
        let _ = TA_HANDLER.take();
    }
}
