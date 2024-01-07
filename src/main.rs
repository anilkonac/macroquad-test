use macroquad::prelude::*;

#[macroquad::main("SimpleGame")]
async fn main() {
    const ACCELERATION: f32 = 250.0;
    const MAX_VELOCITY: f32 = 300.0;

    let mut speed = Vec2::ZERO;
    let mut pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut direction = Vec2::ZERO;

    loop {
        clear_background(BLACK);

        // Handle input
        get_input_direction(&mut direction);

        // Update speed and position
        let dt = get_frame_time();
        if direction != Vec2::ZERO {
            speed += direction.normalize() * ACCELERATION * dt;
            speed = speed.clamp_length_max(MAX_VELOCITY);
        }
        pos += speed * dt;

        // Draw
        draw_player(&pos);
        draw_text_speed(&speed);
        // draw_text_fps();

        next_frame().await
    }
}

fn get_input_direction(direction: &mut Vec2) {
    *direction = Vec2::ZERO;

    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        direction.x += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        direction.x -= 1.0;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        direction.y -= 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        direction.y += 1.0;
    }
}

fn draw_player(pos: &Vec2) {
    static RADIUS: f32 = 20.0;
    static R_COS30: f32 = RADIUS * 0.86602540378;
    static R_SIN30: f32 = RADIUS * 0.5;
    let v1 = vec2(pos.x, pos.y - RADIUS);
    let v2 = vec2(pos.x - R_COS30, pos.y + R_SIN30);
    let v3 = vec2(pos.x + R_COS30, pos.y + R_SIN30);
    draw_triangle_lines(v1, v2, v3, 2.0, WHITE);
}

fn draw_text_speed(speed: &Vec2) {
    let speed_text = "Speed: ".to_string() + &speed.to_string();
    draw_text(&speed_text, 0.0, 16.0, 30.0, GRAY);
}

fn draw_text_fps() {
    let fps_text = String::from("FPS: ") + &get_fps().to_string();
    draw_text(&fps_text, screen_width() - 120.0, 16.0, 30.0, GRAY);
}
