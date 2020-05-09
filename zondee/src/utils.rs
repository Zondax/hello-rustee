use crate::StackStr;
use core::fmt::Write;

pub fn to_hex(data: &[u8]) -> crate::Result<StackStr<[u8; 512]>> {
    if data.len() * 2 >= 512 {
        return Err(crate::Error::InvalidHexInput);
    }
    let mut buf = StackStr::default();
    for &byte in data {
        write!(&mut buf, "{:02x}", byte).expect("Bad hex string");
    }
    Ok(buf)
}
