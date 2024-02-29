use macroquad::prelude::*;
use macroquad_test::{
    core::{pool::ObjectPool, timer::Timer},
    draw_line_w_rot,
};

const LASER_THICKNESS: f32 = 2.0;
const LASER_LENGTH: f32 = 6.0;

const V_LASER_LEFT: Vec2 = vec2(-LASER_LENGTH / 2.0, 0.0);
const V_LASER_RIGHT: Vec2 = vec2(LASER_LENGTH / 2.0, 0.0);

const COLOR_LASER: Color = RED;

#[derive(Default)]
pub struct Laser {
    pub position: Vec2,
    pub speed: Vec2,
    pub direction: Vec2,
}

pub struct LaserManager {
    pool: ObjectPool<Laser>,
    fire_timer: Timer,
}

impl LaserManager {
    pub fn new(num_laser: usize, fire_rate: f32) -> LaserManager {
        LaserManager {
            pool: ObjectPool::new(num_laser),
            fire_timer: Timer::new(fire_rate),
        }
    }

    pub fn fire(&mut self, dt: f32) -> Option<&mut Laser> {
        if self.fire_timer.update(dt) {
            return Some(self.pool.get_next_mut());
        }
        None
    }

    #[inline]
    pub fn stop(&mut self) {
        self.fire_timer.reset();
    }

    pub fn update(&mut self, dt: f32) {
        let sc_width = screen_width();
        let sc_height = screen_height();

        self.pool.for_each_mut(|laser| {
            let pos = &mut laser.position;
            *pos += laser.speed * dt;

            if pos.x < -LASER_LENGTH {
                pos.x = sc_width + LASER_LENGTH;
            } else if pos.x > sc_width + LASER_LENGTH {
                pos.x = -LASER_LENGTH;
            }

            if pos.y < -LASER_LENGTH {
                pos.y = sc_height + LASER_LENGTH;
            } else if pos.y > sc_height + LASER_LENGTH {
                pos.y = -LASER_LENGTH;
            }
        });
    }

    pub fn draw(&self) {
        self.pool.for_each(|laser| {
            draw_line_w_rot(
                laser.direction,
                laser.position,
                V_LASER_LEFT,
                V_LASER_RIGHT,
                LASER_THICKNESS,
                COLOR_LASER,
            )
        });
    }
}
