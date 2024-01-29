use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;
use macroquad_test::draw_player;

#[derive(Default, Clone)]
struct Laser {
    position: Vec2,
    rotation: f32,
    active: bool,
}

#[macroquad::main("SimpleGame")]
async fn main() {
    const ACCELERATION: f32 = 100.0;
    const ACCELERATION_ANGULAR: f32 = 100.0;
    const MAX_VELOCITY: f32 = 200.0;
    const MAX_VELOCITY_ANGULAR: f32 = 200.0;
    const LASER_SPEED: f32 = 20.0;

    let mut speed = Vec2::ZERO;
    let mut speed_angular = 0.0;
    let mut position = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut input_direction = Vec2::ZERO;
    let mut rotation = 0.0;

    let mut laser_pool = vec![Laser::default(); 20];
    let mut num_active_laser: u8 = 0;

    loop {
        clear_background(BLACK);

        // Handle input
        get_input_direction(&mut input_direction);
        if (is_key_down(KeyCode::Space)) {
            let laser = laser_pool.get_mut(num_active_laser as usize).unwrap();
            laser.active = true;
            laser.position = position;
            laser.rotation = rotation;
            num_active_laser += 1;
            num_active_laser = num_active_laser.clamp(0, 19);
        }

        // Update speed and position
        let dt = get_frame_time();
        if input_direction != Vec2::ZERO {
            speed_angular += input_direction.y * ACCELERATION_ANGULAR * dt;
            speed_angular = speed_angular.clamp(-MAX_VELOCITY_ANGULAR, MAX_VELOCITY_ANGULAR);
            let direction = Vec2::from_angle(-FRAC_PI_2 + f32::to_radians(rotation));
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
            laser.position += LASER_SPEED * dt;
        }

        // Draw

        draw_player(position, rotation, input_direction);
        // draw_text_speed(&speed);
        draw_text_fps();
        draw_lasers(&laser_pool);

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

fn draw_lasers(laser_pool: &Vec<Laser>) {
    for laser in laser_pool.iter() {
        let pos = &laser.position;
        draw_line(pos.x, pos.y - 2.0, pos.x, pos.y + 2.0, 2.0, RED)
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
