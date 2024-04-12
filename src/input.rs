use macroquad::prelude::*;

use crate::{
    laser::LaserManager,
    ship::{Ship, SHIP_RADIUS},
    LASER_LIFETIME,
};

const LASER_FIRE_KEY: KeyCode = KeyCode::Space;

pub fn handle_input_fire(ship: &Ship, laser_manager: &mut LaserManager, dt: f32) {
    if is_key_down(LASER_FIRE_KEY) {
        if let Some(fired_laser) = laser_manager.fire(dt) {
            fired_laser.direction = ship.rot_vec;
            fired_laser.position = ship.pos + ship.rot_vec.rotate(vec2(SHIP_RADIUS, 0.0));
            fired_laser.speed = ship.speed + ship.rot_vec * crate::LASER_VELOCITY;
            fired_laser.lifetime = LASER_LIFETIME;
        }
    } else if is_key_released(LASER_FIRE_KEY) {
        laser_manager.stop();
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
        direction.x += 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        direction.x -= 1.0;
    }
}
