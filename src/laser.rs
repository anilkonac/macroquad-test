use macroquad::prelude::*;
use macroquad_test::{
    core::{pool::ObjectPool, timer::Timer},
    draw_line_w_rot, lerp,
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

impl Laser {
    fn teleport(&mut self) {
        let pos = &mut self.position;
        let (screen_width, screen_height) = (screen_width(), screen_height());
        let off_screen_left = pos.x < -FRAC_LASER_LENGTH_2;
        let off_screen_right = pos.x > screen_width + FRAC_LASER_LENGTH_2;
        let off_screen_up = pos.y < -FRAC_LASER_LENGTH_2;
        let off_screen_down = pos.y > screen_height + FRAC_LASER_LENGTH_2;

        if !off_screen_left && !off_screen_right && !off_screen_up && !off_screen_down {
            return;
        }

        let abs_speed_direction = self.speed.normalize_or_zero().abs();

        let lerp_mut = abs_speed_direction.y / 1.0;
        let pos_y_lerp = || lerp(pos.y, screen_height - pos.y, lerp_mut);
        if off_screen_left {
            *pos = vec2(screen_width + FRAC_LASER_LENGTH_2, pos_y_lerp());
        } else if off_screen_right {
            *pos = vec2(-FRAC_LASER_LENGTH_2, pos_y_lerp());
        }

        let lerp_mut = abs_speed_direction.x / 1.0;
        let pos_x_lerp = || lerp(pos.x, screen_width - pos.x, lerp_mut);
        if off_screen_up {
            *pos = vec2(pos_x_lerp(), screen_height + FRAC_LASER_LENGTH_2);
        } else if off_screen_down {
            *pos = vec2(pos_x_lerp(), -FRAC_LASER_LENGTH_2);
        }
    }
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
        self.pool.for_each_mut(|laser| {
            if laser.lifetime <= 0.0 {
                return;
            }
            laser.lifetime -= dt;
            // println!("Update");

            laser.position += laser.speed * dt;

            laser.teleport();
        });
    }

    pub fn draw(&self) {
        self.pool.for_each(|laser| {
            if laser.lifetime <= 0.0 {
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
