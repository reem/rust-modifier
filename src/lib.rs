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

/// Allows determining which modifier to use at runtime.
///
/// **NOTE** You do not need to implement this trait yourself, it is public as
/// you will need to use it as the type for any boxed trait (see the type of
/// `get_mod` in the example).
///
/// ```rust
/// # use modifier::*;
/// # #[derive(Eq, PartialEq, Debug)]
/// struct U(u32);
/// # impl Set for U {};
///
/// struct Increment;
/// struct Decrement;
///
/// // Straightforward implementation details not shown for brevity.
/// impl Modifier<U> for Increment
/// # {
/// #     fn modify(self, u: &mut U) {
/// #         u.0 += 1;
/// #     }
/// # }
/// impl Modifier<U> for Decrement
/// # {
/// #     fn modify(self, u: &mut U) {
/// #         u.0 -= 1;
/// #     }
/// # }
///
/// let get_mod = |up| -> Box<ModifierBox<U>> {
///     if up { Box::new(Increment) } else { Box::new(Decrement) }
/// };
///
/// assert_eq!(U(1), U(0).set((get_mod(true), get_mod(false), get_mod(true))));
/// ```
///
/// *(Yes this is a stupid example that could be done using an enum instead,
/// real examples for this use case are quite complicated)*
pub trait ModifierBox<F: ?Sized> {
    /// Modify `F` with a boxed self.
    fn modify_box(self: Box<Self>, &mut F);
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
            thing.x = self.0;
        }
    }

    impl Modifier<BiggerThing> for ModifyFirst {
        fn modify(self, bigger_thing: &mut BiggerThing) {
            bigger_thing.first = self.0;
        }
    }

    impl Modifier<BiggerThing> for ModifySecond {
        fn modify(self, bigger_thing: &mut BiggerThing) {
            bigger_thing.second = self.0;
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
    fn test_optional_modifier() {
        let get_mod = |b| if b { Some(ModifyX(5)) } else { None };
        let thing = Thing { x: 8 }.set(get_mod(false));
        assert_eq!(thing.x, 8);

        let thing = thing.set(get_mod(true));
        assert_eq!(thing.x, 5);
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

    #[test]
    fn test_boxed_modifiers() {
        let get_mod = |b| -> Box<ModifierBox<Thing>> {
            if b {
                Box::new(ModifyX(1))
            } else {
                Box::new((ModifyX(2), ModifyX(3)))
            }
        };

        let thing = Thing { x: 8 }.set(get_mod(true));
        assert_eq!(thing.x, 1);

        let thing = Thing { x: 8 }.set(get_mod(false));
        assert_eq!(thing.x, 3);
    }

}

