use nannou::prelude::*;
use nannou::draw::Draw;

use camera::Camera;

#[derive(Clone)]
pub struct Particle {
    pub id: u32,
    pub pos: Vector2,
    pub vel: Vector2,
    pub accel: Vector2,
}

impl Particle {
    pub fn new(id: u32, pos: Vector2, vel: Vector2) -> Self {
        Self { id: id, pos: pos, vel: vel, accel: Vector2::new(0.0, 0.0) }
    }

    pub fn draw(&self, draw: &Draw, camera: &Camera, vel_indicator: bool, accel_indicator: bool, viewport_width: f32, viewport_height: f32) {
        let ul = camera.lookat + Vector2::new(-viewport_width / 2.0, viewport_height / 2.0) / camera.zoom;
        let lr = camera.lookat + Vector2::new(viewport_width / 2.0, -viewport_height / 2.0) / camera.zoom;

        const RADIUS: f32 = 3.0;

        // Only draw points inside the viewport
        if self.pos.x >= ul.x && self.pos.x <= lr.x && self.pos.y >= lr.y && self.pos.y <= ul.y {
            let pos = (self.pos - camera.lookat) * camera.zoom;
            let radius = (RADIUS * camera.zoom).max(1.0);
            draw.ellipse().resolution(10).xy(pos).radius(radius).color(BLACK);

            if vel_indicator {
                draw.line().start(pos).end(pos + (self.vel.normalize() * 15.0 * camera.zoom)).thickness(1.0 * camera.zoom).caps_round().color(BLUE);
            }

            if accel_indicator {
                draw.line().start(pos).end(pos + (self.accel.normalize() * 15.0 * camera.zoom)).thickness(1.0 * camera.zoom).caps_round().color(RED);
            }
        }
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
