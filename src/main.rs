use macroquad::prelude::*;

#[macroquad::main("SimpleGame")]
async fn main() {
    const ACCELERATION: f32 = 250.0;
    const MAX_VELOCITY: f32 = 300.0;

    let mut speed = Vec2::ZERO;
    let mut pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut direction = Vec2::ZERO;
    let mut rotation = 0.0;

    loop {
        clear_background(BLACK);

        // Handle input
        get_input_direction(&mut direction, &mut rotation);

        // Update speed and position
        let dt = get_frame_time();
        if direction != Vec2::ZERO {
            speed += direction.normalize() * ACCELERATION * dt;
            speed = speed.clamp_length_max(MAX_VELOCITY);
        }
        pos += speed * dt;

        // Draw
        draw_player(pos, rotation);
        draw_text_speed(&speed);
        // draw_text_fps();

        next_frame().await
    }
}

fn get_input_direction(direction: &mut Vec2, rotation: &mut f32) {
    *direction = Vec2::ZERO;

    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        *rotation += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        *rotation -= 1.0;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        direction.y -= 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        direction.y += 1.0;
    }
}

fn draw_player(pos: Vec2, rot: f32) {
    static RADIUS: f32 = 20.0;
    static R_COS30: f32 = RADIUS * 0.86602540378;
    static R_SIN30: f32 = RADIUS * 0.5;
    static V1: Vec2 = vec2(0.0, -RADIUS);
    static V2: Vec2 = vec2(-R_COS30, R_SIN30);
    static V3: Vec2 = vec2(R_COS30, R_SIN30);

    let rot_rad = rot.to_radians();
    let rot_rad_cos = rot_rad.cos();
    let rot_rad_sin = rot_rad.sin();
    let rotation_matrix = mat2(
        vec2(rot_rad_cos, rot_rad_sin),
        vec2(-rot_rad_sin, rot_rad_cos),
    );

    let v1 = rotation_matrix.mul_vec2(V1) + pos;
    let v2 = rotation_matrix.mul_vec2(V2) + pos;
    let v3 = rotation_matrix.mul_vec2(V3) + pos;
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
