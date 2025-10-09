use core::ptr::Pointee;

use crate::{private, SlicePrereq, AsSlice};

/// A trait for a slice `[Self::Elem]`
#[const_trait]
pub trait Slice: private::Slice + SlicePrereq<<Self as AsSlice>::Elem> + ~const AsSlice + Pointee<Metadata = usize>
{
    
}

impl<T> const Slice for [T]
{
    
}