use std::ops::{Add, AddAssign, Div, Mul, Neg};

use serde::{Deserialize, Serialize};

const EXPONENT: usize = 1;
const GRAVITY: f32 = 9.8 * 1.0 / (10_i32.pow(EXPONENT as u32) as f32);
const MASS: f32 = 1.0;
const TERMINAL_VELOCITY: f32 = 5.0;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
pub type Pos = Vec2;
pub type Vel = Vec2;

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Self {
        *self / self.magnitude()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pos: Pos, // [0..=1]
    pub vel: Vel,
    rad: f32,
}

impl Entity {
    pub fn new(pos: Pos, vel: Vel, rad: f32) -> Self {
        Self { pos, vel, rad }
    }

    pub fn step(&mut self) {
        self.gravity();
        self.pos += self.vel
    }

    fn gravity(&mut self) {
        self.vel.y -= GRAVITY * MASS
    }

    fn drag(&mut self) {
        let dir = self.vel.normalize();
        // TODO: do
        let drag = 1.0;

        self.vel += -dir * drag;
    }
}
