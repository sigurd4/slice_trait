/// A trait for obtaining a slice `[Self::Item]`
#[const_trait]
pub trait AsSlice
{
    type Item: Sized;

    /// Yields slice from generic
    fn as_slice(&self) -> &[Self::Item];

    /// Yields mutable slice from generic
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

impl<T> const AsSlice for [T]
{
    type Item = T;

    fn as_slice(&self) -> &[Self::Item]
    {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item]
    {
        self
    }
}

#[cfg(feature = "alloc")]
impl<T> const AsSlice for alloc::boxed::Box<[T]>
{
    type Item = T;

    fn as_slice(&self) -> &[Self::Item]
    {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item]
    {
        self
    }
}