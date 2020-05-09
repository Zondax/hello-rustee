// https://github.com/mesalock-linux/rust-optee-trustzone-sdk/blob/master/optee-utee/macros/src/lib.rs

use proc_macro::TokenStream;

#[cfg(feature = "framework")]
mod framework;
mod wrapper;

#[cfg(feature = "framework")]
#[proc_macro_attribute]
pub fn framework_os_invoke_command(_: TokenStream, input: TokenStream) -> TokenStream {
    framework::os_invoke_command(input)
}

#[proc_macro_attribute]
pub fn wrapper_os_close_session(_: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::os_workflow::os_close_session(input)
}

#[proc_macro_attribute]
pub fn wrapper_os_create(_: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::os_workflow::os_create(input)
}

#[proc_macro_attribute]
pub fn wrapper_os_destroy(_: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::os_workflow::os_destroy(input)
}

#[proc_macro_attribute]
pub fn wrapper_os_invoke_command(_: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::os_workflow::os_invoke_command(input)
}

#[proc_macro_attribute]
pub fn wrapper_os_open_session(_: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::os_workflow::os_open_session(input)
}

#[proc_macro]
pub fn wrapper_os_params(input: TokenStream) -> TokenStream {
    wrapper::os_params::os_params(input)
}
