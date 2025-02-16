use crate::{private, SlicePrereq, AsSlice};

/// A trait for a slice `[Self::Elem]`
#[const_trait]
pub trait Slice: private::Slice + SlicePrereq<<Self as AsSlice>::Elem> + ~const AsSlice
{
    
}

impl<T> const Slice for [T]
{
    
}