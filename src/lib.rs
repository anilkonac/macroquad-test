use std::f32::consts::PI;

use macroquad::prelude::*;

pub const SQRT_3: f32 = 1.73205080757f32;
pub const DEG_TO_RAD: f32 = PI / 180.0f32;

#[inline]
pub fn draw_line_w_rot(rot_vec: Vec2, pos: Vec2, v1: Vec2, v2: Vec2, thickness: f32, color: Color) {
    let v_11 = rot_vec.rotate(v1) + pos;
    let v_22 = rot_vec.rotate(v2) + pos;
    draw_line(v_11.x, v_11.y, v_22.x, v_22.y, thickness, color);
}

// Modified version of the macroquad::shapes::draw_triangle function
/// Draws a solid triangle between points `v1`, `v2`, and `v3` with  given `colors`.
pub fn draw_triangle(v1: Vec2, v2: Vec2, v3: Vec2, colors: &[Color; 3]) {
    let context = unsafe { get_internal_gl() }.quad_gl;

    let vertices = [
        Vertex::new(v1.x, v1.y, 0., 0., 0., colors[0]),
        Vertex::new(v2.x, v2.y, 0., 0., 0., colors[1]),
        Vertex::new(v3.x, v3.y, 0., 0., 0., colors[2]),
    ];

    let indices: [u16; 3] = [0, 1, 2];

    context.texture(None);
    context.draw_mode(DrawMode::Triangles);
    context.geometry(&vertices, &indices);
}
