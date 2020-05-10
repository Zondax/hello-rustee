#![no_main]
#![no_std]

use rand_core::OsRng;
use remote_signer::{Input,UUID, Output};
use schnorrkel::{signing_context, Keypair};
use zondee::StackVec;
use zondee_utee::{
    framework::invoke_command,
    wrapper::{self, close_session, create, destroy, open_session},
};

wrapper::params!(
    wrapper::Uuid::from_fields(UUID.as_fields()),
    TA_DESCRIPTION: b"Remote signing",
    TA_VERSION: b"0.1"
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
        Input::Sign(msg) => Output::Sign({
            let keypair = Keypair::generate_with(OsRng);
            let ctx = signing_context(b"Some context");
            let mut signature = StackVec::default();
            signature.extend(keypair.sign(ctx.bytes(&msg)).to_bytes().iter().copied());
            signature
        }),
    })
}

#[close_session]
fn close_session() {}

#[destroy]
fn destroy() {}
