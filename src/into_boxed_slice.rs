use crate::AsSlice;

use alloc::{boxed::Box, vec::Vec, alloc::Global};

/// A trait for obtaining a boxed slice `[Self::Item]`
#[const_trait]
pub trait IntoBoxedSlice: ~const AsSlice + Sized
{
    /// Yields boxed slice from generic
    fn into_boxed_slice(self) -> Box<[Self::Item]>;
}

impl<T, const N: usize> IntoBoxedSlice for [T; N]
{
    fn into_boxed_slice(self) -> Box<[Self::Item]>
    {
        let mut boxed = Box::new_in(self, Global);
        let ptr = boxed.as_mut_ptr();
        core::mem::forget(boxed);
        unsafe {
            Box::from_raw_in(core::slice::from_raw_parts_mut(ptr, N), Global)
        }
    }
}

impl<T> const IntoBoxedSlice for Box<[T]>
{
    fn into_boxed_slice(self) -> Box<[Self::Item]>
    {
        self
    }
}

impl<T> /*const*/ IntoBoxedSlice for Vec<T>
{
    fn into_boxed_slice(mut self) -> Box<[Self::Item]>
    {
        self.shrink_to_fit();

        let (ptr, len, cap, alloc) = self.into_raw_parts_with_alloc();
        assert_eq!(len, cap, "Memory leak detected");

        unsafe {
            Box::from_raw_in(core::slice::from_raw_parts_mut(ptr, len), alloc)
        }
    }
}