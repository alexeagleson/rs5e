use rs5e_concepts::class_type::ClassType;

pub trait HasClass {
    fn class(&self) -> &ClassType;
}
