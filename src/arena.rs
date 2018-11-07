use nannou::prelude::*;
use nannou::draw::Draw;

use particle::Particle;
use camera::Camera;
use barneshut::Node;
use barneshut::construct_tree;

pub struct Arena {
    pub particles: Vec<Particle>,
    tree: Option<Node>,

    width: f32,
    height: f32,

    vel_indicator: bool,
    accel_indicator: bool,
    tree_indicator: bool,

    freeze: bool,

    id_counter: u32,
}

impl Arena {
    pub fn new(width: f32, height: f32) -> Self {
        Arena { particles: Vec::new(), tree: None, width: width, height: height, vel_indicator: false, accel_indicator: false, tree_indicator: false, freeze: false, id_counter: 0 }
    }

    pub fn reset(&mut self) {
        let n = self.particles.len();
        self.particles.clear();
        self.big_bang(n);
    }

    pub fn toggle_freeze(&mut self) {
        self.freeze = !self.freeze;
    }

    pub fn big_bang(&mut self, n: usize) {
        for _ in 0..n {
            let r = random_f32() * 10000.0;
            let r = r.sqrt();
            let theta = random_f32() * 2.0 * std::f32::consts::PI;

            let x = r * theta.sin();
            let y = r * theta.cos();

            self.add_particle(Vector2::new(x,y), Vector2::new(-y * 5.0, x * 5.0));
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

    pub fn toggle_tree_indicator(&mut self) {
        self.tree_indicator = !self.tree_indicator;
    }

    pub fn add_particle(&mut self, pos: Vector2, vel: Vector2) {
        let particle = Particle::new(self.id_counter, pos, vel);
        self.id_counter += 1;

        self.particles.push(particle);
    }

    pub fn draw(&self, draw: &Draw, camera: &Camera) {
        if self.tree_indicator {
            if let Some(ref tree) = self.tree {
                tree.draw(draw, camera, self.width, self.height);
            }
        }

        for a in &self.particles {
            a.draw(draw, camera, self.vel_indicator, self.accel_indicator, self.width, self.height);
        }
    }

    pub fn update(&mut self) {
        let tree = construct_tree(&self.particles[..]);
        self.tree = Some(tree);

        //for a in &mut self.particles {
        //    a.update(&particles_copy);
        //}

        if let Some(ref tree) = self.tree {
            for a in &mut self.particles {
                let force = tree.compute_force(a.pos, 0.2);
                a.accel = force;
            }
        }
    }


    pub fn step(&mut self, dt: f32) {
        if self.freeze {
            return;
        }

        for a in &mut self.particles {
            a.step(dt);
        }
    }

}
