use macroquad::prelude::*;

#[macroquad::main("SimpleGame")]
async fn main() {
    const ACCELERATION: f32 = 100.0;
    const MAX_VELOCITY: f32 = 200.0;

    let mut speed = Vec2::ZERO;
    let mut pos = vec2(screen_width() / 2.0, screen_height() / 2.0);

    loop {
        clear_background(RED);

        let mut direction = Vec2::ZERO;

        if is_key_down(KeyCode::Right) {
            direction.x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }

        let dt: f32 = get_frame_time();
        if direction != Vec2::ZERO {
            direction = direction.normalize();
            speed += direction * ACCELERATION * dt;
            speed = speed.clamp_length_max(MAX_VELOCITY);
        }
        pos += speed * dt;

        draw_circle(pos.x, pos.y, 15.0, YELLOW);
        let speed_text = "Speed: ".to_string() + &speed.to_string();
        draw_text(&speed_text, 0.0, 16.0, 30.0, BLACK);

        // let fps_text = String::from("FPS: ") + &get_fps().to_string();
        // draw_text(&fps_text, screen_width() - 120.0, 16.0, 30.0, BLACK);

        next_frame().await
    }
}
