use macroquad::prelude::*;
use macroquad_test::lerp;

pub trait Teleport {
    fn speed_dir(&self) -> Vec2;
    fn position_mut(&mut self) -> &mut Vec2;

    fn teleport(&mut self, screen_size: Vec2, offset: f32) {
        let pos = *self.position_mut();

        let lerp_pos_y = || lerp(pos.y, screen_size.y - pos.y, self.speed_dir().abs().y / 1.0);
        if pos.x < -offset {
            *self.position_mut() = vec2(screen_size.x + offset, lerp_pos_y());
        } else if pos.x > screen_size.x + offset {
            *self.position_mut() = vec2(-offset, lerp_pos_y());
        }

        let lerp_pos_x = || lerp(pos.x, screen_size.x - pos.x, self.speed_dir().abs().x / 1.0);
        if pos.y < -offset {
            *self.position_mut() = vec2(lerp_pos_x(), screen_size.y + offset);
        } else if pos.y > screen_size.y + offset {
            *self.position_mut() = vec2(lerp_pos_x(), -offset);
        }
    }
}
