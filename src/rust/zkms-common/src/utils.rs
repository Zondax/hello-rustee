use std::fmt::Write;

/*
//#[cfg(all(feature = "with-serde", feature = "with-serde_cbor"))]
pub fn deserialize<'a: 'b, 'b, T>(bytes: &'a [u8], scratch: &mut [u8]) -> T
where
    T: serde::Deserialize<'b>,
{
    serde_cbor::de::from_slice_with_scratch(bytes, scratch).expect("Bad binary representation")
}

//#[cfg(all(feature = "with-serde", feature = "with-serde_cbor"))]
pub fn serialize<T>(instance: &T, buffer: &mut [u8])
where
    T: serde::Serialize,
{
    let writer = serde_cbor::ser::SliceWrite::new(buffer);
    let mut ser = serde_cbor::Serializer::new(writer);
    instance
        .serialize(&mut ser)
        .expect("Bad instance representation");
}
*/

pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T, serde_cbor::Error>
where
    T: serde::Deserialize<'a>,
{
    serde_cbor::from_slice(bytes)
}

pub fn serialize<T>(instance: &T) -> Result<Vec<u8>, serde_cbor::Error>
where
    T: serde::Serialize,
{
    /*let writer = serde_cbor::ser::SliceWrite::new(buffer);
    let mut ser = serde_cbor::Serializer::new(writer);
    instance
        .serialize(&mut ser)
        .expect("Bad instance representation");*/
    serde_cbor::to_vec(instance)
}
