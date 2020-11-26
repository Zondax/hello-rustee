// TODO: use the error code define in zondee_utee::error module
use ta_app::{close_session, open_session};
use zondee_utee::wrapper::{TaErrorCode as Error, Trace};

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> u32 {
    Trace::msg(format_args!("CreateEntryPoint has been called\n"));
    // Only one instance is allowed to run by session
    if let Err(_) = open_session() {
        Trace::msg(format_args!("Error can not create inner handler"));
        Error::AccessDenied as _
    } else {
        Trace::msg(format_args!("*****Session created\n"));
        0
    }
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() -> () {
    Trace::msg(format_args!("Destroying entry point\n"));
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_session_context: *const u8) -> () {
    Trace::msg(format_args!("CLossing session\n"));
    close_session();
}
