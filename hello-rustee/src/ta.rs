#![no_main]
#![no_std]

use hello_rustee::{Input, Output, UUID};
use zondee_utee::wrapper::{
    close_session, create, destroy, invoke_command, open_session, params, Parameters, Result,
    Trace, Uuid,
};

params!(
    Uuid::from_fields(UUID.as_fields()),
    DESCRIPTION: b"Hi RusTEE",
    VERSION: b"1.0"
);

#[create]
fn create() -> Result<()> {
    Trace::msg(format_args!("TA Created\n"));
    // Instantiate your ta_app here
    Ok(())
}

#[open_session]
fn open_session() -> Result<()> {
    Trace::msg(format_args!("opening TA session\n"));
    Ok(())
}

#[close_session]
fn close_session() {
    Trace::msg(format_args!("closing TA session\n"));
}

#[destroy]
fn destroy() {
    Trace::msg(format_args!("destroying TA session\n"));
}

#[invoke_command]
fn invoke_command(id: u32, params: &mut Parameters) -> Result<()> {
    Trace::msg(format_args!("processing command\n"));
    Ok(())
}

//#[setup]
//fn setup(input: Input) -> wrapper::Result<Output> {
//    Ok(match input {
//        Input::HelloFromRee(hello_from_ree) => {
//            Trace::msg(format_args!("{}", hello_from_ree.as_str()));
//            Output::HelloFromTee("From TEE, this is a UTF-8 message 🎊".into())
//        }
//        Input::Version => Output::Version(42),
//    })
//}
