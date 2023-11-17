//! Applies when a D20 is rolled in certain conditions (attack roll, ability check)

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CriticalState {
    Critical,
    Normal,
    CriticalFail,
}

impl CriticalState {
    /// The state of this value is critical
    #[must_use]
    pub const fn is_critical(&self) -> bool {
        matches!(self, Self::Critical)
    }
}
