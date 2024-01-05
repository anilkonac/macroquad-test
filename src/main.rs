use macroquad::prelude::*;

#[macroquad::main("SimpleGame")]
async fn main() {
    const ACCELERATION: f32 = 250.0;
    const MAX_VELOCITY: f32 = 300.0;

    let mut speed = Vec2::ZERO;
    let mut pos = vec2(screen_width() / 2.0, screen_height() / 2.0);

    loop {
        clear_background(RED);

        let dt = get_frame_time();

        let direction = get_input_direction();
        if direction != Vec2::ZERO {
            let direction = direction.normalize();
            speed += direction * ACCELERATION * dt;
            speed = speed.clamp_length_max(MAX_VELOCITY);
        }

        pos += speed * dt;

        draw_circle(pos.x, pos.y, 15.0, YELLOW);
        print_text_speed(&speed);
        // print_text_fps();

        next_frame().await
    }
}

fn get_input_direction() -> Vec2 {
    let mut direction = Vec2::ZERO;

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

    direction
}

fn print_text_speed(speed: &Vec2) {
    let speed_text = "Speed: ".to_string() + &speed.to_string();
    draw_text(&speed_text, 0.0, 16.0, 30.0, BLACK);
}

fn print_text_fps() {
    let fps_text = String::from("FPS: ") + &get_fps().to_string();
    draw_text(&fps_text, screen_width() - 120.0, 16.0, 30.0, BLACK);
}
