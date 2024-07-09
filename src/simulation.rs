use crate::prelude::*;

pub struct Object {
    pub pos: entity::Pos,
    pub size: entity::Vec2,
}

impl Object {
    pub fn new(pos: entity::Pos, size: entity::Vec2) -> Self {
        Self { pos, size }
    }
}

// state
#[derive(Default)]
pub struct Simulation {
    pub entities: Vec<entity::Entity>,
    pub objects: Vec<Object>,
}

impl Simulation {
    pub fn add_entity(&mut self, entity: entity::Entity) {
        self.entities.push(entity)
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn step(&mut self, delta: f32) {
        for ent in 0..self.entities.len() {
            let mut ent = self.entities[ent];
            ent.step(delta);
            if let Some(obj) = self.colliding(&ent) {
                println!("colliding");
                ent.collide((obj.pos - ent.pos).normalize())
            }
        }
    }

    pub fn colliding(&self, ent: &entity::Entity) -> Option<&Object> {
        self.objects
            .iter()
            .filter(|&obj| ent.colliding(obj))
            .collect::<Vec<_>>()
            // TODO: shouldnt do first, should return a vec of objects and apply collision to all of them
            .first()
            .copied()
    }
}
