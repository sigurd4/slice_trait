use core::{fmt, hash::Hash, marker::{Destruct, Freeze}};

macro_rules! op {
    ($trait:ident 1) => {
        type $trait: LengthValue = <Self as ops::${concat(Length, $trait)}>::Output;
    };
    ($trait:ident 2) => {
        type $trait<Rhs: LengthValue>: LengthValue = <Self as ops::${concat(Length, $trait)}<Rhs>>::Output;
    };
    ($trait:ident [1]) => {
        type $trait: Length<Elem = Self::Elem> + ?Sized = value::Length<value::$trait<Self::Value>, Self::Elem>;
    };
    ($trait:ident [2]) => {
        type $trait<Rhs: Length<Elem = Self::Elem> + ?Sized>: Length<Elem = Self::Elem> + ?Sized = value::Length<value::$trait<Self::Value, Rhs::Value>, Self::Elem>;
    };
    ($trait:ident::$fn:ident 1) => {
        pub type $trait<X> = <X as LengthValue>::$trait;
        pub const fn $fn<X>(x: X) -> $trait<X>
        where
            X: LengthValue
        {
            <X as ops::${concat(Length, $trait)}>::$fn(x)
        }
    };
    ($trait:ident::$fn:ident 2) => {
        pub type $trait<Lhs, Rhs> = <Lhs as LengthValue>::$trait<Rhs>;
        pub const fn $fn<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> $trait<Lhs, Rhs>
        where
            Lhs: LengthValue,
            Rhs: LengthValue
        {
            <Lhs as ops::${concat(Length, $trait)}<Rhs>>::$fn(lhs, rhs)
        }
    };
    (pub $trait:ident [1]) => {
        pub type $trait<X> = <X as Length>::$trait;
    };
    (pub $trait:ident [2]) => {
        pub type $trait<Lhs, Rhs> = <Lhs as Length>::$trait<Rhs>;
    };
}

#[rustc_on_unimplemented(
    message = "`{Self}` is not a valid bulk length",
    label = "The only valid lengths are `[_]` or `[_; _]`",
)]
pub trait Length: private::Length<_Value = Self::Value>
{
    type Value: LengthValue<Length<Self::Elem> = Self, _Length<Self::Elem> = Self, Metadata = Self::Metadata, _Metadata = Self::Metadata>
        + LengthIntersect<Self::_Value, Output = Self::_Value>
        /*+ LengthIntersect<usize, Output = usize>
        + LengthMin<Self::_Value, Output = Self::_Value>
        + LengthMin<usize, Output = usize>
        + LengthMax<Self::_Value, Output = Self::_Value>
        + LengthMax<[(); 0], Output = Self::_Value>
        + LengthMax<usize, Output = usize>
        + LengthAdd<[(); 0], Output = Self::_Value>
        + LengthAdd<usize, Output = usize>
        + LengthSub<[(); 0], Output = Self::_Value>
        + LengthSub<usize, Output = usize>
        + LengthMul<[(); 1], Output = Self::_Value>
        + LengthMul<usize, Output = usize>
        + LengthDiv<[(); 1], Output = Self::_Value>
        + LengthDiv<usize, Output = usize>
        + LengthSaturatingAdd<[(); 0], Output = Self::_Value>
        + LengthSaturatingAdd<usize, Output = usize>
        + LengthSaturatingSub<[(); 0], Output = Self::_Value>
        + LengthSaturatingSub<usize, Output = usize>
        + LengthSaturatingMul<[(); 1], Output = Self::_Value>
        + LengthSaturatingMul<usize, Output = usize>
        + LengthSaturatingDiv<[(); 1], Output = Self::_Value>
        + LengthSaturatingDiv<usize, Output = usize>
        + LengthDivCeil<[(); 1], Output = Self::_Value>
        + LengthDivCeil<usize, Output = usize>
        + LengthDivFloor<[(); 1], Output = Self::_Value>
        + LengthDivFloor<usize, Output = usize>
        + LengthWindowed<[(); 1], Output = Self::_Value>*/
        ;
    type Mapped<U>: Length<Elem = U, Value = Self::Value, _Value = Self::Value, Metadata = Self::Metadata> + ?Sized = value::Length<Self::Value, U>;

