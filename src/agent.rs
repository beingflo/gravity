use nannou::prelude::*;
use nannou::draw::Draw;

use arena::MAX_SPEED;
use arena::MAX_FORCE;

#[derive(Clone)]
pub struct Agent {
    id: u32,
    pos: Point2,
    vel: Vector2,
    accel: Vector2,
}

impl Agent {
    pub fn new(id: u32) -> Self {
        Agent { id: id, pos: Point2::new(0.0, 0.0), vel: Vector2::new(0.0, 0.0), accel: Vector2::new(0.0, 0.0) }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn set_vel(&mut self, vel: Vector2) {
        self.vel = vel;
    }

    pub fn draw(&self, draw: &Draw) {
        const RADIUS: f32 = 5.0;
        const LENGTH: f32 = 12.0;
        const WIDTH: f32 = 1.5;

        draw.ellipse().xy(self.pos).radius(RADIUS).color(BLACK);
        draw.line().start(self.pos).end(self.pos + (self.vel.normalize() * LENGTH)).thickness(WIDTH).caps_round().color(BLACK);
    }

    fn wrap_pos(&mut self, width: f32, height: f32) {
        if self.pos.x > width / 2.0 {
            self.pos.x -= width;
        }
        if self.pos.x < -width / 2.0 {
            self.pos.x += width;
        }

        if self.pos.y > height / 2.0 {
            self.pos.y -= height;
        }
        if self.pos.y < -height / 2.0 {
            self.pos.y += height;
        }
    }

    fn distance_squared(&self, other: &Agent, width: f32, height: f32) -> f32 {
        let x_dist_direct = (self.pos.x - other.pos.x).abs();
        let x_dist_indirect = width - (self.pos.x - other.pos.x).abs();

        let x_min = x_dist_direct.min(x_dist_indirect);

        let y_dist_direct = (self.pos.y - other.pos.y).abs();
        let y_dist_indirect = height - (self.pos.y - other.pos.y).abs();

        let y_min = y_dist_direct.min(y_dist_indirect);

        x_min*x_min + y_min*y_min

    }

    pub fn update(&mut self, neighbors: &[Agent], width: f32, height: f32) {
        let cohesion_coeff = 0.5;
        let alignment_coeff = 0.1;
        let separation_coeff = 50.0;

        let desired_sep = 20.0;
        let mut sep_steer = Vector2::new(0.0, 0.0);
        let mut sep_count = 0;

        let align_neighborhood = 50.0;
        let mut align_steer = Vector2::new(0.0, 0.0);
        let mut align_count = 0;

        let cohe_neighborhood = 200.0;
        let mut cohe_steer = Vector2::new(0.0, 0.0);
        let mut cohe_count = 0;

        for a in neighbors {
            if a.id == self.id {
                continue;
            }

            let distance = self.distance_squared(a, width, height);
            if distance < desired_sep * desired_sep {
                let diff = (self.pos - a.pos).normalize();
                let diff = diff / (distance.sqrt().max(0.1));
                sep_steer += diff;
                sep_count += 1;
            }

            if distance < align_neighborhood * align_neighborhood {
                align_steer += a.vel;
                align_count += 1;
            }

            if distance < cohe_neighborhood * cohe_neighborhood {
                cohe_steer += a.pos;
                cohe_count += 1;
            }
        }

        if sep_count > 0 {
            sep_steer /= sep_count as f32;
            self.accel += sep_steer * separation_coeff;
            self.accel = self.accel.limit_magnitude(MAX_FORCE);
        }

        if align_count > 0 {
            align_steer /= align_count as f32;
            self.accel += align_steer * alignment_coeff;
            self.accel = self.accel.limit_magnitude(MAX_FORCE);
        }

        if cohe_count > 0 {
            cohe_steer /= cohe_count as f32;
            self.accel += (cohe_steer - self.pos).normalize() * cohesion_coeff;
            self.accel = self.accel.limit_magnitude(MAX_FORCE);
        }
    }

    // Agents can only move in the direction they're oriented in
    pub fn step(&mut self, dt: f32, width: f32, height: f32) {
        self.accel = self.accel.limit_magnitude(MAX_FORCE);

        self.vel += self.accel*dt;
        self.vel = self.vel.limit_magnitude(MAX_SPEED);

        self.pos += self.vel*dt;

        self.wrap_pos(width, height);
    }
}
