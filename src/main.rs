use macroquad::prelude::*;

mod input;
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

#[macroquad::main(window_conf)]
async fn main() {
    let mut ship = ship::Ship::default();
    let mut laser_pool = laser::LaserPool::create(20);
    let mut input_direction = Vec2::ZERO;
    let mut fire_time_accum = LASER_FIRE_PERIOD;

    loop {
        let dt = get_frame_time();

        // Handle input
        input::handle_input_direction(&mut input_direction);
        input::handle_input_fire(&ship, &mut laser_pool, &mut fire_time_accum, dt);

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
