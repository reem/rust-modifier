#![license = "MIT"]
#![deny(missing_docs, warnings)]
#![feature(phase, globs)]

//! Overloadable modification through both owned and mutable references
//! to a type with minimal code duplication.

use std::ptr;

/// Allows use of the implemented type as an argument to Set::set.
///
/// This allows types to be used for ad-hoc overloading of Set::set
/// to perform complex updates to the parameter of Modifier.
pub trait Modifier<F> {
    /// Modify `F` with self.
    fn modify(self, F) -> F;
}

/// A blanket trait providing the set and set_mut methods for all types.
pub trait Set {
    /// Modify self using the provided modifier.
    fn set<M: Modifier<Self>>(self, modifier: M) -> Self {
        modifier.modify(self)
    }

    /// Modify self through a mutable reference with the provided modifier.
    fn set_mut<M: Modifier<Self>>(&mut self, modifier: M) -> &mut Self {
        *self = modifier.modify(unsafe { ptr::read(&*self as *const _) });
        self
    }
}

impl<T> Set for T {}

#[cfg(test)]
mod test {
    #[phase(plugin)] extern crate stainless;
    pub use super::*;

    pub struct Thing {
        x: uint
    }

    pub struct ModifyX(uint);

    impl Modifier<Thing> for ModifyX {
        fn modify(self, mut thing: Thing) -> Thing {
            let ModifyX(val) = self;
            thing.x = val;
            thing
        }
    }

    describe! modifier {
        it "should support modifying through ModifyX using set and set_mut" {
            let mut thing = Thing { x: 6 };
            thing.set_mut(ModifyX(8));
            assert_eq!(thing.x, 8);

            let thing = thing.set(ModifyX(9));
            assert_eq!(thing.x, 9);
        }
    }
}

