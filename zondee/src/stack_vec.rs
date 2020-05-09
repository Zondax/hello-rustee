use crate::array::Array;
use core::{
    fmt,
    iter::Extend,
    ops::{Deref, DerefMut},
    slice::Iter,
};
#[cfg(feature = "with-serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct StackVec<A> {
    array: A,
    len: usize,
}

impl<A> StackVec<A>
where
    A: Array,
{
    pub fn new(array: A, len: usize) -> Self {
        Self { array, len }
    }

    pub fn is_empty(&self) -> bool {
        self.len != 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, item: A::Item) {
        assert!(self.len() <= A::CAPACITY);
        *self.array.slice_mut().get_mut(self.len).unwrap() = item;
        self.len += 1;
    }
}

#[cfg(feature = "with-serde")]
impl<'de, A, T> Deserialize<'de> for StackVec<A>
where
    A: Array<Item = T>,
    T: Copy + Default + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use core::marker::PhantomData;
        use serde::de::{Error, SeqAccess, Visitor};

        struct StackVecVisitor<'de, A, T>(PhantomData<(&'de (), A, T)>);

        impl<'de, A, T> Visitor<'de> for StackVecVisitor<'de, A, T>
        where
            A: Array<Item = T>,
            T: Copy + Default + Deserialize<'de>,
        {
            type Value = StackVec<A>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "an array with no more than {} items",
                    A::CAPACITY
                )
            }

            fn visit_seq<SA>(self, mut seq: SA) -> Result<Self::Value, SA::Error>
            where
                SA: SeqAccess<'de>,
            {
                let mut ss = StackVec::default();
                while let Some(value) = seq.next_element()? {
                    if ss.len() == A::CAPACITY {
                        return Err(SA::Error::invalid_length(A::CAPACITY + 1, &self));
                    }
                    ss.push(value);
                }
                Ok(ss)
            }
        }

        deserializer.deserialize_seq(StackVecVisitor::<A, T>(PhantomData))
    }
}

#[cfg(feature = "with-serde")]
impl<A> Serialize for StackVec<A>
where
    A: Array,
    A::Item: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self)
    }
}

impl<A> fmt::Debug for StackVec<A>
where
    A: Array,
    A::Item: fmt::Debug,
{
    #[inline]
    fn fmt(&self, args: &mut fmt::Formatter) -> fmt::Result {
        args.debug_struct("StaticString")
            .field("array", &self)
            .field("len", &self.len())
            .finish()
    }
}

impl<A> AsMut<[A::Item]> for StackVec<A>
where
    A: Array,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [A::Item] {
        self.array.slice_mut().get_mut(0..self.len).unwrap()
    }
}

impl<A> AsRef<[A::Item]> for StackVec<A>
where
    A: Array,
{
    #[inline]
    fn as_ref(&self) -> &[A::Item] {
        self.array.slice().get(0..self.len).unwrap()
    }
}

impl<A> Default for StackVec<A>
where
    A: Array,
    A::Item: Copy + Default,
{
    #[inline]
    fn default() -> Self {
        StackVec::new(A::new_filled(), 0)
    }
}

impl<A> Deref for StackVec<A>
where
    A: Array,
{
    type Target = [A::Item];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<A> DerefMut for StackVec<A>
where
    A: Array,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<A> Extend<A::Item> for StackVec<A>
where
    A: Array,
{
    #[inline]
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A::Item>,
    {
        iter.into_iter().for_each(|e| self.push(e));
    }
}

impl<'a, A> IntoIterator for &'a StackVec<A>
where
    A: Array,
{
    type Item = &'a A::Item;
    type IntoIter = Iter<'a, A::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.array.slice().iter()
    }
}
