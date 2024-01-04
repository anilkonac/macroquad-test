use macroquad::prelude::*;

#[macroquad::main("SimpleGame")]
async fn main() {
    const ACCELERATION: f32 = 5.0;
    const MAX_VELOCITY: f32 = 10.0;

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

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            speed += direction * ACCELERATION * get_frame_time();
            speed = speed.clamp_length_max(MAX_VELOCITY);
        }
        pos += speed;

        draw_circle(pos.x, pos.y, 15.0, YELLOW);

        next_frame().await
    }
}
