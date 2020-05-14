use hello_rustee::{Input, Output, UUID};
use zondee_teec::framework::send_msg;

fn main() -> zondee_teec::Result<()> {
    hello_from_ree()?;
    version()?;
    Ok(())
}

fn hello_from_ree() -> zondee_teec::Result<()> {
    let input = Input::HelloFromRee("🎉 From REE, this is a UTF-8 message".into());
    let output: Output = send_msg(UUID.into(), input)?;
    println!("{:?}", output);
    Ok(())
}

fn version() -> zondee_teec::Result<()> {
    let input = Input::Version;
    let output: Output = send_msg(UUID.into(), input)?;
    println!("{:?}", output);
    Ok(())
}
