use core::{ops::{DerefMut, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive}, ptr::Pointee};

/// Prerequesites for a slice of `T`
pub trait SlicePrereq<T> = ?Sized
    + /* ~const */
    IndexMut<usize, Output = <[T] as Index<usize>>::Output>
    + /* ~const */
    IndexMut<Range<usize>, Output = <[T] as Index<Range<usize>>>::Output>
    + /* ~const */
    IndexMut<RangeInclusive<usize>, Output = <[T] as Index<RangeInclusive<usize>>>::Output>
    + /* ~const */
    IndexMut<RangeFrom<usize>, Output = <[T] as Index<RangeFrom<usize>>>::Output>
    + /* ~const */
    IndexMut<RangeTo<usize>, Output = <[T] as Index<RangeTo<usize>>>::Output>
    + /* ~const */
    IndexMut<RangeToInclusive<usize>, Output = <[T] as Index<RangeToInclusive<usize>>>::Output>
    + /* ~const */
    IndexMut<RangeFull, Output = <[T] as Index<RangeFull>>::Output>
    + Pointee<Metadata = usize>;

/// Prerequesites for a boxed slice of `T`
pub trait BoxedSlicePrereq<T> = Sized
    + DerefMut<Target = [T]>;