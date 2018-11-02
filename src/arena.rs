use nannou::prelude::*;
use nannou::draw::Draw;

use particle::Particle;
use camera::Camera;

pub struct Arena {
    particles: Vec<Particle>,

    width: f32,
    height: f32,

    id_counter: u32,
}

impl Arena {
    pub fn new(width: f32, height: f32) -> Self {
        Arena { particles: Vec::new(), width: width, height: height, id_counter: 0 }
    }

    pub fn uniform(mut self, n: u32) -> Self {
        for _ in 0..n {
            let x = (random_f32() * self.width) - (self.width / 2.0);
            let y = (random_f32() * self.height) - (self.height / 2.0);

            self.add_particle(Vector2::new(x,y), Vector2::new(0.0, 0.0));
        }

        self
    }

    pub fn update_size(&mut self, size: Vector2) {
        self.width = size.x;
        self.height = size.y;
    }

    pub fn add_particle(&mut self, pos: Vector2, vel: Vector2) {
        let particle = Particle::new(self.id_counter, pos, vel);
        self.id_counter += 1;

        self.particles.push(particle);
    }

    pub fn draw(&self, draw: &Draw, camera: &Camera) {
        for a in &self.particles {
            a.draw(draw, camera);
        }
    }

    pub fn update(&mut self) {
        let agents_copy = self.particles.clone();

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