    op!(Intersect [2]);
    op!(Min [2]);
    op!(Max [2]);
    op!(Add [2]);
    op!(Sub [2]);
    op!(Mul [2]);
    op!(Div [2]);
    op!(Rem [2]);
    op!(SaturatingAdd [2]);
    op!(SaturatingSub [2]);
    op!(SaturatingMul [2]);
    op!(SaturatingDiv [2]);
    op!(DivCeil [2]);
    op!(DivFloor [2]);
    op!(Windowed [2]);
    op!(Interspersed [1]);
}
impl<T> Length for T
where
    T: private::Length + ?Sized
{
    type Value = Self::_Value;
}

op!(pub Intersect [2]);
op!(pub Min [2]);
op!(pub Max [2]);
op!(pub Add [2]);
op!(pub Sub [2]);
op!(pub Mul [2]);
op!(pub Div [2]);
op!(pub Rem [2]);
op!(pub SaturatingAdd [2]);
op!(pub SaturatingSub [2]);
op!(pub SaturatingMul [2]);
op!(pub SaturatingDiv [2]);
op!(pub DivCeil [2]);
op!(pub DivFloor [2]);
op!(pub Windowed [2]);
op!(pub Interspersed [1]);

pub use crate::Elem;
use crate::length::ops::*;
pub type Value<T> = <T as Length>::Value;
pub type Mapped<T, U> = <T as Length>::Mapped<U>;

pub mod value
{
    use core::cmp::Ordering;

    use super::*;

    pub type Length<T, U> = <T as LengthValue>::Length<U>;

    pub const fn from_metadata<T>(n: T::Metadata) -> T
    where
        T: LengthValue
    {
        <T as private::LengthValue>::from_metadata(n)
    }
    pub const fn into_metadata<T>(len: T) -> T::Metadata
    where
        T: LengthValue
    {
        <T as private::LengthValue>::into_metadata(len)
    }
    pub const fn or_len<T>(n: usize) -> T
    where
        T: LengthValue
    {
        <T as private::LengthValue>::or_len(n)
    }
    pub const fn len<T>(len: T) -> usize
    where
        T: LengthValue
    {
        <T as private::LengthValue>::len(len)
    }
    pub const fn len_metadata<T>(len: T::Metadata) -> usize
    where
        T: LengthValue
    {
        self::len(from_metadata::<T>(len))
    }

    pub type Intersect<X, Y> = <X as LengthValue>::Intersect<Y>;
    op!(Min::min 2);
    op!(Max::max 2);
    op!(Add::add 2);
    op!(Sub::sub 2);
    op!(Mul::mul 2);
    op!(Div::div 2);
    op!(Rem::rem 2);
    op!(SaturatingAdd::saturating_add 2);
    op!(SaturatingSub::saturating_sub 2);
    op!(SaturatingMul::saturating_mul 2);
    op!(SaturatingDiv::saturating_div 2);
    op!(DivCeil::div_ceil 2);
    op!(DivFloor::div_floor 2);
    op!(Windowed::windowed 2);
    op!(Interspersed::interspersed 1);

    macro_rules! cmp_op {
        ($($op:ident)* -> $ret:ty) => {
            $(
                pub const fn $op<L, R>(lhs: L, rhs: R) -> $ret
                where
                    L: LengthValue,
                    R: LengthValue
                {
                    usize::$op(&len(lhs), &len(rhs))
                }
            )*
        };
    }
    
    cmp_op!(cmp -> Ordering);
    cmp_op!(eq ne gt lt ge le -> bool);

