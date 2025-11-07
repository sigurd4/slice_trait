#![cfg_attr(not(test), no_std)]
#![feature(trait_alias)]
#![feature(const_trait_impl)]
#![feature(ptr_metadata)]
#![feature(const_index)]
#![feature(const_convert)]
#![cfg_attr(feature = "alloc", feature(allocator_api))]

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
//! const fn first<'a, S: ~const Slice + ?Sized>(slice: &'a S) -> Option<&'a S::Elem>
//! where
//!     S::Elem: Copy,
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
        slice,
        boxed_slice for cfg(feature = "alloc")
    },
    pub mod {
        same for cfg(feature = "same"),
        length for cfg(feature = "length")
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

        const fn first<'a, S: [const] Slice + ?Sized>(slice: &'a S) -> Option<&'a S::Elem>
        where
            S::Elem: Copy
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
