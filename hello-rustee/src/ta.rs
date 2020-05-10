#![no_main]
#![no_std]

use hello_rustee::{Input, Output, UUID};
use zondee::StackStr;
use zondee_utee::{
    framework::invoke_command,
    wrapper::{self, close_session, create, destroy, open_session, Trace},
};

wrapper::params!(
    wrapper::Uuid::from_fields(UUID.as_fields()),
    TA_DESCRIPTION: b"Hi RusTEE",
    TA_VERSION: b"1.0"
);

#[create]
fn create() -> wrapper::Result<()> {
    Ok(())
}

#[open_session]
fn open_session() -> wrapper::Result<()> {
    Ok(())
}

#[invoke_command]
fn invoke_command(input: Input) -> wrapper::Result<Output> {
    Ok(match input {
        Input::HelloFromRee(hello_from_ree) => {
            Trace::msg(format_args!("{}", hello_from_ree.as_str()));
            Output::HelloFromTee(StackStr::from_str("From TEE, this is a UTF-8 message 🎊"))
        }
        Input::Version => Output::Version(42),
    })
}

#[close_session]
fn close_session() {}

#[destroy]
fn destroy() {}
