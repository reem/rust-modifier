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

    pub struct BiggerThing {
        first: usize,
        second: usize
    }

    impl Set for Thing {}
    impl Set for BiggerThing {}

    pub struct ModifyX(usize);
    pub struct ModifyFirst(usize);
    pub struct ModifySecond(usize);

    impl Modifier<Thing> for ModifyX {
        fn modify(self, thing: &mut Thing) {
            let ModifyX(val) = self;
            thing.x = val;
        }
    }

    impl Modifier<BiggerThing> for ModifyFirst {
        fn modify(self, bigger_thing: &mut BiggerThing) {
            let ModifyFirst(val) = self;
            bigger_thing.first = val;
        }
    }

    impl Modifier<BiggerThing> for ModifySecond {
        fn modify(self, bigger_thing: &mut BiggerThing) {
            let ModifySecond(val) = self;
            bigger_thing.second = val;
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

    #[test]
    fn test_tuple_different_fields() {
        let bigger_thing = BiggerThing { first: 1, second: 2}.set((ModifyFirst(10), ModifySecond(12)));
        assert_eq!(bigger_thing.first, 10);
        assert_eq!(bigger_thing.second, 12);
    }
}

