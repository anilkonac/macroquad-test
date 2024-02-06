use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;
use macroquad_test::draw_line_w_rot;

use crate::ship::SHIP_RADIUS;

const LASER_VELOCITY: f32 = 300.0;
const V_LAS_1: Vec2 = vec2(-2.0, 0.0);
const V_LAS_2: Vec2 = vec2(2.0, 0.0);

#[derive(Default, Clone)]
struct Laser {
    position: Vec2,
    speed: Vec2,
    direction: Vec2,
    active: bool,
}

impl Laser {
    fn init(&mut self, ship_pos: Vec2, ship_rot_rad: f32, ship_speed: Vec2) {
        self.active = true;
        self.position = ship_pos + Vec2::from_angle(ship_rot_rad).rotate(vec2(0.0, -SHIP_RADIUS));
        self.direction = Vec2::from_angle(-FRAC_PI_2 + ship_rot_rad);
        self.speed = ship_speed + self.direction * LASER_VELOCITY;
    }
}

pub struct LaserPool {
    lasers: Vec<Laser>,
    index_next_laser: usize,
}

impl LaserPool {
    pub fn create(num_laser: usize) -> LaserPool {
        Self {
            lasers: vec![Laser::default(); num_laser],
            index_next_laser: 0,
        }
    }

    pub fn activate_next_laser(&mut self, ship_pos: Vec2, ship_speed: Vec2, ship_rot_rad: f32) {
        let laser = self.lasers.get_mut(self.index_next_laser).unwrap();
        laser.init(ship_pos, ship_rot_rad, ship_speed);
        self.index_next_laser += 1;
        if self.index_next_laser > self.lasers.len() - 1 {
            self.index_next_laser = 0;
        }
    }

    pub fn update(&mut self, dt: f32) {
        for laser in self.lasers.iter_mut() {
            if !laser.active {
                return;
            }
            laser.position += laser.speed * dt;
        }
    }

    pub fn draw(&self) {
        for laser in self.lasers.iter() {
            if !laser.active {
                return;
            }
            draw_line_w_rot(laser.direction, laser.position, V_LAS_1, V_LAS_2, 1.0, RED)
        }
    }
}
