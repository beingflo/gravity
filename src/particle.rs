use nannou::prelude::*;
use nannou::draw::Draw;

use arena::MAX_SPEED;
use arena::MAX_FORCE;

#[derive(Clone)]
pub struct Particle {
    id: u32,
    pos: Point2,
    vel: Vector2,
    accel: Vector2,
}

impl Particle {
    pub fn new(id: u32) -> Self {
        Self { id: id, pos: Point2::new(0.0, 0.0), vel: Vector2::new(0.0, 0.0), accel: Vector2::new(0.0, 0.0) }
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

    fn distance_squared(&self, other: &Particle, width: f32, height: f32) -> f32 {
        let x_dist_direct = (self.pos.x - other.pos.x).abs();
        let x_dist_indirect = width - (self.pos.x - other.pos.x).abs();

        let x_min = x_dist_direct.min(x_dist_indirect);

        let y_dist_direct = (self.pos.y - other.pos.y).abs();
        let y_dist_indirect = height - (self.pos.y - other.pos.y).abs();

        let y_min = y_dist_direct.min(y_dist_indirect);

        x_min*x_min + y_min*y_min

    }

    pub fn update(&mut self, neighbors: &[Particle], width: f32, height: f32) {
    }

    pub fn step(&mut self, dt: f32, width: f32, height: f32) {
        self.accel = self.accel.limit_magnitude(MAX_FORCE);

        self.vel += self.accel*dt;
        self.vel = self.vel.limit_magnitude(MAX_SPEED);

        self.pos += self.vel*dt;

        self.wrap_pos(width, height);
    }
}
