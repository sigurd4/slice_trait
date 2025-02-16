use crate::{private, AsSlice, BoxedSlicePrereq, IntoBoxedSlice};

use alloc::boxed::Box;

/// A trait for a boxed slice `[Self::Elem]`
#[const_trait]
pub trait BoxedSlice: private::BoxedSlice + BoxedSlicePrereq<<Self as AsSlice>::Elem> + ~const IntoBoxedSlice
{
    
}

impl<T> const BoxedSlice for Box<[T]>
{
    
}