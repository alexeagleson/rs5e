//! Behaviour that describes an entity that can take some kind of hit and
//! defends against it with the concept of AC (Armor Class). Actual armor
//! might be an implementation of this trait however we do not want to
//! say that it is specific to armor, as other sources might provide
//! an AC value that are not specific to armor.

use rs5e_concepts::armor_class::{ArmorClass, DEFAULT_ARMOR_CLASS};

pub trait Defender {
    fn armor_class(&self) -> &ArmorClass {
        &DEFAULT_ARMOR_CLASS
    }
}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[derive(Default)]
    pub struct MockDefends {
        armor_class: ArmorClass,
    }

    impl Defender for MockDefends {
        fn armor_class(&self) -> &ArmorClass {
            &self.armor_class
        }
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use crate::defender::{mocks::MockDefends, Defender};
    use rs5e_concepts::armor_class::ArmorClass;

    #[test]
    fn defends_works() {
        let defends = MockDefends::default();

        assert_eq!(defends.armor_class(), &ArmorClass::new(10));
    }
}
