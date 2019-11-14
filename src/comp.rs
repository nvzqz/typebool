//! Boolean operations expressed as composed concrete types.

use crate::Bool;

use core::marker::PhantomData;
use crate::ops;

/// The negation of a [`Bool`].
///
/// [`Bool`]: ../trait.Bool.html
pub struct Not<A> {
    _marker: PhantomData<A>
}

impl<A: Bool> Bool for Not<A> {
    const VALUE: bool = !A::VALUE;
}

impl<A> ops::Not for Not<A> {
    type Output = A;
}

/// The logical conjunction (intersection) of two [`Bool`]s.
///
/// [`Bool`]: ../trait.Bool.html
pub struct And<A, B> {
    _marker: PhantomData<(A, B)>
}

impl<A: Bool, B: Bool> Bool for And<A, B> {
    const VALUE: bool = A::VALUE & B::VALUE;
}

impl<A, B> ops::Not for And<A, B> {
    // DeMorgan's law: !(A & B) === !A | !B
    type Output = Or<Not<A>, Not<B>>;
}

/// The logical disjunction (union) of two [`Bool`]s.
///
/// [`Bool`]: ../trait.Bool.html
pub struct Or<A, B> {
    _marker: PhantomData<(A, B)>
}

impl<A: Bool, B: Bool> Bool for Or<A, B> {
    const VALUE: bool = A::VALUE | B::VALUE;
}

impl<A, B> ops::Not for Or<A, B> {
    // DeMorgan's law: !(A | B) === !A & !B
    type Output = And<Not<A>, Not<B>>;
}

/// The exclusive disjunction of two [`Bool`]s.
///
/// [`Bool`]: ../trait.Bool.html
pub struct Xor<A, B> {
    _marker: PhantomData<(A, B)>
}

impl<A: Bool, B: Bool> Bool for Xor<A, B> {
    const VALUE: bool = A::VALUE != B::VALUE;
}
