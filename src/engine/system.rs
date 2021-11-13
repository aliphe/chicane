use crate::engine::entities_repository::entities_repository::EntitiesRepository;

pub trait System {
    fn tick(&self, entities_repository: &mut dyn EntitiesRepository) -> Result<(), String>;
}
