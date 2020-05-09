use crate::{Array, StackVec};
use core::{
    fmt,
    iter::Extend,
    ops::{Deref, DerefMut},
    slice::Iter,
    str,
};
#[cfg(feature = "with-serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct StackStr<A>(StackVec<A>);

impl<A> StackStr<A>
where
    A: Array<Item = u8>,
{
    pub fn new(array: A, len: usize) -> Self {
        Self(StackVec::new(array, len))
    }

    pub fn from_arguments(args: fmt::Arguments) -> Self {
        let mut ss = StackStr::default();
        fmt::write(&mut ss, args).expect("Bad formatting");
        ss
    }

    pub fn from_str(s: &str) -> Self {
        assert!(s.len() <= A::CAPACITY);
        let mut ss = StackStr::default();
        ss.extend(s.as_bytes().iter().copied());
        ss
    }

    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
    }
}

#[cfg(feature = "with-serde")]
impl<'de, A> Deserialize<'de> for StackStr<A>
where
    A: Array<Item = u8>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use core::marker::PhantomData;
        use serde::de::{Error, Unexpected, Visitor};

        struct StackStrVisitor<'de, A>(PhantomData<(&'de (), A)>);

        impl<'de, A> Visitor<'de> for StackStrVisitor<'de, A>
        where
            A: Array<Item = u8>,
        {
            type Value = StackStr<A>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "an array with no more than {} items",
                    A::CAPACITY
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(StackStr::from_str(v))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let map = |_| E::invalid_value(Unexpected::Bytes(v), &self);
                let s = str::from_utf8(v).map_err(map)?;
                Ok(StackStr::from_str(s))
            }
        }

        deserializer.deserialize_seq(StackStrVisitor::<A>(PhantomData))
    }
}

#[cfg(feature = "with-serde")]
impl<A> Serialize for StackStr<A>
where
    A: Array<Item = u8>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self)
    }
}

impl<A> AsMut<str> for StackStr<A>
where
    A: Array<Item = u8>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        str::from_utf8_mut(&mut self.0).unwrap()
    }
}

impl<A> AsRef<str> for StackStr<A>
where
    A: Array<Item = u8>,
{
    #[inline]
    fn as_ref(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
    }
}

impl<A> Default for StackStr<A>
where
    A: Array<Item = u8>,
{
    #[inline]
    fn default() -> Self {
        Self(StackVec::default())
    }
}

impl<A> Deref for StackStr<A>
where
    A: Array<Item = u8>,
{
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<A> fmt::Display for StackStr<A>
where
    A: Array<Item = u8>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self as &str)
    }
}

impl<A> DerefMut for StackStr<A>
where
    A: Array<Item = u8>,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<A> Extend<A::Item> for StackStr<A>
where
    A: Array<Item = u8>,
{
    #[inline]
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A::Item>,
    {
        self.0.extend(iter);
    }
}

impl<A> fmt::Debug for StackStr<A>
where
    A: Array<Item = u8>,
    A::Item: fmt::Debug,
{
    #[inline]
    fn fmt(&self, args: &mut fmt::Formatter) -> fmt::Result {
        args.debug_tuple("StaticString").field(&self.0).finish()
    }
}

impl<'a, A> IntoIterator for &'a StackStr<A>
where
    A: Array<Item = u8>,
{
    type Item = &'a A::Item;
    type IntoIter = Iter<'a, A::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<A> fmt::Write for StackStr<A>
where
    A: Array<Item = u8>,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let len = self.0.len();
        let remaining_capacity = A::CAPACITY - len;
        if s.len() > remaining_capacity {
            return Err(core::fmt::Error);
        }
        self.0.extend(s.as_bytes().iter().copied());
        Ok(())
    }
}
