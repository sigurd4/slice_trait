

mod private
{
    use core::{marker::Destruct, ptr::Pointee};

    use crate::AsSlice;

    #[rustc_on_unimplemented(
        message = "`{Self}` is not a valid bulk length",
        label = "The only valid lengths are `[_]` or `[_; _]`",
    )]
    pub const trait Length: AsSlice
    {
        type LengthSpec: const LengthSpec<Length<Self::Elem> = Self, Metadata = <Self as Pointee>::Metadata>;

        fn len_metadata(n: <Self as Pointee>::Metadata) -> usize;
    }
    impl<T> const Length for [T]
    {
        type LengthSpec = usize;

        fn len_metadata(n: <Self as Pointee>::Metadata) -> usize
        {
            n
        }
    }
    impl<T, const N: usize> const Length for [T; N]
    {
        type LengthSpec = [(); N];

        fn len_metadata((): <Self as Pointee>::Metadata) -> usize
        {
            N
        }
    }

    pub const trait LengthSpec: Copy + const Destruct
    {
        type Length<T>: const Length<Elem = T, LengthSpec = Self> + Pointee<Metadata = Self::Metadata> + ?Sized;
        type Metadata: Copy + const Default;
        
        fn or_len_metadata(n: usize) -> Self;
        fn from_metadata(n: Self::Metadata) -> Self;
        fn into_metadata(self) -> Self::Metadata;
        fn len_metadata(self) -> usize;
    }
    impl const LengthSpec for usize
    {
        type Length<T> = [T];
        type Metadata = usize;

        fn or_len_metadata(n: usize) -> Self
        {
            n
        }
        fn from_metadata(n: Self::Metadata) -> Self
        {
            n
        }
        fn into_metadata(self) -> Self::Metadata
        {
            self
        }
        fn len_metadata(self) -> usize
        {
            self
        }
    }
    impl<const N: usize> const LengthSpec for [(); N]
    {
        type Length<T> = [T; N];
        type Metadata = ();

        fn or_len_metadata(_: usize) -> Self
        {
            [(); N]
        }
        fn from_metadata((): Self::Metadata) -> Self
        {
            [(); N]
        }
        fn into_metadata(self) -> Self::Metadata
        {
            
        }
        fn len_metadata(self) -> usize
        {
            N
        }
    }

    pub const trait LengthMin<R>: LengthSpec
    where
        R: LengthSpec
    {
        type LengthMin: LengthSpec;

        fn len_min(self, other: R) -> Self::LengthMin;
    }
    impl<L, R> const LengthMin<R> for L
    where
        L: ~const LengthSpec,
        R: ~const LengthSpec
    {
        default type LengthMin = usize;

        default fn len_min(self, other: R) -> Self::LengthMin
        {
            self.len_metadata().min(other.len_metadata()).same().ok().unwrap()
        }
    }
    impl<const M: usize, const N: usize> const LengthMin<[(); N]> for [(); M]
    where
        [(); M.min(N)]:
    {
        type LengthMin = [(); M.min(N)];

        fn len_min(self, _: [(); N]) -> Self::LengthMin
        {
            [(); M.min(N)]
        }
    }

    pub const trait LengthMax<R>: LengthSpec
    where
        R: LengthSpec
    {
        type LengthMax: LengthSpec;

        fn len_max(self, other: R) -> Self::LengthMax;
    }
    impl<L, R> const LengthMax<R> for L
    where
        L: ~const LengthSpec,
        R: ~const LengthSpec
    {
        default type LengthMax = usize;

        default fn len_max(self, other: R) -> Self::LengthMax
        {
            self.len_metadata().max(other.len_metadata()).same().ok().unwrap()
        }
    }
    impl<const M: usize, const N: usize> const LengthMax<[(); N]> for [(); M]
    where
        [(); M.max(N)]:
    {
        type LengthMax = [(); M.max(N)];

        fn len_max(self, _: [(); N]) -> Self::LengthMax
        {
            [(); M.max(N)]
        }
    }

    pub const trait LengthSatAdd<R>: LengthSpec
    where
        R: LengthSpec
    {
        type LengthSatAdd: LengthSpec;

        fn len_sat_add(self, other: R) -> Self::LengthSatAdd;
    }
    impl<L, R> const LengthSatAdd<R> for L
    where
        L: ~const LengthSpec,
        R: ~const LengthSpec
    {
        default type LengthSatAdd = usize;

        default fn len_sat_add(self, other: R) -> Self::LengthSatAdd
        {
            self.len_metadata().saturating_add(other.len_metadata()).same().ok().unwrap()
        }
    }
    impl<const M: usize, const N: usize> const LengthSatAdd<[(); N]> for [(); M]
    where
        [(); M.saturating_add(N)]:
    {
        type LengthSatAdd = [(); M.saturating_add(N)];

        fn len_sat_add(self, _: [(); N]) -> Self::LengthSatAdd
        {
            [(); M.saturating_add(N)]
        }
    }

    pub const trait LengthSatSub<R>: LengthSpec
    where
        R: LengthSpec
    {
        type LengthSatSub: LengthSpec;

        fn len_sat_sub(self, other: R) -> Self::LengthSatSub;
    }
    impl<L, R> const LengthSatSub<R> for L
    where
        L: ~const LengthSpec,
        R: ~const LengthSpec
    {
        default type LengthSatSub = usize;

        default fn len_sat_sub(self, other: R) -> Self::LengthSatSub
        {
            self.len_metadata().saturating_sub(other.len_metadata()).same().ok().unwrap()
        }
    }
    impl<const M: usize, const N: usize> const LengthSatSub<[(); N]> for [(); M]
    where
        [(); M.saturating_sub(N)]:
    {
        type LengthSatSub = [(); M.saturating_sub(N)];

        fn len_sat_sub(self, _: [(); N]) -> Self::LengthSatSub
        {
            [(); M.saturating_sub(N)]
        }
    }

    pub const trait LengthSatMul<R>: LengthSpec
    where
        R: LengthSpec
    {
        type LengthSatMul: LengthSpec;

        fn len_sat_mul(self, other: R) -> Self::LengthSatMul;
    }
    impl<L, R> const LengthSatMul<R> for L
    where
        L: ~const LengthSpec,
        R: ~const LengthSpec
    {
        default type LengthSatMul = usize;

        default fn len_sat_mul(self, other: R) -> Self::LengthSatMul
        {
            self.len_metadata().saturating_mul(other.len_metadata()).same().ok().unwrap()
        }
    }
    impl<const M: usize, const N: usize> const LengthSatMul<[(); N]> for [(); M]
    where
        [(); M.saturating_mul(N)]:
    {
        type LengthSatMul = [(); M.saturating_mul(N)];

        fn len_sat_mul(self, _: [(); N]) -> Self::LengthSatMul
        {
            [(); M.saturating_mul(N)]
        }
    }

    pub const trait LengthMul<R>: LengthSpec
    where
        R: LengthSpec
    {
        type LengthMul: LengthSpec;

        fn len_mul(self, other: R) -> Self::LengthMul;
    }
    impl<L, R> const LengthMul<R> for L
    where
        L: ~const LengthSpec,
        R: ~const LengthSpec
    {
        default type LengthMul = usize;

        default fn len_mul(self, other: R) -> Self::LengthMul
        {
            (self.len_metadata()*other.len_metadata()).same().ok().unwrap()
        }
    }
    impl<const M: usize, const N: usize> const LengthMul<[(); N]> for [(); M]
    where
        [(); M*N]:
    {
        type LengthMul = [(); M*N];

        fn len_mul(self, _: [(); N]) -> Self::LengthMul
        {
            [(); M*N]
        }
    }

    pub const trait LengthDivCeil<R>: LengthSpec
    where
        R: LengthSpec
    {
        type LengthDivCeil: LengthSpec;

        fn len_div_ceil(self, other: R) -> Self::LengthDivCeil;
    }
    impl<L, R> const LengthDivCeil<R> for L
    where
        L: ~const LengthSpec,
        R: ~const LengthSpec
    {
        default type LengthDivCeil = usize;

        default fn len_div_ceil(self, other: R) -> Self::LengthDivCeil
        {
            self.len_metadata().div_ceil(other.len_metadata()).same().ok().unwrap()
        }
    }
    impl<const M: usize, const N: usize> const LengthDivCeil<[(); N]> for [(); M]
    where
        [(); M.div_ceil(N)]:
    {
        type LengthDivCeil = [(); M.div_ceil(N)];

        fn len_div_ceil(self, _: [(); N]) -> Self::LengthDivCeil
        {
            [(); M.div_ceil(N)]
        }
    }

    pub const trait LengthDiv<R>: LengthSpec
    where
        R: LengthSpec
    {
        type LengthDiv: LengthSpec;

        fn len_div(self, other: R) -> Self::LengthDiv;
    }
    impl<L, R> const LengthDiv<R> for L
    where
        L: ~const LengthSpec,
        R: ~const LengthSpec
    {
        default type LengthDiv = usize;

        default fn len_div(self, other: R) -> Self::LengthDiv
        {
            (self.len_metadata()/other.len_metadata()).same().ok().unwrap()
        }
    }
    impl<const M: usize, const N: usize> const LengthDiv<[(); N]> for [(); M]
    where
        [(); M/N]:
    {
        type LengthDiv = [(); M/N];

        fn len_div(self, _: [(); N]) -> Self::LengthDiv
        {
            [(); M/N]
        }
    }

    pub const trait LengthInterspersed: LengthSpec
    {
        type LengthInterspersed: LengthSpec;

        fn len_interspersed(self) -> Self::LengthInterspersed;
    }
    impl<L> const LengthInterspersed for L
    where
        L: ~const LengthSpec
    {
        default type LengthInterspersed = usize;

        default fn len_interspersed(self) -> Self::LengthInterspersed
        {
            let n = self.len_metadata();
            (n + n.saturating_sub(1)).same().ok().unwrap()
        }
    }
    impl<const N: usize> const LengthInterspersed for [(); N]
    where
        [(); N + N.saturating_sub(1)]:
    {
        type LengthInterspersed = [(); N + N.saturating_sub(1)];

        fn len_interspersed(self) -> Self::LengthInterspersed
        {
            [(); N + N.saturating_sub(1)]
        }
    }

    pub const trait LengthWindowed<const N: usize>: LengthSpec
    {
        type LengthWindowed: LengthSpec;

        fn len_interspersed(self) -> Self::LengthWindowed;
    }
    impl<L, const N: usize> const LengthWindowed<N> for L
    where
        L: ~const LengthSpec
    {
        default type LengthWindowed = usize;

        default fn len_interspersed(self) -> Self::LengthWindowed
        {
            let m = self.len_metadata();
            m.saturating_sub(N - 1).same().ok().unwrap()
        }
    }
    impl<const N: usize, const M: usize> const LengthWindowed<N> for [(); M]
    where
        [(); M.saturating_sub(N - 1)]:
    {
        type LengthWindowed = [(); M.saturating_sub(N - 1)];

        fn len_interspersed(self) -> Self::LengthWindowed
        {
            [(); M.saturating_sub(N - 1)]
        }
    }
}