//! Type-level [booleans][boolean] meant for compile-time hackery.
//!
//! [boolean]: https://en.m.wikipedia.org/wiki/Boolean

#![no_std]
#![deny(missing_docs)]

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

/// `false` as a dedicated type.
pub struct False;

impl Bool for False {
    const VALUE: bool = false;
}
