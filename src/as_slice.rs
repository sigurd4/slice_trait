/// A trait for obtaining a slice `[Self::Item]`
#[const_trait]
pub trait AsSlice
{
    type Elem: Sized;

    /// Yields slice from generic
    fn as_slice(&self) -> &[Self::Elem];

    /// Yields mutable slice from generic
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
}

impl<T> const AsSlice for [T]
{
    type Elem = T;

    fn as_slice(&self) -> &[Self::Elem]
    {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem]
    {
        self
    }
}

impl<T, const N: usize> const AsSlice for [T; N]
{
    type Elem = T;

    fn as_slice(&self) -> &[Self::Elem]
    {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem]
    {
        self.as_mut_slice()
    }
}

#[cfg(feature = "alloc")]
impl<T, A> const AsSlice for alloc::vec::Vec<T, A>
where
    A: core::alloc::Allocator
{
    type Elem = T;

    fn as_slice(&self) -> &[Self::Elem]
    {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem]
    {
        self.as_mut_slice()
    }
}

#[cfg(feature = "alloc")]
impl<T, A> const AsSlice for alloc::boxed::Box<T, A>
where
    A: core::alloc::Allocator,
    T: ~const AsSlice + ?Sized
{
    type Elem = T::Elem;

    fn as_slice(&self) -> &[Self::Elem]
    {
        (**self).as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem]
    {
        (**self).as_mut_slice()
    }
}