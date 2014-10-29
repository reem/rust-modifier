#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]
#![feature(phase, globs)]

//! Crate comment goes here

/// Allows use of the implemented type as an argument to Set::set.
///
/// This allows types to be used for ad-hoc overloading of Set::set
/// to perform complex updates to the parameter of Modifier.
pub trait Modifier<For> {
    /// Modify `For` with self.
    fn modify(self, &mut For);
}

/// A blanket trait providing the set method for all types.
pub trait Set {
    /// Modify self using the provided modifier.
    fn set<M: Modifier<Self>>(&mut self, modifier: M) {
        modifier.modify(self)
    }
}

impl<T> Set for T {}

#[cfg(test)]
mod test {
    #[phase(plugin)] extern crate stainless;
    pub use super::*;

    pub struct Thing {
        x: uint,
        y: String
    }

    pub struct ModifyX(uint);
    pub struct ModifyY(String);

    impl Modifier<Thing> for ModifyX {
        fn modify(self, thing: &mut Thing) {
            let ModifyX(val) = self;
            thing.x = val;
        }
    }

    impl Modifier<Thing> for ModifyY {
        fn modify(self, thing: &mut Thing) {
            let ModifyY(val) = self;
            thing.y = val;
        }
    }

    describe! modifier {
        it "should modify thing.x when ModifyX is used" {
            let mut thing = Thing { x: 8, y: "".into_string() };
            thing.set(ModifyX(8));
            assert_eq!(thing.x, 8);
        }

        it "should modify thing.y when ModifyY is used" {
            let mut thing = Thing { x: 8, y: "".into_string() };
            thing.set(ModifyY("hello".into_string()));
            assert_eq!(thing.y.as_slice(), "hello");
        }
    }
}

