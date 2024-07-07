use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pos2 {
    x: f32,
    y: f32,
}
pub type Vel2 = Pos2;

impl Pos2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pos: Pos2, // [0..=1]
    vel: Vel2,
    rad: f32,
}

impl Entity {
    pub fn new(pos: Pos2, vel: Vel2, rad: f32) -> Self {
        Self { pos, vel, rad }
    }
}
