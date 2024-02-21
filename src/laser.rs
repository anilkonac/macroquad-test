use macroquad::prelude::*;
use macroquad_test::draw_line_w_rot;

const LASER_THICKNESS: f32 = 2.0;
const LASER_LENGTH: f32 = 6.0;

const V_LASER_LEFT: Vec2 = vec2(-LASER_LENGTH / 2.0, 0.0);
const V_LASER_RIGHT: Vec2 = vec2(LASER_LENGTH / 2.0, 0.0);

const COLOR_LASER: Color = RED;

#[derive(Default, Clone)]
pub struct Laser {
    pub position: Vec2,
    pub speed: Vec2,
    pub direction: Vec2,
    // active: bool,
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

    pub fn get_next_laser(&mut self) -> &mut Laser {
        let num_lasers = self.lasers.len();
        let laser = &mut self.lasers[self.index_next_laser];
        self.index_next_laser += 1;
        if self.index_next_laser > num_lasers - 1 {
            self.index_next_laser = 0;
        }
        laser
    }

    pub fn update(&mut self, dt: f32) {
        for laser in self.lasers.iter_mut() {
            // if !laser.active {
            //     return;
            // }
            laser.position += laser.speed * dt;
        }
    }

    pub fn draw(&self) {
        for laser in self.lasers.iter() {
            // if !laser.active {
            //     return;
            // }
            draw_line_w_rot(
                laser.direction,
                laser.position,
                V_LASER_LEFT,
                V_LASER_RIGHT,
                LASER_THICKNESS,
                COLOR_LASER,
            )
        }
    }
}
