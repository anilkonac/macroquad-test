use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;

use crate::{laser::Laser, ship::SHIP_RADIUS};

mod laser;
mod ship;

#[macroquad::main("TheGame")]
async fn main() {
    const ACCELERATION: f32 = 100.0;
    const ACCELERATION_ANGULAR: f32 = 100.0;
    const MAX_VELOCITY: f32 = 200.0;
    const MAX_VELOCITY_ANGULAR: f32 = 200.0;

    let mut speed = Vec2::ZERO;
    let mut speed_angular = 0.0;
    let mut position = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut input_direction = Vec2::ZERO;
    let mut rotation: f32 = 0.0;

    let mut laser_pool = vec![Laser::default(); 20];
    let mut index_next_laser: u8 = 0;

    loop {
        clear_background(BLACK);
        let rot_rad = rotation.to_radians();

        // Handle input
        get_input_direction(&mut input_direction);
        if is_key_pressed(KeyCode::Space) {
            let laser = laser_pool.get_mut(index_next_laser as usize).unwrap();
            laser.active = true;
            laser.position = position + Vec2::from_angle(rot_rad).rotate(vec2(0.0, -SHIP_RADIUS));
            laser.rotation_rad = rot_rad;
            laser.speed = speed;
            index_next_laser += 1;
            // num_active_laser = num_active_laser.clamp(0, 19);
            if index_next_laser as usize > laser_pool.len() - 1 {
                index_next_laser = 0;
            }
        }

        // Update speed and position
        let dt = get_frame_time();
        let rot_rad = f32::to_radians(rotation);
        if input_direction != Vec2::ZERO {
            speed_angular += input_direction.y * ACCELERATION_ANGULAR * dt;
            speed_angular = speed_angular.clamp(-MAX_VELOCITY_ANGULAR, MAX_VELOCITY_ANGULAR);
            let direction = Vec2::from_angle(-FRAC_PI_2 + rot_rad);
            speed += direction * input_direction.x * ACCELERATION * dt;
            speed = speed.clamp_length_max(MAX_VELOCITY);
        }
        position += speed * dt;
        rotation += speed_angular * dt;

        // update lasers
        for laser in laser_pool.iter_mut() {
            if !laser.active {
                continue;
            }
            laser.update(dt);
        }

        // Draw

        ship::draw_ship(position, rot_rad, input_direction);
        // draw_text_speed(&speed);
        draw_text_fps();
        // Draw lasers
        for laser in laser_pool.iter() {
            if !laser.active {
                continue;
            }
            laser.draw();
        }

        next_frame().await
    }
}

fn get_input_direction(direction: &mut Vec2) {
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

fn draw_text_speed(speed: &Vec2) {
    let speed_text = "Speed: ".to_string() + &speed.to_string();
    draw_text(&speed_text, 0.0, 16.0, 30.0, GRAY);
}

fn draw_text_fps() {
    let fps_text = String::from("FPS: ") + &get_fps().to_string();
    draw_text(&fps_text, screen_width() - 120.0, 16.0, 30.0, GRAY);
}
