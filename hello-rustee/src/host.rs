use hello_rustee::{Input, Output, UUID};
use zondee::{
    framework::{self, Client},
    wrapper, StackStr,
};

// Theorically, these parameters shouldn't be needed by the host. Maybe a linking problem?
#[no_mangle]
pub static trace_ext_prefix: &[u8] = b"TA\0";
#[no_mangle]
pub static trace_level: libc::c_int = 0;

fn main() -> framework::Result<()> {
    let ctx = Default::default();
    let uuid = wrapper::client::Uuid::from_os_uuid(UUID);
    let mut client = Client::new(ctx, "HOST", Default::default())?;
    client.open_session(uuid, &mut Default::default())?;
    hello_from_ree(&mut client)?;
    version(&mut client)?;
    Ok(())
}

fn hello_from_ree(client: &mut Client) -> framework::Result<()> {
    let input = Input::HelloFromRee(StackStr::from_str("🎉 From REE, this is a UTF-8 message"));
    let rslt: Output = client.invoke_command(input)?;
    println!("{:?}", rslt);
    Ok(())
}

fn version(client: &mut Client) -> framework::Result<()> {
    let input = Input::Version;
    let rslt: Output = client.invoke_command(input)?;
    println!("{:?}", rslt);
    Ok(())
}
