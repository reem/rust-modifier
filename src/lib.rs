#![deny(missing_docs, warnings)]

//! Overloadable modification through both owned and mutable references
//! to a type with minimal code duplication.

/// Allows use of the implemented type as an argument to Set::set.
///
/// This allows types to be used for ad-hoc overloading of Set::set
/// to perform complex updates to the parameter of Modifier.
pub trait Modifier<F: ?Sized> {
    /// Modify `F` with self.
    fn modify(self, &mut F);
}

/// A trait providing the set and set_mut methods for all types.
///
/// Simply implement this for your types and they can be used
/// with modifiers.
pub trait Set {
    /// Modify self using the provided modifier.
    #[inline(always)]
    fn set<M: Modifier<Self>>(mut self, modifier: M) -> Self where Self: Sized {
        modifier.modify(&mut self);
        self
    }

    /// Modify self through a mutable reference with the provided modifier.
    #[inline(always)]
    fn set_mut<M: Modifier<Self>>(&mut self, modifier: M) -> &mut Self {
        modifier.modify(self);
        self
    }
}

mod impls;

#[cfg(test)]
mod test {
    pub use super::*;

    pub struct Thing {
        x: usize
    }

    impl Set for Thing {}

    pub struct ModifyX(usize);

    impl Modifier<Thing> for ModifyX {
        fn modify(self, thing: &mut Thing) {
            let ModifyX(val) = self;
            thing.x = val;
        }
    }

    #[test]
    fn test_set_and_set_mut() {
        let mut thing = Thing { x: 6 };
        thing.set_mut(ModifyX(8));
        assert_eq!(thing.x, 8);

        let thing = thing.set(ModifyX(9));
        assert_eq!(thing.x, 9);
    }

    #[test]
    fn test_tuple_chains() {
        let thing = Thing { x: 8 }.set((ModifyX(5), ModifyX(112)));
        assert_eq!(thing.x, 112);
    }
}

