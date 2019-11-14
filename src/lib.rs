//! Type-level [booleans][boolean] meant for compile-time hackery.
//!
//! [boolean]: https://en.m.wikipedia.org/wiki/Boolean

#![no_std]
#![deny(missing_docs)]

pub mod comp;
pub mod ops;

/// A type that expresses a boolean value.
pub trait Bool {
    /// The runtime boolean value.
    const VALUE: bool;
}

/// `true` as a dedicated type.
pub struct True;

impl Bool for True {
    const VALUE: bool = true;
}

impl ops::Not for True {
    type Output = False;
}

impl<T> ops::And<T> for True {
    type Output = T;
}

impl<T> ops::Or<T> for True {
    type Output = True;
}

impl ops::Xor<True> for True {
    type Output = False;
}

impl ops::Xor<False> for True {
    type Output = True;
}

/// `false` as a dedicated type.
pub struct False;

impl Bool for False {
    const VALUE: bool = false;
}

impl ops::Not for False {
    type Output = True;
}

impl<T> ops::And<T> for False {
    type Output = False;
}

impl<T> ops::Or<T> for False {
    type Output = T;
}

impl ops::Xor<True> for False {
    type Output = True;
}

impl ops::Xor<False> for False {
    type Output = False;
}
