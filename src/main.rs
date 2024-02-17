use macroquad::prelude::*;

use crate::{laser::LaserPool, ship::Ship};

mod laser;
mod ship;

// Main constants that affect gameplay
const SHIP_RADIUS: f32 = 20.0;
const SHIP_ACCELERATION: f32 = 100.0;
const SHIP_ACCELERATION_ANGULAR: f32 = 100.0;
const SHIP_VELOCITY_MAX: f32 = 200.0;
const SHIP_VELOCITY_ANGULAR_MAX: f32 = 200.0;
const LASER_VELOCITY: f32 = 300.0;
const LASER_FIRE_PERIOD: f32 = 0.3;
const LASER_FIRE_KEY: KeyCode = KeyCode::Space;

#[macroquad::main(window_conf)]
async fn main() {
    let mut ship = Ship::default();
    let mut laser_pool = LaserPool::create(20);
    let mut input_direction = Vec2::ZERO;

    loop {
        let dt = get_frame_time();

        // Handle input
        get_input_direction(&mut input_direction);
        laser_pool.handle_firing(&ship, dt);

        // Update
        ship.update(input_direction, dt);
        laser_pool.update(dt);

        // Draw
        clear_background(BLACK);
        ship.draw(input_direction);
        laser_pool.draw();
        // draw_text_speed(&speed);
        draw_text_fps();

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "TheGame".to_owned(),
        high_dpi: cfg!(target_os = "macos"),
        sample_count: 4,
        ..Default::default()
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

#[allow(dead_code)]
fn draw_text_speed(speed: &Vec2) {
    let speed_text = "Speed: ".to_string() + &speed.to_string();
    draw_text(&speed_text, 0.0, 16.0, 30.0, GRAY);
}

#[allow(dead_code)]
fn draw_text_fps() {
    let fps_text = String::from("FPS: ") + &get_fps().to_string();
    draw_text(&fps_text, screen_width() - 120.0, 16.0, 30.0, GRAY);
}
