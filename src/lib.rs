#![cfg_attr(not(test), no_std)]
#![feature(trait_alias)]
#![feature(const_trait_impl)]
#![feature(const_deref)]
#![feature(const_array_as_mut_slice)]
#![feature(allocator_api)]
#![cfg_attr(feature = "alloc", feature(const_vec_string_slice))]

//! A trait for any slice, with item as an associated type.
//! 
//! This crate is a subset of the crate [`slice_ops`](https://crates.io/crates/slice_ops).
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

#[cfg(feature = "alloc")]
extern crate alloc;

moddef::moddef!(
    flat(pub) mod {
        as_slice,
        into_boxed_slice for cfg(feature = "alloc"),
        prereq,
        slice,
        boxed_slice for cfg(feature = "alloc")
    }
);


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
    pub trait Slice: crate::AsSlice {}

    impl<T> Slice for [T] {}

    #[cfg(feature = "alloc")]
    pub trait BoxedSlice: crate::IntoBoxedSlice {}

    #[cfg(feature = "alloc")]
    impl<T> BoxedSlice for alloc::boxed::Box<[T]> {}
}
