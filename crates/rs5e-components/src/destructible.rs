use super::{defender::Defender, destroyed::Destroyed, identifiable::Identifiable};
use rs5e_concepts::hp_change::HpChange;

#[derive(Debug)]
pub enum DestructionState<T, U>
where
    T: Destructible<Destroyed = U>,
    U: Destroyed,
{
    Destructible(T),
    Destroyed(U),
}

impl<T, U> DestructionState<T, U>
where
    T: Destructible<Destroyed = U>,
    U: Destroyed,
{
    pub const fn as_destructible(&self) -> Option<&T> {
        match self {
            Self::Destructible(destructible) => Some(destructible),
            Self::Destroyed(_) => None,
        }
    }

    pub const fn as_destroyed(&self) -> Option<&U> {
        match self {
            Self::Destructible(_) => None,
            Self::Destroyed(destroyed) => Some(destroyed),
        }
    }
}

pub trait Destructible: Identifiable + Defender + Sized {
    type Destroyed: Destroyed;

    fn hp(&self) -> u32;

    fn max_hp(&self) -> u32;

    fn take_damage(&mut self, damage: u32) -> HpChange;

    fn destroy(self) -> Self::Destroyed;

    fn should_destroy(&self) -> bool {
        self.hp() == 0
    }

    fn checked_destroy(self) -> DestructionState<Self, Self::Destroyed> {
        if self.should_destroy() {
            DestructionState::Destroyed(self.destroy())
        } else {
            DestructionState::Destructible(self)
        }
    }
}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    #[allow(clippy::wildcard_imports)]
    use super::*;
    use crate::{
        armor::mocks::MockArmor, combatant::Combatant, destroyed::mocks::MockDestroyed,
        has_armor::HasArmor,
    };
    use rs5e_concepts::{
        cover_state::{CoverState, DEFAULT_COVER_STATE},
        id::Id,
        prone_state::{ProneState, DEFAULT_PRONE_STATE},
    };
    use rs5e_macro_derive::Identifiable;

    #[derive(Debug, Identifiable)]
    pub struct MockDestructible {
        id: Id,
        hp: u32,
        max_hp: u32,
        equipped_armor: MockArmor,
    }

    impl Defender for MockDestructible {}

    impl Combatant for MockDestructible {
        fn prone_state(&self) -> &ProneState {
            &DEFAULT_PRONE_STATE
        }

        fn cover_state(&self) -> &CoverState {
            &DEFAULT_COVER_STATE
        }
    }

    impl HasArmor for MockDestructible {
        type Armor = MockArmor;

        fn equipped_armor(&self) -> Option<&Self::Armor> {
            Some(&self.equipped_armor)
        }
    }

    impl Destructible for MockDestructible {
        type Destroyed = MockDestroyed;

        fn hp(&self) -> u32 {
            self.hp
        }

        fn max_hp(&self) -> u32 {
            self.max_hp
        }

        fn take_damage(&mut self, damage: u32) -> HpChange {
            let before = self.hp();
            self.hp = self.hp.saturating_sub(damage);
            HpChange {
                before,
                max: self.max_hp(),
                after: self.hp(),
            }
        }

        fn destroy(self) -> Self::Destroyed {
            MockDestroyed { id: self.id }
        }
    }

    impl MockDestructible {
        #[must_use]
        pub fn new(hp: u32) -> Self {
            Self {
                id: Id::new_incremental(),
                max_hp: hp,
                hp,
                equipped_armor: MockArmor::new(),
            }
        }
    }
}
