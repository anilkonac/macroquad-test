use macroquad::prelude::*;
use macroquad_test::{draw_line_w_rot, draw_triangle, normalize_rad, DEG_TO_RAD, SQRT_3};

use crate::{teleport::Teleport, FrameStatus, SHIP_ACCELERATION, SHIP_VELOCITY_MAX};

pub const SHIP_RADIUS: f32 = 20.0;
const FRAC_RADIUS_4: f32 = SHIP_RADIUS / 4.0;
const SRADIUS_COS30: f32 = SHIP_RADIUS * 0.86602540378f32;
const SRADIUS_SIN30: f32 = SHIP_RADIUS * 0.5;

const V_SHIP_TOP: Vec2 = vec2(SHIP_RADIUS, 0.0);
const V_SHIP_LEFT: Vec2 = vec2(-SRADIUS_SIN30, -SRADIUS_COS30);
const V_SHIP_RIGHT: Vec2 = vec2(-SRADIUS_SIN30, SRADIUS_COS30);
const V_FIRE_TOP: Vec2 = vec2(-SRADIUS_SIN30, 0.0);
const V_FIRE_BTM: Vec2 = vec2(-SRADIUS_SIN30 - 9.0, 0.0);

const V_FIRE_R_R_1: Vec2 = vec2(-FRAC_RADIUS_4, FRAC_RADIUS_4 * SQRT_3);
const V_FIRE_R_R_2: Vec2 = vec2(-V_FIRE_R_R_1.x + 6.0, V_FIRE_R_R_1.y);
const V_FIRE_R_L_1: Vec2 = vec2(V_FIRE_R_R_1.x, -V_FIRE_R_R_1.y);
const V_FIRE_R_L_2: Vec2 = vec2(V_FIRE_R_R_2.x, -V_FIRE_R_R_2.y);

const V_FIRE_RADIAL_TOP: Vec2 = vec2(SHIP_RADIUS, 0.0);
const V_FIRE_RADIAL_LEFT: Vec2 = vec2(SHIP_RADIUS, -6.0);
const V_FIRE_RADIAL_RIGHT: Vec2 = vec2(SHIP_RADIUS, 6.0);

const ACCELERATION_ANGULAR_RAD: f32 = crate::SHIP_ACCELERATION_ANGULAR * DEG_TO_RAD;
const MAX_VELOCITY_ANGULAR_RAD: f32 = crate::SHIP_VELOCITY_ANGULAR_MAX * DEG_TO_RAD;

const SHIP_VERTEX_COLORS: [Color; 3] = [PURPLE, WHITE, WHITE];
const CLR_THR: Color = BLUE;

pub struct Ship {
    pub pos: Vec2,
    pub speed: Vec2,
    pub rot_vec: Vec2,
    pub rotation_rad: f32,
    speed_angular_rad: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
            speed: Vec2::ZERO,
            rot_vec: Vec2::from_angle(0.0),
            rotation_rad: 0.0,
            speed_angular_rad: 0.0,
        }
    }
}

impl Teleport for Ship {
    #[inline(always)]
    fn speed_dir(&self) -> Vec2 {
        self.speed.normalize_or_zero()
    }

    #[inline(always)]
    fn position_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }
}

impl Ship {
    pub fn update(&mut self, input_direction: Vec2, frame: &FrameStatus) {
        self.pos += self.speed * frame.dt;
        self.rotation_rad += self.speed_angular_rad * frame.dt;
        self.rotation_rad = normalize_rad(self.rotation_rad);
        self.rot_vec = Vec2::from_angle(self.rotation_rad);
        self.teleport(frame.screen_size, SHIP_RADIUS);

        if input_direction != Vec2::ZERO {
            self.speed_angular_rad += input_direction.y * ACCELERATION_ANGULAR_RAD * frame.dt;
            self.speed_angular_rad = self
                .speed_angular_rad
                .clamp(-MAX_VELOCITY_ANGULAR_RAD, MAX_VELOCITY_ANGULAR_RAD);
            self.speed += self.rot_vec * input_direction.x * SHIP_ACCELERATION * frame.dt;
            self.speed = self.speed.clamp_length_max(SHIP_VELOCITY_MAX);
        }
    }

    pub fn draw(&self, input_dir: Vec2) {
        let rot_vec = self.rot_vec;

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
        draw_triangle(&[
            Vertex::new(v1.x, v1.y, 0., 0., 0., SHIP_VERTEX_COLORS[0]),
            Vertex::new(v2.x, v2.y, 0., 0., 0., SHIP_VERTEX_COLORS[1]),
            Vertex::new(v3.x, v3.y, 0., 0., 0., SHIP_VERTEX_COLORS[2]),
        ]);
    }
}
