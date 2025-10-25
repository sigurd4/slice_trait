use core::ops::{Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::{private, AsSlice};

/// A trait for a slice `[Self::Elem]`
#[const_trait]
pub trait Slice: private::Slice + ~const AsSlice
    + ~const IndexMut<usize, Output = <[<Self as AsSlice>::Elem] as Index<usize>>::Output>
    + ~const IndexMut<Range<usize>, Output = <[<Self as AsSlice>::Elem] as Index<Range<usize>>>::Output>
    + ~const IndexMut<RangeInclusive<usize>, Output = <[<Self as AsSlice>::Elem] as Index<RangeInclusive<usize>>>::Output>
    + ~const IndexMut<RangeFrom<usize>, Output = <[<Self as AsSlice>::Elem] as Index<RangeFrom<usize>>>::Output>
    + ~const IndexMut<RangeTo<usize>, Output = <[<Self as AsSlice>::Elem] as Index<RangeTo<usize>>>::Output>
    + ~const IndexMut<RangeToInclusive<usize>, Output = <[<Self as AsSlice>::Elem] as Index<RangeToInclusive<usize>>>::Output>
    + ~const IndexMut<RangeFull, Output = <[<Self as AsSlice>::Elem] as Index<RangeFull>>::Output>
{
    
}

impl<T> const Slice for [T]
{
    
}