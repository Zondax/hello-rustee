// TODO: use the error code define in zondee_utee::error module
use zondee_utee::wrapper::TaErrorCode as Error;

use ta_app::{close_session, open_session};

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> u32 {
    // Only one instance is allowed to run by session
    if let Err(_) = open_session() {
        Error::AccessDenied as _
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() -> () {}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_session_context: *const u8) -> () {
    close_session();
}
