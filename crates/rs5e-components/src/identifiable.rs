use rs5e_concepts::id::Id;

pub trait Identifiable {
    fn id(&self) -> Id;

    fn set_id(&mut self, new_id: Id);
}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    use super::*;
    use rs5e_macro_derive::Identifiable;

    #[derive(Debug, Identifiable)]
    pub struct MockIdentifiable {
        pub(super) id: Id,
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {
    use super::{mocks::MockIdentifiable, *};

    #[test]
    fn identifier_works() {
        let mut identifiable_entity = MockIdentifiable {
            id: Id::new_incremental(),
        };

        identifiable_entity.set_id(Id::new_incremental());

        // We generated two mock IDs, so would should be at the second ID starting from 1
        assert_eq!(identifiable_entity.id(), Id::from_u64(2));
    }
}
