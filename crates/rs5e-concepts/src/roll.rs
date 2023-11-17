pub trait Roll {
    fn raw_value(&self) -> u32;

    fn bonus_value(&self) -> i32;

    fn total_value(&self) -> u32 {
        self.raw_value().saturating_add_signed(self.bonus_value())
    }
}
