use macroquad::prelude::*;
use macroquad_test::lerp;

pub trait Teleport {
    fn speed(&self) -> Vec2;
    fn position(&self) -> Vec2;
    fn set_position(&mut self, x: f32, y: f32);

    fn teleport(&mut self, radius: f32) {
        let pos = self.position();
        let (screen_width, screen_height) = (screen_width(), screen_height());

        let off_screen_left = pos.x < -radius;
        let off_screen_right = pos.x > screen_width + radius;
        let off_screen_up = pos.y < -radius;
        let off_screen_down = pos.y > screen_height + radius;
        if !off_screen_left && !off_screen_right && !off_screen_up && !off_screen_down {
            return;
        }

        let abs_speed_direction = self.speed().normalize_or_zero().abs();

        let lerp_mult = abs_speed_direction.y / 1.0;
        let pos_y_lerp = || lerp(pos.y, screen_height - pos.y, lerp_mult);
        if off_screen_left {
            self.set_position(screen_width + radius, pos_y_lerp());
        } else if off_screen_right {
            self.set_position(-radius, pos_y_lerp());
        }

        let lerp_mult = abs_speed_direction.x / 1.0;
        let pos_x_lerp = || lerp(pos.x, screen_width - pos.x, lerp_mult);
        if off_screen_up {
            self.set_position(pos_x_lerp(), screen_height + radius);
        } else if off_screen_down {
            self.set_position(pos_x_lerp(), -radius);
        }
    }
}
