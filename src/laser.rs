use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;
use macroquad_test::draw_line_w_rot;

use crate::ship::{Ship, SHIP_RADIUS};

const LASER_VELOCITY: f32 = 300.0;
const LASER_THICKNESS: f32 = 2.0;
const LASER_LENGTH: f32 = 6.0;

const V_LASER_LEFT: Vec2 = vec2(-LASER_LENGTH / 2.0, 0.0);
const V_LASER_RIGHT: Vec2 = vec2(LASER_LENGTH / 2.0, 0.0);

#[derive(Default, Clone)]
struct Laser {
    position: Vec2,
    speed: Vec2,
    direction: Vec2,
    active: bool,
}

impl Laser {
    fn init(&mut self, ship: &Ship) {
        self.active = true;
        self.position =
            ship.pos + Vec2::from_angle(ship.rotation_rad).rotate(vec2(0.0, -SHIP_RADIUS));
        self.direction = Vec2::from_angle(-FRAC_PI_2 + ship.rotation_rad);
        self.speed = ship.speed + self.direction * LASER_VELOCITY;
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

    pub fn activate_next_laser(&mut self, ship: &Ship) {
        let laser = self.lasers.get_mut(self.index_next_laser).unwrap();
        laser.init(ship);
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
            draw_line_w_rot(
                laser.direction,
                laser.position,
                V_LASER_LEFT,
                V_LASER_RIGHT,
                LASER_THICKNESS,
                RED,
            )
        }
    }
}
