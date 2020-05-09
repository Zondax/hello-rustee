use serde::{Deserialize, Serialize};
use serde_cbor::{de, ser::SliceWrite, Serializer};

pub fn deserialize<'a: 'b, 'b, T>(bytes: &'a [u8], scratch: &mut [u8]) -> T
where
    T: Deserialize<'b>,
{
    de::from_slice_with_scratch(bytes, scratch).expect("Bad binary representation")
}

pub fn serialize<T>(instance: &T, buffer: &mut [u8])
where
    T: Serialize,
{
    let writer = SliceWrite::new(buffer);
    let mut ser = Serializer::new(writer);
    instance
        .serialize(&mut ser)
        .expect("Bad instance representation");
}
