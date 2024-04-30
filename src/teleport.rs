use macroquad::prelude::*;
use macroquad_test::lerp;

pub trait Teleport {
    fn speed_dir(&self) -> Vec2;
    fn position_mut(&mut self) -> &mut Vec2;

    fn teleport(&mut self, screen_size: Vec2, offset: f32) {
        let speed_dir = self.speed_dir();
        let pos = self.position_mut();

        let lerp_pos_y = || lerp(pos.y, screen_size.y - pos.y, speed_dir.y.abs() / 1.0);
        if pos.x < -offset {
            *pos = vec2(screen_size.x + offset, lerp_pos_y());
        } else if pos.x > screen_size.x + offset {
            *pos = vec2(-offset, lerp_pos_y());
        } else {
            let lerp_pos_x = || lerp(pos.x, screen_size.x - pos.x, speed_dir.x.abs() / 1.0);
            if pos.y < -offset {
                *pos = vec2(lerp_pos_x(), screen_size.y + offset);
            } else if pos.y > screen_size.y + offset {
                *pos = vec2(lerp_pos_x(), -offset);
            }
        }
    }
}