    macro_rules! checked_op {
        ($($trait:ident::$fn:ident)*) => {
            $(
                pub const fn ${concat(checked_, $fn)}<L, R>(lhs: L, rhs: R) -> Option<$trait<L, R>>
                where
                    L: LengthValue,
                    R: LengthValue
                {
                    match usize::${concat(checked_, $fn)}(len(lhs), len(rhs))
                    {
                        Some(_) => Some($fn(lhs, rhs)),
                        None => None
                    }
                }
            )*
        };
    }
    checked_op!(Sub::sub Add::add Mul::mul Div::div Rem::rem);
}

pub const fn as_metadata<T>(len: &T) -> T::Metadata
where
    T: Length + ?Sized
{
    core::ptr::metadata(len)
}
pub const fn as_value<T>(len: &T) -> T::Value
where
    T: Length + ?Sized
{
    value::from_metadata(as_metadata(len))
}
pub const fn len<T>(len: &T) -> usize
where
    T: Length + ?Sized
{
    value::len(as_value(len))
}
pub const fn len_metadata<T>(metadata: T::Metadata) -> usize
where
    T: Length + ?Sized
{
    value::len_metadata::<T::Value>(metadata)
}
pub const fn from_raw_parts<'a, T>(ptr: *const T::Elem, metadata: T::Metadata) -> &'a T
where
    T: Length + ?Sized
{
    unsafe {
        ptr_from_raw_parts::<T>(ptr, metadata).as_ref().unwrap()
    }
}
pub const fn from_raw_parts_mut<'a, T>(ptr: *mut T::Elem, metadata: T::Metadata) -> &'a mut T
where
    T: Length + ?Sized
{
    unsafe {
        ptr_from_raw_parts_mut::<T>(ptr, metadata).as_mut().unwrap()
    }
}
pub const fn from_ptr_len<'a, T, L>(ptr: *const T, len: L) -> &'a L::Length<T>
where
    L: LengthValue,
    T: 'a
{
    unsafe {
        ptr_from_ptr_len(ptr, len).as_ref().unwrap()
    }
}
pub const fn from_mut_ptr_len<'a, T, L>(ptr: *mut T, len: L) -> &'a mut L::Length<T>
where
    L: LengthValue,
    T: 'a
{
    unsafe {
        ptr_from_mut_ptr_len(ptr, len).as_mut().unwrap()
    }
}
pub const fn ptr_from_raw_parts<T>(ptr: *const T::Elem, metadata: T::Metadata) -> *const T
where
    T: Length + ?Sized
{
    core::ptr::from_raw_parts::<T>(ptr, metadata)
}
pub const fn ptr_from_raw_parts_mut<T>(ptr: *mut T::Elem, metadata: T::Metadata) -> *mut T
where
    T: Length + ?Sized
{
    core::ptr::from_raw_parts_mut::<T>(ptr, metadata)
}
pub const fn ptr_from_ptr_len<'a, T, L>(ptr: *const T, len: L) -> *const L::Length<T>
where
    L: LengthValue,
    T: 'a
{
    from_raw_parts(ptr, value::into_metadata(len))
}
pub const fn ptr_from_mut_ptr_len<'a, T, L>(ptr: *mut T, len: L) -> *mut L::Length<T>
where
    L: LengthValue,
    T: 'a
{
    from_raw_parts_mut(ptr, value::into_metadata(len))
}

