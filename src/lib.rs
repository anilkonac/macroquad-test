use macroquad::prelude::*;

pub fn draw_player(pos: Vec2, rot: f32, input_dir: Vec2) {
    static RADIUS: f32 = 20.0;
    static R_COS30: f32 = RADIUS * 0.86602540378;
    static R_SIN30: f32 = RADIUS * 0.5;

    // static V_THROTTLE_BACK_1: Vec2 = vec2()

    let rot_rad = rot.to_radians();
    let rotation_matrix = Mat2::from_angle(rot_rad);

    // Draw engine impulse
    if input_dir.x > 0.0 {
        static V_THROTTLE_1: Vec2 = vec2(0.0, R_SIN30);
        static V_THROTTLE_2: Vec2 = vec2(0.0, R_SIN30 + 10.0);

        draw_line_with_rotation(&rotation_matrix, pos, V_THROTTLE_1, V_THROTTLE_2, 6.0, RED);
    } else if input_dir.x < 0.0 {
        static SQRT_3: f32 = 1.73205080757;
        static V_THROTTLE_BACK_1: Vec2 = vec2((RADIUS / 4.0) * SQRT_3, -RADIUS / 4.0);
        static V_THROTTLE_BACK_2: Vec2 = vec2((RADIUS / 4.0) * SQRT_3, -(RADIUS / 4.0) - 6.0);
        static V_THROTTLE_BACK_3: Vec2 = vec2(-(RADIUS / 4.0) * SQRT_3, -RADIUS / 4.0);
        static V_THROTTLE_BACK_4: Vec2 = vec2(-(RADIUS / 4.0) * SQRT_3, -(RADIUS / 4.0) - 6.0);

        draw_line_with_rotation(
            &rotation_matrix,
            pos,
            V_THROTTLE_BACK_1,
            V_THROTTLE_BACK_2,
            3.0,
            RED,
        );
        draw_line_with_rotation(
            &rotation_matrix,
            pos,
            V_THROTTLE_BACK_3,
            V_THROTTLE_BACK_4,
            3.0,
            RED,
        );
    }

    if input_dir.y.abs() > 0.0 {
        static V_THROTTLE_RADIAL_1: Vec2 = vec2(0.0, -RADIUS);
        let v_th_r_1 = rotation_matrix.mul_vec2(V_THROTTLE_RADIAL_1) + pos;
        if input_dir.y > 0.0 {
            static V_THROTTLE_RADIAL_2: Vec2 = vec2(-6.0, -RADIUS);

            let v_th_r_2 = rotation_matrix.mul_vec2(V_THROTTLE_RADIAL_2) + pos;
            draw_line(v_th_r_1.x, v_th_r_1.y, v_th_r_2.x, v_th_r_2.y, 2.0, RED);
        } else if input_dir.y < 0.0 {
            static V_THROTTLE_RADIAL_3: Vec2 = vec2(6.0, -RADIUS);

            let v_th_r_2 = rotation_matrix.mul_vec2(V_THROTTLE_RADIAL_3) + pos;
            draw_line(v_th_r_1.x, v_th_r_1.y, v_th_r_2.x, v_th_r_2.y, 2.0, RED);
        }
    }

    static V1: Vec2 = vec2(0.0, -RADIUS);
    static V2: Vec2 = vec2(-R_COS30, R_SIN30);
    static V3: Vec2 = vec2(R_COS30, R_SIN30);

    let v1 = rotation_matrix.mul_vec2(V1) + pos;
    let v2 = rotation_matrix.mul_vec2(V2) + pos;
    let v3 = rotation_matrix.mul_vec2(V3) + pos;
    draw_triangle_lines(v1, v2, v3, 2.0, WHITE);
    // draw_poly_lines(pos.x, pos.y, 3, RADIUS, -90.0 + rot, 2.0, WHITE);
}

fn draw_line_with_rotation(
    rot_mat: &Mat2,
    pos: Vec2,
    v1: Vec2,
    v2: Vec2,
    thickness: f32,
    color: Color,
) {
    let v_11 = rot_mat.mul_vec2(v1) + pos;
    let v_22 = rot_mat.mul_vec2(v2) + pos;

    draw_line(v_11.x, v_11.y, v_22.x, v_22.y, thickness, color);
}
