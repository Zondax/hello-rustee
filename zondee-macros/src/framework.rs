// https://github.com/mesalock-linux/rust-optee-trustzone-sdk/blob/master/optee-utee/macros/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg};

pub fn os_invoke_command(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_arg_ty = match &fun.sig.inputs[0] {
        FnArg::Typed(r) => &r.ty,
        _ => panic!("Wrong fn signature"),
    };
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_InvokeCommandEntryPoint(
            _sess_ctx: *mut core::ffi::c_void,
            _cmd_id: u32,
            param_types: u32,
            params: &mut [zondee_utee::wrapper::raw::TEE_Param; 4],
        ) -> zondee_utee::wrapper::raw::TEE_Result {
            let mut params = zondee_utee::wrapper::params(params, param_types);
            let mut scratch = [0; 128];
            let mut input_mem = unsafe { params[0].as_memref().expect("Buffer doesn't exist") };
            let decoded: #fun_arg_ty;
            decoded = zondee::deserialize(input_mem.buffer(), &mut scratch);
            match #fun_name(decoded) {
                Ok(output) => {
                    let mut output_mem = unsafe { params[1].as_memref().expect("Buffer doesn't exist") };
                    zondee::serialize(&output, output_mem.buffer());
                    zondee_utee::wrapper::raw::TEE_SUCCESS
                },
                Err(e) => e as u32
            }
        }

        #fun
    )
    .into()
}
