mod error;
mod params;
#[allow(
    clippy::all,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
pub mod raw;
mod trace;
#[allow(clippy::all, non_camel_case_types)]
mod user_ta_header;
#[allow(clippy::all, non_camel_case_types)]
mod uuid;

#[cfg(feature = "with-zondee-macros")]
pub use zondee_macros::{
    wrapper_utee_close_session as close_session, wrapper_utee_create as create,
    wrapper_utee_destroy as destroy, wrapper_utee_invoke_command as invoke_command,
    wrapper_utee_open_session as open_session, wrapper_utee_params as params,
};
pub use {self::uuid::*, error::*, params::*, trace::*, user_ta_header::*};

pub type Result<T> = core::result::Result<T, TeeErrorCode>;
