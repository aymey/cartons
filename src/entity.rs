use std::ops::{Add, AddAssign, Deref};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}
pub type Vel2 = Pos2;

impl Add for Pos2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Pos2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Pos2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pos: Pos2, // [0..=1]
    pub vel: Vel2,
    rad: f32,
}

impl Entity {
    pub fn new(pos: Pos2, vel: Vel2, rad: f32) -> Self {
        Self { pos, vel, rad }
    }

    pub fn step(&mut self) {
        self.pos += self.vel
    }
}
