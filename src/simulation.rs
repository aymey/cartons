use crate::prelude::*;

// state
#[derive(Default)]
pub struct Simulation {
    pub entities: Vec<entity::Entity>,
    // pub objects
}

impl Simulation {
    pub fn step(&mut self, delta: f32) {
        self.entities.iter_mut().for_each(|ent| ent.step(delta));
    }

    pub fn add(&mut self, entity: entity::Entity) {
        self.entities.push(entity)
    }
}
