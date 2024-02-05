use macroquad::prelude::*;

use super::laser::Laser;

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
            laser.update(dt);
        }
    }

    pub fn draw(&self) {
        for laser in self.lasers.iter() {
            if !laser.active {
                return;
            }
            laser.draw();
        }
    }
}