pub trait LengthValue: const private::LengthValue<_Length<()> = Self::Length<()>, _Metadata = Self::Metadata>
{
    type Length<T>: Length<Elem = T, Value = Self, _Value = Self, Metadata = Self::Metadata,
        Intersect<Self::Length<T>> = Self::Length<T>,
        /*Intersect<[T]> = [T],
        Min<Self::Length<T>> = Self::Length<T>,
        Min<[T]> = [T],
        Max<Self::Length<T>> = Self::Length<T>,
        Max<[T; 0]> = Self::Length<T>,
        Max<[T]> = [T],
        Add<[T; 0]> = Self::Length<T>,
        Add<[T]> = [T],
        Sub<[T; 0]> = Self::Length<T>,
        Sub<[T]> = [T],
        Mul<[T; 1]> = Self::Length<T>,
        Mul<[T]> = [T],
        Div<[T; 1]> = Self::Length<T>,
        Div<[T]> = [T],
        SaturatingAdd<[T; 0]> = Self::Length<T>,
        SaturatingAdd<[T]> = [T],
        SaturatingSub<[T; 0]> = Self::Length<T>,
        SaturatingSub<[T]> = [T],
        SaturatingMul<[T; 1]> = Self::Length<T>,
        SaturatingMul<[T]> = [T],
        SaturatingDiv<[T; 1]> = Self::Length<T>,
        SaturatingDiv<[T]> = [T],
        DivCeil<[T; 1]> = Self::Length<T>,
        DivCeil<[T]> = [T],
        DivFloor<[T; 1]> = Self::Length<T>,
        DivFloor<[T]> = [T],
        Windowed<[T; 1]> = Self::Length<T>*/
    > + ?Sized;
    type Metadata: fmt::Debug + Copy + Send + Sync + const Ord + Hash + Unpin + Freeze + const Default + const Destruct + 'static;

    op!(Intersect 2);
    op!(Min 2);
    op!(Max 2);
    op!(Add 2);
    op!(Sub 2);
    op!(Mul 2);
    op!(Div 2);
    op!(Rem 2);
    op!(SaturatingAdd 2);
    op!(SaturatingSub 2);
    op!(SaturatingMul 2);
    op!(SaturatingDiv 2);
    op!(DivCeil 2);
    op!(DivFloor 2);
    op!(Windowed 2);
    op!(Interspersed 1);
}
impl<T> LengthValue for T
where
    T: const private::LengthValue
{
    type Length<U> = <Self as private::LengthValue>::_Length<U>;
    type Metadata = <Self as private::LengthValue>::_Metadata;
}

mod ops
{
    use super::*;

    use crate::same::Same;

    #[doc(hidden)]
    pub trait LengthIntersect<R>: LengthValue
    where
        R: LengthValue
    {
        #[doc(hidden)]
        type Output: LengthValue;
    }
    impl<L, R> LengthIntersect<R> for L
    where
        L: LengthValue,
        R: LengthValue
    {
        #[doc(hidden)]
        default type Output = usize;
    }
    impl<L> LengthIntersect<L> for L
    where
        L: LengthValue
    {
        #[doc(hidden)]
        type Output = L;
    }

    macro_rules! op {
        ($trait:ident::$fn:ident($x:ident) $expr:expr) => {
            #[doc(hidden)]
            pub const trait ${concat(Length, $trait)}: LengthValue
            {
                #[doc(hidden)]
                type Output: LengthValue;

                #[doc(hidden)]
                fn $fn(x: Self) -> <Self as super::LengthValue>::$trait;
            }
            const impl<L> ${concat(Length, $trait)} for L
            where
                L: LengthValue
            {
                default type Output = usize;

                default fn $fn(x: Self) -> Self::$trait
                {
                    let $x = L::len(x);
                    $expr.same().ok().unwrap()
                }
            }
            #[allow(non_upper_case_globals)]
            const impl<const $x: usize> ${concat(Length, $trait)} for [(); $x]
            where
                [(); $expr]:
            {
                type Output = [(); $expr];

                fn $fn(_: Self) -> Self::Output
                {
                    [(); $expr]
                }
            }
        };
        ($trait:ident::$fn:ident($lhs:ident, $rhs:ident) $expr:expr) => {
            #[doc(hidden)]
            pub const trait ${concat(Length, $trait)}<R>: LengthValue
            where
                R: LengthValue
            {
                #[doc(hidden)]
                type Output: LengthValue;

                #[doc(hidden)]
                fn $fn(lhs: Self, rhs: R) -> <Self as super::LengthValue>::$trait<R>;
            }
            const impl<L, R> ${concat(Length, $trait)}<R> for L
            where
                L: LengthValue,
                R: LengthValue
            {
                default type Output = usize;

                default fn $fn(lhs: Self, rhs: R) -> Self::$trait<R>
                {
                    let $lhs = L::len(lhs);
                    let $rhs = R::len(rhs);
                    $expr.same().ok().unwrap()
                }
            }
            #[allow(non_upper_case_globals)]
            const impl<const $lhs: usize, const $rhs: usize> ${concat(Length, $trait)}<[(); $rhs]> for [(); $lhs]
            where
                [(); $expr]:
            {
                type Output = [(); $expr];

                fn $fn(_: Self, _: [(); $rhs]) -> Self::Output
                {
                    [(); $expr]
                }
            }
        };
    }

