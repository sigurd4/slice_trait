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

impl<T, const N: usize> const AsSlice for [T; N]
{
    type Item = T;

    fn as_slice(&self) -> &[Self::Item]
    {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item]
    {
        self.as_mut_slice()
    }
}

#[cfg(feature = "alloc")]
impl<T, A> const AsSlice for alloc::vec::Vec<T, A>
where
    A: core::alloc::Allocator
{
    type Item = T;

    fn as_slice(&self) -> &[Self::Item]
    {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item]
    {
        self.as_mut_slice()
    }
}

#[cfg(feature = "alloc")]
impl<T, A> const AsSlice for alloc::boxed::Box<[T], A>
where
    A: core::alloc::Allocator
{
    type Item = T;

    fn as_slice(&self) -> &[Self::Item]
    {
        &**self
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item]
    {
        &mut **self
    }
}