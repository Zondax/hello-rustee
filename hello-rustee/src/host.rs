use hello_rustee::{Input, Output, UUID};
use zondee_teec::wrapper::raw::{TEEC_Operation, TEEC_Session};

fn main() -> zondee_teec::Result<()> {
    println!("calling hello_from_ree");
    Ok(())
}
