use std::f32::consts::{FRAC_PI_2, PI};

use macroquad::prelude::*;
use macroquad_test::draw_line_w_rot;

const SQRT_3: f32 = 1.73205080757f32;
const DEG_TO_RAD: f32 = PI / 180.0f32;

pub const SHIP_RADIUS: f32 = 20.0;
const FRAC_RADIUS_4: f32 = SHIP_RADIUS / 4.0;
const SR_COS30: f32 = SHIP_RADIUS * 0.86602540378f32;
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
const CLR_THR: Color = BLUE;

const ACCELERATION: f32 = 100.0;
const ACCELERATION_ANGULAR: f32 = 100.0;
const ACCELERATION_ANGULAR_RAD: f32 = ACCELERATION_ANGULAR * DEG_TO_RAD;
const MAX_VELOCITY: f32 = 200.0;
const MAX_VELOCITY_ANGULAR: f32 = 200.0;
const MAX_VELOCITY_ANGULAR_RAD: f32 = MAX_VELOCITY_ANGULAR * DEG_TO_RAD;

pub struct Ship {
    pub pos: Vec2,
    pub speed: Vec2,
    pub rotation_rad: f32,
    speed_angular: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
            speed: Vec2::ZERO,
            rotation_rad: 0.0,
            speed_angular: 0.0,
        }
    }
}

impl Ship {
    pub fn update(&mut self, input_direction: Vec2, dt: f32) {
        if input_direction != Vec2::ZERO {
            self.speed_angular += input_direction.y * ACCELERATION_ANGULAR_RAD * dt;
            self.speed_angular = self
                .speed_angular
                .clamp(-MAX_VELOCITY_ANGULAR_RAD, MAX_VELOCITY_ANGULAR_RAD);
            let direction = Vec2::from_angle(-FRAC_PI_2 + self.rotation_rad);
            self.speed += direction * input_direction.x * ACCELERATION * dt;
            self.speed = self.speed.clamp_length_max(MAX_VELOCITY);
        }
        self.pos += self.speed * dt;
        self.rotation_rad += self.speed_angular * dt;
    }

    pub fn draw(&self, input_dir: Vec2) {
        let rot_vec = Vec2::from_angle(self.rotation_rad);

        // Draw the thrust of the engine
        if input_dir.x > 0.0 {
            // Forward thrust
            draw_line_w_rot(rot_vec, self.pos, V_FIRE_TOP, V_FIRE_BTM, 6.0, CLR_THR);
        } else if input_dir.x < 0.0 {
            // Backward thrust
            draw_line_w_rot(rot_vec, self.pos, V_FIRE_R_R_1, V_FIRE_R_R_2, 3.0, CLR_THR);
            draw_line_w_rot(rot_vec, self.pos, V_FIRE_R_L_1, V_FIRE_R_L_2, 3.0, CLR_THR);
        }

        // Draw radial thrust
        if input_dir.y.abs() > 0.0 {
            let v1 = rot_vec.rotate(V_FIRE_RADIAL_TOP) + self.pos;
            let v2 = match input_dir.y > 0.0 {
                true => rot_vec.rotate(V_FIRE_RADIAL_LEFT) + self.pos,
                false => rot_vec.rotate(V_FIRE_RADIAL_RIGHT) + self.pos,
            };
            draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, CLR_THR);
        }

        // Draw the ship
        let v1 = rot_vec.rotate(V_SHIP_TOP) + self.pos;
        let v2 = rot_vec.rotate(V_SHIP_LEFT) + self.pos;
        let v3 = rot_vec.rotate(V_SHIP_RIGHT) + self.pos;
        // draw_triangle_lines(v1, v2, v3, 2.0, COLOR_SHIP);
        draw_triangle(v1, v2, v3, COLOR_SHIP);
        // draw_poly_lines(self.position.x, self.position.y, 3, RADIUS, -90.0 + rot, 2.0, WHITE);
    }
}
