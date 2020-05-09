// https://github.com/mesalock-linux/rust-optee-trustzone-sdk/blob/master/optee-utee/macros/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn os_create(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_CreateEntryPoint() -> zondee::wrapper::os::raw::TEE_Result {
            let rslt: zondee::wrapper::os::Result<()> = #fun_name();
            match rslt {
                Ok(_) => zondee::wrapper::os::raw::TEE_SUCCESS,
                Err(e) => e as u32
            }
        }

        #fun
    )
    .into()
}

pub fn os_open_session(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_OpenSessionEntryPoint(
            _param_types: u32,
            _params: &mut [zondee::wrapper::os::raw::TEE_Param; 4],
            _sess_ctx: *mut *mut core::ffi::c_void,
        ) -> zondee::wrapper::os::raw::TEE_Result {
            let rslt: zondee::wrapper::os::Result<()> = #fun_name();
            match rslt {
                Ok(_) => zondee::wrapper::os::raw::TEE_SUCCESS,
                Err(e) => e as u32
            }
        }

        #fun
    )
    .into()
}

pub fn os_invoke_command(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_InvokeCommandEntryPoint(
            _sess_ctx: *mut core::ffi::c_void,
            cmd_id: u32,
            param_types: u32,
            params: &mut [zondee::wrapper::os::raw::TEE_Param; 4],
        ) -> zondee::wrapper::os::raw::TEE_Result {
            let mut params = zondee::wrapper::os::params(params, param_types);
            let fun: fn(u32, &mut zondee::wrapper::os::params) -> zondee::wrapper::os::Result<()> = #fun_name;
            match fun(cmd_id, &mut parameters) {
                Ok(_) => {
                    optee_utee_sys::TEE_SUCCESS
                },
                Err(e) => e.raw_code()
            }
        }

        #fun
    )
    .into()
}

pub fn os_close_session(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_CloseSessionEntryPoint(_sess_ctx: *mut core::ffi::c_void) {
            #fun_name();
        }

        #fun
    )
    .into()
}

pub fn os_destroy(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_DestroyEntryPoint() {
            #fun_name();
        }

        #fun
    )
    .into()
}
