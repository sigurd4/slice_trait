use crate::AsSlice;

/// A trait for obtaining a boxed slice `[Self::Item]`
#[const_trait]
pub trait IntoBoxedSlice: /*~const*/ AsSlice + Sized
{
    #[cfg(feature = "alloc")]
    /// Yields boxed slice from generic
    fn into_boxed_slice(self) -> alloc::boxed::Box<[Self::Item]>;
}

#[cfg(feature = "alloc")]
impl<T> const IntoBoxedSlice for alloc::boxed::Box<[T]>
{
    fn into_boxed_slice(self) -> alloc::boxed::Box<[Self::Item]>
    {
        self
    }
}