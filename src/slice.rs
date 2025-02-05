use crate::{private, SlicePrereq, AsSlice};

/// A trait for a slice `[Self::Item]`
#[const_trait]
pub trait Slice: private::Slice + SlicePrereq<<Self as AsSlice>::Item> + ~const AsSlice
{
    
}

impl<T> const Slice for [T]
{
    
}