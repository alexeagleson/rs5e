//! Describes a change in HP to an entity, does not specify the reason for the change

#[derive(Debug)]
pub struct HpChange {
    pub max: u32,
    pub before: u32,
    pub after: u32,
}
