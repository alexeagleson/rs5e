use rs5e_concepts::level::Level;

pub trait HasLevel {
    fn level(&self) -> &Level;
}
