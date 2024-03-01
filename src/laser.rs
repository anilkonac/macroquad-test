use macroquad::prelude::*;
use macroquad_test::{
    core::{pool::ObjectPool, timer::Timer},
    draw_line_w_rot,
};

const LASER_THICKNESS: f32 = 2.0;
const LASER_LENGTH: f32 = 6.0;
const FRAC_LASER_LENGTH_2: f32 = LASER_LENGTH / 2.0;

const V_LASER_LEFT: Vec2 = vec2(-FRAC_LASER_LENGTH_2, 0.0);
const V_LASER_RIGHT: Vec2 = vec2(FRAC_LASER_LENGTH_2, 0.0);

const COLOR_LASER: Color = RED;

#[derive(Default)]
pub struct Laser {
    pub position: Vec2,
    pub speed: Vec2,
    pub direction: Vec2,
    pub lifetime: f32,
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
            if laser.lifetime < 0.0 {
                return;
            }
            laser.lifetime -= dt;

            let pos = &mut laser.position;
            *pos += laser.speed * dt;

            if pos.x < -FRAC_LASER_LENGTH_2 {
                pos.x = sc_width + FRAC_LASER_LENGTH_2;
            } else if pos.x > sc_width + FRAC_LASER_LENGTH_2 {
                pos.x = -FRAC_LASER_LENGTH_2;
            }

            if pos.y < -FRAC_LASER_LENGTH_2 {
                pos.y = sc_height + FRAC_LASER_LENGTH_2;
            } else if pos.y > sc_height + FRAC_LASER_LENGTH_2 {
                pos.y = -FRAC_LASER_LENGTH_2;
            }
        });
    }

    pub fn draw(&self) {
        self.pool.for_each(|laser| {
            if laser.lifetime < 0.0 {
                return;
            }
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
