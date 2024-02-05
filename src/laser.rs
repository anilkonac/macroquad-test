use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;

const LASER_VELOCITY: f32 = 200.0;

#[derive(Default, Clone)]
pub(crate) struct Laser {
    pub(crate) position: Vec2,
    pub(crate) speed: Vec2,
    pub(crate) rotation_rad: f32,
    pub(crate) active: bool,
}

impl Laser {
    pub(crate) fn draw(&self) {
        static V1: Vec2 = vec2(0.0, -2.0);
        static V2: Vec2 = vec2(0.0, 2.0);

        let rot_vec = Vec2::from_angle(self.rotation_rad);

        let v1 = rot_vec.rotate(V1) + self.position;
        let v2 = rot_vec.rotate(V2) + self.position;

        draw_line(v1.x, v1.y, v2.x, v2.y, 1.0, RED);
    }

    pub(crate) fn update(&mut self, dt: f32) {
        let direction = Vec2::from_angle(-FRAC_PI_2 + self.rotation_rad);
        self.position += self.speed * dt + direction * LASER_VELOCITY * dt;
    }
}
