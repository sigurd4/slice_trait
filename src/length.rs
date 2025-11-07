use core::{fmt, hash::Hash, marker::Freeze};

macro_rules! op {
    ($trait:ident 1) => {
        type $trait: LengthValue = <Self as private::${concat(Length, $trait)}>::Output;
    };
    ($trait:ident 2) => {
        type $trait<Rhs: LengthValue>: LengthValue = <Self as private::${concat(Length, $trait)}<Rhs>>::Output;
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
            <X as private::${concat(Length, $trait)}>::$fn(x)
        }
    };
    ($trait:ident::$fn:ident 2) => {
        pub type $trait<Lhs, Rhs> = <Lhs as LengthValue>::$trait<Rhs>;
        pub const fn $fn<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> $trait<Lhs, Rhs>
        where
            Lhs: LengthValue,
            Rhs: LengthValue
        {
            <Lhs as private::${concat(Length, $trait)}<Rhs>>::$fn(lhs, rhs)
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
    type Value: LengthValue<Length<Self::Elem> = Self, _Length<Self::Elem> = Self, Metadata = Self::Metadata, _Metadata = Self::Metadata>;
    type Mapped<U>: Length<Elem = U, Value = Self::Value, _Value = Self::Value, Metadata = Self::Metadata> + ?Sized = value::Length<Self::Value, U>;

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
pub type Value<T> = <T as Length>::Value;
pub type Mapped<T, U> = <T as Length>::Mapped<U>;

pub mod value
{
    use super::*;

    pub type Length<T, U> = <T as LengthValue>::Length<U>;

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
}

pub trait LengthValue: const private::LengthValue
{
    type Length<T>: Length<Elem = T, Value = Self, _Value = Self, Metadata = Self::Metadata> + ?Sized;
    type Metadata: fmt::Debug + Copy + Send + Sync + const Ord + Hash + Unpin + Freeze + const Default;

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
    type Length<U> = Self::_Length<U>;
    type Metadata = Self::_Metadata;
}

mod private
{
    use core::fmt;
    use core::hash::Hash;
    use core::marker::Freeze;
    use core::{marker::Destruct, ptr::Pointee};

    use crate::AsSlice;

    use crate::same::Same;

    macro_rules! op {
        ($trait:ident::$fn:ident($x:ident) $expr:expr) => {
            pub const trait ${concat(Length, $trait)}: const LengthValue
            {
                type Output: const LengthValue;

                fn $fn(x: Self) -> <Self as super::LengthValue>::$trait;
            }
            impl<L> const ${concat(Length, $trait)} for L
            where
                L: const LengthValue
            {
                default type Output = usize;

                default fn $fn(x: Self) -> Self::Output
                {
                    let $x = L::len_metadata(x);
                    $expr.same().ok().unwrap()
                }
            }
            #[allow(non_upper_case_globals)]
            impl<const $x: usize> const ${concat(Length, $trait)} for [(); $x]
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
            pub const trait ${concat(Length, $trait)}<R>: const LengthValue
            where
                R: const LengthValue
            {
                type Output: const LengthValue;

                fn $fn(lhs: Self, rhs: R) -> <Self as super::LengthValue>::$trait<R>;
            }
            impl<L, R> const ${concat(Length, $trait)}<R> for L
            where
                L: const LengthValue,
                R: const LengthValue
            {
                default type Output = usize;

                default fn $fn(lhs: Self, rhs: R) -> Self::Output
                {
                    let $lhs = L::len_metadata(lhs);
                    let $rhs = R::len_metadata(rhs);
                    $expr.same().ok().unwrap()
                }
            }
            #[allow(non_upper_case_globals)]
            impl<const $lhs: usize, const $rhs: usize> const ${concat(Length, $trait)}<[(); $rhs]> for [(); $lhs]
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

    #[rustc_on_unimplemented(
        message = "`{Self}` is not a valid bulk length",
        label = "The only valid lengths are `[_]` or `[_; _]`",
    )]
    pub trait Length: AsSlice + Pointee
    {
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

    pub const trait LengthValue: Copy + const Destruct
    {
        type _Length<T>: Length<Elem = T, _Value = Self, Metadata = Self::_Metadata> + ?Sized;
        type _Metadata: fmt::Debug + Copy + Send + Sync + const Ord + Hash + Unpin + Freeze + const Default;
        
        fn or_len_metadata(n: usize) -> Self;
        fn from_metadata(n: Self::_Metadata) -> Self;
        fn into_metadata(len: Self) -> Self::_Metadata;
        fn len_metadata(len: Self) -> usize;
    }
    impl const LengthValue for usize
    {
        type _Length<T> = [T];
        type _Metadata = usize;

        fn or_len_metadata(n: usize) -> Self
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
        fn len_metadata(len: Self) -> usize
        {
            len
        }
    }
    impl<const N: usize> const LengthValue for [(); N]
    {
        type _Length<T> = [T; N];
        type _Metadata = ();

        fn or_len_metadata(_: usize) -> Self
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
        fn len_metadata(_len: Self) -> usize
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