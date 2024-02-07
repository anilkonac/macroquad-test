use macroquad::prelude::*;

#[inline]
pub fn draw_line_w_rot(rot_vec: Vec2, pos: Vec2, v1: Vec2, v2: Vec2, thickness: f32, color: Color) {
    let v_11 = rot_vec.rotate(v1) + pos;
    let v_22 = rot_vec.rotate(v2) + pos;
    draw_line(v_11.x, v_11.y, v_22.x, v_22.y, thickness, color);
}
