use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;
use macroquad_test::draw_line_w_rot;

use crate::ship::SHIP_RADIUS;

const LASER_VELOCITY: f32 = 200.0;
const V_LAS_TOP: Vec2 = vec2(0.0, -2.0);
const V_LAS_BTTM: Vec2 = vec2(0.0, 2.0);

#[derive(Default, Clone)]
pub(crate) struct Laser {
    position: Vec2,
    speed: Vec2,
    rotation_rad: f32,
    pub(crate) active: bool,
}

impl Laser {
    pub(crate) fn init(&mut self, ship_pos: Vec2, ship_rot_rad: f32, ship_speed: Vec2) {
        self.active = true;
        self.position = ship_pos + Vec2::from_angle(ship_rot_rad).rotate(vec2(0.0, -SHIP_RADIUS));
        self.rotation_rad = ship_rot_rad;
        self.speed = ship_speed;
    }

    pub(crate) fn update(&mut self, dt: f32) {
        let direction = Vec2::from_angle(-FRAC_PI_2 + self.rotation_rad);
        // self.position += self.speed * dt + direction * LASER_VELOCITY * dt;
        self.position += (self.speed + direction * LASER_VELOCITY) * dt;
    }

    #[inline]
    pub(crate) fn draw(&self) {
        let rot_vec = Vec2::from_angle(self.rotation_rad);
        draw_line_w_rot(rot_vec, self.position, V_LAS_TOP, V_LAS_BTTM, 1.0, RED)
    }
}
