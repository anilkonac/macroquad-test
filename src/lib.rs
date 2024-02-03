use macroquad::prelude::*;

const SQRT_3: f32 = 1.73205080757;

const SHIP_RADIUS: f32 = 20.0;
const FRAC_RADIUS_4: f32 = SHIP_RADIUS / 4.0;
const SR_COS30: f32 = SHIP_RADIUS * 0.86602540378;
const SR_SIN30: f32 = SHIP_RADIUS * 0.5;

const V_SHIP_TOP: Vec2 = vec2(0.0, -SHIP_RADIUS);
const V_SHIP_LEFT: Vec2 = vec2(-SR_COS30, SR_SIN30);
const V_SHIP_RIGHT: Vec2 = vec2(SR_COS30, SR_SIN30);
const V_FIRE_TOP: Vec2 = vec2(0.0, SR_SIN30);
const V_FIRE_BTM: Vec2 = vec2(0.0, SR_SIN30 + 9.0);

const V_FIRE_R_R_1: Vec2 = vec2(FRAC_RADIUS_4 * SQRT_3, -FRAC_RADIUS_4);
const V_FIRE_R_R_2: Vec2 = vec2(FRAC_RADIUS_4 * SQRT_3, -FRAC_RADIUS_4 - 6.0);
const V_FIRE_R_L_1: Vec2 = vec2(-V_FIRE_R_R_1.x, V_FIRE_R_R_1.y);
const V_FIRE_R_L_2: Vec2 = vec2(-V_FIRE_R_R_2.x, V_FIRE_R_R_2.y);

const V_FIRE_RADIAL_TOP: Vec2 = vec2(0.0, -SHIP_RADIUS);
const V_FIRE_RADIAL_LEFT: Vec2 = vec2(-6.0, -SHIP_RADIUS);
const V_FIRE_RADIAL_RIGHT: Vec2 = vec2(6.0, -SHIP_RADIUS);

const COLOR_SHIP: Color = WHITE;
const COLOR_THRUST: Color = BLUE;

pub fn draw_player(pos: Vec2, rot: f32, input_dir: Vec2) {
    let rot_rad = rot.to_radians();
    let rot_mat = Mat2::from_angle(rot_rad);

    // Draw the thrust of the engine
    if input_dir.x > 0.0 {
        // Forward thrust
        draw_line_w_rot(&rot_mat, pos, V_FIRE_TOP, V_FIRE_BTM, 6.0, COLOR_THRUST);
    } else if input_dir.x < 0.0 {
        // Backward thrust
        draw_line_w_rot(&rot_mat, pos, V_FIRE_R_R_1, V_FIRE_R_R_2, 3.0, COLOR_THRUST);
        draw_line_w_rot(&rot_mat, pos, V_FIRE_R_L_1, V_FIRE_R_L_2, 3.0, COLOR_THRUST);
    }

    // Draw radial thrust
    if input_dir.y.abs() > 0.0 {
        let v1 = rot_mat.mul_vec2(V_FIRE_RADIAL_TOP) + pos;
        let v2 = match input_dir.y > 0.0 {
            true => rot_mat.mul_vec2(V_FIRE_RADIAL_LEFT) + pos,
            false => rot_mat.mul_vec2(V_FIRE_RADIAL_RIGHT) + pos,
        };
        draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, COLOR_THRUST);
    }

    // Draw the ship
    let v1 = rot_mat.mul_vec2(V_SHIP_TOP) + pos;
    let v2 = rot_mat.mul_vec2(V_SHIP_LEFT) + pos;
    let v3 = rot_mat.mul_vec2(V_SHIP_RIGHT) + pos;
    // draw_triangle_lines(v1, v2, v3, 2.0, COLOR_SHIP);
    draw_triangle(v1, v2, v3, COLOR_SHIP);
    // draw_poly_lines(pos.x, pos.y, 3, RADIUS, -90.0 + rot, 2.0, WHITE);
}

#[inline]
fn draw_line_w_rot(rot_mat: &Mat2, pos: Vec2, v1: Vec2, v2: Vec2, thickness: f32, color: Color) {
    let v_11 = rot_mat.mul_vec2(v1) + pos;
    let v_22 = rot_mat.mul_vec2(v2) + pos;
    draw_line(v_11.x, v_11.y, v_22.x, v_22.y, thickness, color);
}
