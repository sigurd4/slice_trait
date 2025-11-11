use core::ops::DerefMut;

use crate::{private, AsSlice, IntoBoxedSlice};

use alloc::boxed::Box;

/// A trait for a boxed slice `[Self::Elem]`
pub const trait BoxedSlice: private::BoxedSlice + /*~const*/ DerefMut<Target = [<Self as AsSlice>::Elem]> + ~const IntoBoxedSlice
{
    
}

impl<T> const BoxedSlice for Box<[T]>
{
    
}