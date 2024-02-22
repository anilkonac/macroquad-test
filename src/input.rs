use macroquad::prelude::*;
use macroquad_test::Timer;
use std::f32::consts::FRAC_PI_2;

use crate::{laser::LaserPool, ship::Ship, SHIP_RADIUS};

const LASER_FIRE_KEY: KeyCode = KeyCode::Space;

pub fn handle_input_fire(ship: &Ship, laser_pool: &mut LaserPool, fire_timer: &mut Timer, dt: f32) {
    if is_key_down(LASER_FIRE_KEY) {
        if fire_timer.update(dt) {
            let next_laser = laser_pool.get_next_laser();
            next_laser.direction = Vec2::from_angle(-FRAC_PI_2 + ship.rotation_rad);
            next_laser.position = ship.pos + next_laser.direction.rotate(vec2(SHIP_RADIUS, 0.0));
            next_laser.speed = ship.speed + next_laser.direction * crate::LASER_VELOCITY;
        }
    } else if is_key_released(LASER_FIRE_KEY) {
        fire_timer.reset();
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
