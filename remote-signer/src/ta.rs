#![no_main]
#![no_std]

use remote_signer::{Input, Output, UUID};
use zondee::{
    framework::os::invoke_command,
    wrapper::{
        self,
        os::{self, close_session, create, destroy, open_session},
    },
    StackVec,
};
use rand_core::OsRng;
use schnorrkel::{signing_context, Keypair};

wrapper::os::params!(
    UUID,
    TA_DESCRIPTION: b"Remote signing",
    TA_VERSION: b"0.1"
);

#[create]
fn create() -> os::Result<()> {
    Ok(())
}

#[open_session]
fn open_session() -> os::Result<()> {
    Ok(())
}

#[invoke_command]
fn invoke_command(input: Input) -> os::Result<Output> {
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
