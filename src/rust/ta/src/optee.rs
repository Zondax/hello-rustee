#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> u32 {
    0 // TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() -> () {}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(session_context: *const u8) -> () {}
