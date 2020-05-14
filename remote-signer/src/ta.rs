#![no_main]
#![no_std]

use rand_core::OsRng;
use remote_signer::{Input, Output, UUID};
use schnorrkel::{signing_context, Keypair};
use zondee_utee::{framework::setup, wrapper};

wrapper::params!(
    wrapper::Uuid::from_fields(UUID.as_fields()),
    DESCRIPTION: b"Remote signing",
    VERSION: b"0.1"
);

#[setup]
fn setup(input: Input) -> wrapper::Result<Output> {
    Ok(match input {
        Input::Sign(msg) => Output::Sign({
            let keypair = Keypair::generate_with(OsRng);
            let ctx = signing_context(b"Some context");
            let mut signature = heapless::Vec::default();
            signature.extend(keypair.sign(ctx.bytes(&msg)).to_bytes().iter().copied());
            signature
        }),
    })
}
