use core::ptr::Thin;

/// A trait for obtaining a slice `[Self::Item]`
pub const trait AsSlice
{
    type Elem: Sized + Thin;

    /// Yields slice from generic
    fn as_slice(&self) -> &[Self::Elem];

    /// Yields mutable slice from generic
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
}

const impl<T> AsSlice for [T]
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

const impl<T, const N: usize> AsSlice for [T; N]
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
const impl<T, A> AsSlice for alloc::vec::Vec<T, A>
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
const impl<T, A> AsSlice for alloc::boxed::Box<T, A>
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