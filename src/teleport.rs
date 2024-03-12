use macroquad::prelude::*;
use macroquad_test::lerp;

pub trait Teleport {
    fn speed_dir(&self) -> Vec2;
    fn position_mut(&mut self) -> &mut Vec2;

    fn teleport(&mut self, screen_size: Vec2, offset: f32) {
        let pos = *self.position_mut();

        let off_screen_left = pos.x < -offset;
        let off_screen_right = pos.x > screen_size.x + offset;
        let off_screen_up = pos.y < -offset;
        let off_screen_down = pos.y > screen_size.y + offset;
        if !off_screen_left && !off_screen_right && !off_screen_up && !off_screen_down {
            return;
        }

        let abs_speed_direction = self.speed_dir().abs();
        let pos = self.position_mut();

        let lerp_mult = abs_speed_direction.y / 1.0;
        let pos_y_lerp = || lerp(pos.y, screen_size.y - pos.y, lerp_mult);
        if off_screen_left {
            *pos = vec2(screen_size.x + offset, pos_y_lerp());
        } else if off_screen_right {
            *pos = vec2(-offset, pos_y_lerp());
        }

        let lerp_mult = abs_speed_direction.x / 1.0;
        let pos_x_lerp = || lerp(pos.x, screen_size.x - pos.x, lerp_mult);
        if off_screen_up {
            *pos = vec2(pos_x_lerp(), screen_size.y + offset);
        } else if off_screen_down {
            *pos = vec2(pos_x_lerp(), -offset);
        }
    }
}
