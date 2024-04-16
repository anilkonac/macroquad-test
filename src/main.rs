use macroquad::prelude::*;

mod input;
mod laser;
mod ship;
mod teleport;

// Main constants that affect gameplay
const SHIP_ACCELERATION: f32 = 200.0;
const SHIP_ACCELERATION_ANGULAR: f32 = 100.0;
const SHIP_VELOCITY_MAX: f32 = 500.0;
const SHIP_VELOCITY_ANGULAR_MAX: f32 = 200.0;
const LASER_VELOCITY: f32 = 500.0;
const LASER_FIRE_PERIOD: f32 = 0.3;
const LASER_LIFETIME: f32 = 3.0;

#[macroquad::main(window_conf)]
async fn main() {
    let mut ship = ship::Ship::default();
    let mut laser_manager = laser::LaserManager::new(16, LASER_FIRE_PERIOD);
    let mut input_direction = Vec2::ZERO;
    let mut frame_status = FrameStatus::default();

    loop {
        frame_status.update();

        // Handle input
        input::handle_input_direction(&mut input_direction);
        input::handle_input_fire(&ship, &mut laser_manager, frame_status.dt);

        // Update
        ship.update(input_direction, &frame_status);
        laser_manager.update(&frame_status);

        // Draw
        clear_background(BLACK);
        ship.draw(input_direction);
        laser_manager.draw();

        // Draw debug texts
        // draw_text_speed(ship.speed);
        // draw_text_rotation(ship.rotation_rad);
        draw_text_fps(&frame_status);

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

#[derive(Default)]
struct FrameStatus {
    screen_size: Vec2,
    dt: f32,
}

impl FrameStatus {
    fn update(&mut self) {
        self.dt = get_frame_time();
        self.screen_size.x = screen_width();
        self.screen_size.y = screen_height();
    }
}

#[allow(dead_code)]
fn draw_text_speed(speed: Vec2) {
    let speed_text = "Speed: ".to_string() + &speed.to_string();
    draw_text(&speed_text, 0.0, 16.0, 30.0, GRAY);
}

#[allow(dead_code)]
fn draw_text_rotation(rot: f32) {
    let speed_text = "Rotation: ".to_string() + &rot.to_string();
    draw_text(&speed_text, 0.0, 16.0, 30.0, GRAY);
}

#[allow(dead_code)]
fn draw_text_fps(frame: &FrameStatus) {
    let fps_text = String::from("FPS: ") + &((1.0 / frame.dt) as i32).to_string();
    draw_text(&fps_text, frame.screen_size.x - 120.0, 16.0, 30.0, GRAY);
}
