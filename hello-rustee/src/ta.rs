#![no_main]
#![no_std]

use hello_rustee::{Input, Output, UUID};
use zondee_utee::{
    framework::setup,
    wrapper::{self, Trace},
};

wrapper::params!(
    wrapper::Uuid::from_fields(UUID.as_fields()),
    DESCRIPTION: b"Hi RusTEE",
    VERSION: b"1.0"
);

#[setup]
fn setup(input: Input) -> wrapper::Result<Output> {
    Ok(match input {
        Input::HelloFromRee(hello_from_ree) => {
            Trace::msg(format_args!("{}", hello_from_ree.as_str()));
            Output::HelloFromTee("From TEE, this is a UTF-8 message 🎊".into())
        }
        Input::Version => Output::Version(42),
    })
}
