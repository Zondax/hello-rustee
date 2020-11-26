// TODO: use the error code define in zondee_utee::error module
use zondee_utee::wrapper::Trace;

#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> u32 {
    Trace::msg(format_args!("CreateEntryPoint has been called\n"));
    0
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() -> () {
    Trace::msg(format_args!("Destroying entry point\n"));
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_session_context: *const u8) -> () {
    Trace::msg(format_args!("CLossing session\n"));
}
