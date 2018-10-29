use nannou::prelude::*;
use nannou::draw::Draw;

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
        const RADIUS: f32 = 3.0;

        draw.ellipse().resolution(10).xy(self.pos).radius(RADIUS).color(BLACK);
    }

    pub fn update(&mut self, neighbors: &[Particle]) {
        let mut force = Vector2::new(0.0, 0.0);

        // Not realistic
        let g = 5000.0;

        let eps = 5.0;

        for p in neighbors {
            if p.id == self.id {
                continue;
            }

            let diff = self.pos - p.pos;
            let r2 = (diff.x * diff.x) + (diff.y * diff.y);
            force += -(diff / (r2 + eps).powf(3.0 / 2.0)) * g;
        }

        self.accel = force;
    }

    pub fn step(&mut self, dt: f32) {
        self.vel += self.accel*dt;
        self.pos += self.vel*dt;
    }
}
