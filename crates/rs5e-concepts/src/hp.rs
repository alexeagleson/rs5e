#[derive(Debug)]
pub struct Hp {
    pub max: u32,
    pub current: u32,
}

impl Hp {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self {
            current: value,
            max: value,
        }
    }
}
