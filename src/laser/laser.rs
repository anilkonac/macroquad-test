use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;
use macroquad_test::draw_line_w_rot;

use crate::ship::SHIP_RADIUS;

const LASER_VELOCITY: f32 = 300.0;
const V_LAS_1: Vec2 = vec2(-2.0, 0.0);
const V_LAS_2: Vec2 = vec2(2.0, 0.0);

#[derive(Default, Clone)]
pub struct Laser {
    position: Vec2,
    speed: Vec2,
    direction: Vec2,
    pub active: bool,
}

impl Laser {
    pub fn init(&mut self, ship_pos: Vec2, ship_rot_rad: f32, ship_speed: Vec2) {
        self.active = true;
        self.position = ship_pos + Vec2::from_angle(ship_rot_rad).rotate(vec2(0.0, -SHIP_RADIUS));
        self.direction = Vec2::from_angle(-FRAC_PI_2 + ship_rot_rad);
        self.speed = ship_speed + self.direction * LASER_VELOCITY;
    }

    #[inline]
    pub fn update(&mut self, dt: f32) {
        self.position += self.speed * dt;
    }

    #[inline]
    pub fn draw(&self) {
        draw_line_w_rot(self.direction, self.position, V_LAS_1, V_LAS_2, 1.0, RED)
    }
}
