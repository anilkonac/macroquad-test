use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;

use crate::{laser::LaserPool, ship::Ship, LASER_FIRE_KEY, LASER_FIRE_PERIOD, SHIP_RADIUS};

pub fn handle_input_fire(
    ship: &Ship,
    laser_pool: &mut LaserPool,
    fire_time_accum: &mut f32,
    dt: f32,
) {
    if is_key_down(LASER_FIRE_KEY) {
        *fire_time_accum += dt;
        let diff = *fire_time_accum - LASER_FIRE_PERIOD;
        if diff > 0.0 {
            let next_laser = laser_pool.get_next_laser();
            next_laser.direction = Vec2::from_angle(-FRAC_PI_2 + ship.rotation_rad);
            next_laser.position = ship.pos + next_laser.direction.rotate(vec2(SHIP_RADIUS, 0.0));
            next_laser.speed = ship.speed + next_laser.direction * crate::LASER_VELOCITY;
            *fire_time_accum = diff;
        }
    } else if is_key_released(LASER_FIRE_KEY) {
        *fire_time_accum = LASER_FIRE_PERIOD;
    }
}

pub fn handle_input_direction(direction: &mut Vec2) {
    *direction = Vec2::ZERO;

    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        direction.y += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        direction.y -= 1.0;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        direction.x = 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        direction.x -= 1.0;
    }
}
