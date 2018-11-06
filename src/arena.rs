use nannou::prelude::*;
use nannou::draw::Draw;

use particle::Particle;
use camera::Camera;

pub struct Arena {
    pub particles: Vec<Particle>,

    width: f32,
    height: f32,

    vel_indicator: bool,
    accel_indicator: bool,

    id_counter: u32,
}

impl Arena {
    pub fn new(width: f32, height: f32) -> Self {
        Arena { particles: Vec::new(), width: width, height: height, vel_indicator: false, accel_indicator: false, id_counter: 0 }
    }

    pub fn reset(&mut self) {
        let n = self.particles.len();
        self.particles.clear();
        self.big_bang(n);
    }

    pub fn big_bang(&mut self, n: usize) {
        for _ in 0..n {
            let r = random_f32() * 10000.0;
            let r = r.sqrt();
            let theta = random_f32() * 2.0 * std::f32::consts::PI;

            let x = r * theta.sin();
            let y = r * theta.cos();

            self.add_particle(Vector2::new(x,y), Vector2::new(-y * 3.0, x * 3.0));
        }
    }

    pub fn update_size(&mut self, size: Vector2) {
        self.width = size.x;
        self.height = size.y;
    }

    pub fn toggle_velocity_indicator(&mut self) {
        self.vel_indicator = !self.vel_indicator;
    }

    pub fn toggle_acceleration_indicator(&mut self) {
        self.accel_indicator = !self.accel_indicator;
    }

    pub fn add_particle(&mut self, pos: Vector2, vel: Vector2) {
        let particle = Particle::new(self.id_counter, pos, vel);
        self.id_counter += 1;

        self.particles.push(particle);
    }

    pub fn draw(&self, draw: &Draw, camera: &Camera) {
        for a in &self.particles {
            a.draw(draw, camera, self.vel_indicator, self.accel_indicator);
        }
    }

    pub fn update(&mut self) {
        let agents_copy = self.particles.clone();

        // TODO make this O(nlogn)
        for a in &mut self.particles {
            a.update(&agents_copy);
        }
    }


    pub fn step(&mut self, dt: f32) {
        for a in &mut self.particles {
            a.step(dt);
        }
    }

}
