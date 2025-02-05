use crate::{private, AsSlice, BoxedSlicePrereq, IntoSlice};

use alloc::boxed::Box;

/// A trait for a boxed slice `[Self::Item]`
#[const_trait]
pub trait BoxedSlice: private::BoxedSlice + BoxedSlicePrereq<<Self as AsSlice>::Item> + ~const IntoSlice
{
    
}

impl<T> const BoxedSlice for Box<[T]>
{
    
}