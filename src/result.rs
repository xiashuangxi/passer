pub trait ResultEntity {
    type Entity;
    fn entity(&self) -> Self::Entity;
}
