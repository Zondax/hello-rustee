use hello_rustee::{Input, Output, UUID};
use zondee_teec::framework::Client;

fn main() -> zondee_teec::Result<()> {
    let ctx = Default::default();
    let mut client = Client::new(ctx, "HOST", Default::default())?;
    client.open_session(UUID.into(), &mut Default::default())?;
    hello_from_ree(&mut client)?;
    version(&mut client)?;
    Ok(())
}

fn hello_from_ree(client: &mut Client) -> zondee_teec::Result<()> {
    let input = Input::HelloFromRee("🎉 From REE, this is a UTF-8 message".into());
    let rslt: Output = client.invoke_command(input)?;
    println!("{:?}", rslt);
    Ok(())
}

fn version(client: &mut Client) -> zondee_teec::Result<()> {
    let input = Input::Version;
    let rslt: Output = client.invoke_command(input)?;
    println!("{:?}", rslt);
    Ok(())
}
