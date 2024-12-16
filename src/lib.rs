#![cfg_attr(not(test), no_std)]
#![feature(trait_alias)]
#![feature(const_trait_impl)]

//! A trait for any slice, with item as an associated type.
//!
//! # Example
//!
//! ```rust
//! #![feature(const_trait_impl)]
//!
//! use slice_trait::*;
//!
//! const A: &[i32] = [1, 2, 3].as_slice();
//!
//! const fn first<'a, S: ~const Slice + ?Sized>(slice: &'a S) -> Option<&'a S::Item>
//! where
//!     S::Item: Copy,
//! {
//!     slice.as_slice().first()
//! }
//!
//! assert_eq!(first(A), Some(&1));
//! ```

use core::ops::{Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Prerequesites for a slice of `T`
pub trait SlicePrereq<T> = ?Sized
    + /* ~const */
    Index<usize, Output = <[T] as Index<usize>>::Output>
    + /* ~const */
    Index<Range<usize>, Output = <[T] as Index<Range<usize>>>::Output>
    + /* ~const */
    Index<RangeInclusive<usize>, Output = <[T] as Index<RangeInclusive<usize>>>::Output>
    + /* ~const */
    Index<RangeFrom<usize>, Output = <[T] as Index<RangeFrom<usize>>>::Output>
    + /* ~const */
    Index<RangeTo<usize>, Output = <[T] as Index<RangeTo<usize>>>::Output>
    + /* ~const */
    Index<RangeToInclusive<usize>, Output = <[T] as Index<RangeToInclusive<usize>>>::Output>
    + /* ~const */
    Index<RangeFull, Output = <[T] as Index<RangeFull>>::Output>
    + /* ~const */
    IndexMut<usize>
    + /* ~const */
    IndexMut<Range<usize>>
    + /* ~const */
    IndexMut<RangeInclusive<usize>>
    + /* ~const */
    IndexMut<RangeFrom<usize>>
    + /* ~const */
    IndexMut<RangeTo<usize>>
    + /* ~const */
    IndexMut<RangeToInclusive<usize>>
    + /* ~const */
    IndexMut<RangeFull>;

/// A trait for a slice `[Self::Item]`
#[const_trait]
pub trait Slice: private::Slice + SlicePrereq<<Self as Slice>::Item>
{
    type Item;

    /// Yields slice from generic
    fn as_slice(&self) -> &[Self::Item];

    /// Yields mutable slice from generic
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

impl<T> const Slice for [T]
{
    type Item = T;

    fn as_slice(&self) -> &[Self::Item]
    {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item]
    {
        self
    }
}

#[cfg(test)]
mod test
{
    use crate::*;

    #[test]
    fn test()
    {
        const A: &[i32] = [1, 2, 3].as_slice();

        const fn first<'a, S: ~const Slice + ?Sized>(slice: &'a S) -> Option<&'a S::Item>
        where
            S::Item: Copy
        {
            slice.as_slice().first()
        }

        assert_eq!(first(A), Some(&1));
    }
}

mod private
{
    pub trait Slice {}

    impl<T> Slice for [T] {}
}
