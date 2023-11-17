pub trait Named {
    fn name(&self) -> &str;

    fn set_name(&mut self, new_name: String);
}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    use super::*;
    use rs5e_macro_derive::Named;

    #[derive(Debug, Named)]
    pub struct MockNamed {
        pub(super) name: String,
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {
    use super::{mocks::MockNamed, *};

    #[test]
    fn name_works() {
        let mut named_entity = MockNamed {
            name: String::from("My name"),
        };

        named_entity.set_name(String::from("My new name"));

        assert_eq!(named_entity.name(), "My new name");
    }
}