    op!(Min::min(a, b) Ord::min(a, b));
    op!(Max::max(a, b) Ord::max(a, b));
    op!(Add::add(a, b) a + b);
    op!(Sub::sub(a, b) a - b);
    op!(Mul::mul(a, b) a*b);
    op!(Div::div(a, b) a/b);
    op!(Rem::rem(a, b) a % b);
    op!(SaturatingAdd::saturating_add(a, b) a.saturating_add(b));
    op!(SaturatingSub::saturating_sub(a, b) a.saturating_sub(b));
    op!(SaturatingMul::saturating_mul(a, b) a.saturating_mul(b));
    op!(SaturatingDiv::saturating_div(a, b) a.saturating_div(b));
    op!(DivCeil::div_ceil(a, b) a.div_ceil(b));
    op!(DivFloor::div_floor(a, b) a.div_floor(b));
    op!(Windowed::windowed(a, b) a.saturating_sub(b - 1));
    op!(Interspersed::interspersed(a) a + a.saturating_sub(1));
}

mod private
{
    use core::fmt;
    use core::hash::Hash;
    use core::marker::Freeze;
    use core::{marker::Destruct, ptr::Pointee};

    use crate::AsSlice;

    #[doc(hidden)]
    #[rustc_on_unimplemented(
        message = "`{Self}` is not a valid length",
        label = "The only valid lengths are `[_]` or `[_; _]`",
    )]
    pub trait Length: AsSlice + Pointee
    {
        #[doc(hidden)]
        type _Value: const LengthValue<_Length<Self::Elem> = Self, _Metadata = Self::Metadata>;
    }
    impl<T> Length for [T]
    {
        type _Value = usize;
    }
    impl<T, const N: usize> Length for [T; N]
    {
        type _Value = [(); N];
    }

    #[doc(hidden)]
    pub const trait LengthValue: Copy + const Destruct + 'static
    {
        #[doc(hidden)]
        type _Length<T>: Length<Elem = T, _Value = Self, Metadata = Self::_Metadata> + ?Sized;
        #[doc(hidden)]
        type _Metadata: fmt::Debug + Copy + Send + Sync + const Ord + Hash + Unpin + Freeze + const Default + const Destruct + 'static;
        
        fn or_len(n: usize) -> Self;
        fn from_metadata(n: Self::_Metadata) -> Self;
        fn into_metadata(len: Self) -> Self::_Metadata;
        fn len(len: Self) -> usize;
    }
    const impl LengthValue for usize
    {
        type _Length<T> = [T];
        type _Metadata = usize;

        fn or_len(n: usize) -> Self
        {
            n
        }
        fn from_metadata(n: Self::_Metadata) -> Self
        {
            n
        }
        fn into_metadata(len: Self) -> Self::_Metadata
        {
            len
        }
        fn len(len: Self) -> usize
        {
            len
        }
    }
    const impl<const N: usize> LengthValue for [(); N]
    {
        type _Length<T> = [T; N];
        type _Metadata = ();

        fn or_len(_: usize) -> Self
        {
            [(); N]
        }
        fn from_metadata((): Self::_Metadata) -> Self
        {
            [(); N]
        }
        fn into_metadata(_len: Self) -> Self::_Metadata
        {
            
        }
        fn len(_len: Self) -> usize
        {
            N
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::length::Sub;

    #[allow(unused)]
    const TEST: Sub<[(); 5], [(); 2]> = [(), (), ()];
}