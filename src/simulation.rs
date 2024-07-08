use crate::prelude::*;

const EXPONENT: usize = 1;
const GRAVITY: f32 = 9.8 * 1.0/(10_i32.pow(EXPONENT as u32) as f32);
const MASS: f32 = 1.0;

// state
struct Simulation<'a> {
    pub delta: f32,
    entities: Vec<&'a mut entity::Entity>,
    // pub objects
}

impl Simulation<'_> {
    fn step(&mut self) {
        self.gravity();
        self.entities.iter_mut().for_each(|ent| ent.step());
    }

    fn gravity(&mut self) {
        self.entities.iter_mut().for_each(|ent| ent.vel.y -= GRAVITY * MASS * self.delta)
    }
}
