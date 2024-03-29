use macroquad::prelude::*;
use std::f32::consts::PI;

pub mod core;

const PI_2: f32 = PI * 2.0;
pub const SQRT_3: f32 = 1.73205080757f32;
pub const DEG_TO_RAD: f32 = PI / 180.0f32;

#[inline]
pub fn draw_line_w_rot(rot_vec: Vec2, pos: Vec2, v1: Vec2, v2: Vec2, thickness: f32, color: Color) {
    let v_11 = rot_vec.rotate(v1) + pos;
    let v_22 = rot_vec.rotate(v2) + pos;
    draw_line(v_11.x, v_11.y, v_22.x, v_22.y, thickness, color);
}

pub fn draw_triangle(vertices: &[Vertex; 3]) {
    static INDICES: [u16; 3] = [0, 1, 2];

    let context = unsafe { get_internal_gl() }.quad_gl;
    context.texture(None);
    context.draw_mode(DrawMode::Triangles);
    context.geometry(vertices, &INDICES);
}

#[inline]
pub fn normalize_rad(rad_val: f32) -> f32 {
    if rad_val < 0.0 {
        return rad_val + PI_2;
    }
    if rad_val > PI_2 {
        return rad_val - PI_2;
    }
    rad_val
}

#[inline(always)]
pub fn lerp(a: f32, b: f32, alpha: f32) -> f32 {
    a * (1. - alpha) + b * alpha
}
