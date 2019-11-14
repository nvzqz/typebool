//! Boolean operations expressed as traits.

/// The negation of a [`Bool`].
///
/// [`Bool`]: ../trait.Bool.html
pub trait Not {
    /// The result of this operation.
    type Output;
}

/// The logical conjunction (intersection) of two [`Bool`]s.
///
/// [`Bool`]: ../trait.Bool.html
pub trait And<A> {
    /// The result of this operation.
    type Output;
}

/// The logical disjunction (union) of two [`Bool`]s.
///
/// [`Bool`]: ../trait.Bool.html
pub trait Or<A> {
    /// The result of this operation.
    type Output;
}

/// The exclusive disjunction of two [`Bool`]s.
///
/// [`Bool`]: ../trait.Bool.html
pub trait Xor<A> {
    /// The result of this operation.
    type Output;
}
