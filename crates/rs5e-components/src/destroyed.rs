use super::identifiable::Identifiable;

pub trait Destroyed: Identifiable {}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    use super::*;
    use rs5e_concepts::id::Id;
    use rs5e_macro_derive::Identifiable;

    #[derive(Debug, Identifiable)]
    pub struct MockDestroyed {
        pub (crate) id: Id,
    }

    impl Destroyed for MockDestroyed {}

    impl MockDestroyed {
        pub fn new() -> Self {
            Self {
                id: Id::new_incremental(),
            }
        }
    }
}
