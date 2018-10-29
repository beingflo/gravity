use nannou::prelude::*;
use nannou::draw::Draw;

use particle::Particle;

//pub const MAX_SPEED: f32 = 5.0;

pub struct Arena {
    agents: Vec<Particle>,

    width: f32,
    height: f32,

    id_counter: u32,
}

impl Arena {
    pub fn new(width: f32, height: f32) -> Self {
        Arena { agents: Vec::new(), width: width, height: height, id_counter: 0 }
    }

    pub fn update_size(&mut self, size: Vector2) {
        self.width = size.x;
        self.height = size.y;
    }

    pub fn add_particle(&mut self) {
        let mut agent = Particle::new(self.id_counter);
        self.id_counter += 1;

        let x = (random_f32() * self.width) - (self.width / 2.0);
        let y = (random_f32() * self.height) - (self.height / 2.0);

        agent.set_pos(x, y);

        //let v_x = (random_f32() * MAX_SPEED) - (MAX_SPEED / 2.0);
        //let v_y = (random_f32() * MAX_SPEED) - (MAX_SPEED / 2.0);

        //let vel = Vector2::new(v_x, v_y);
        let vel = Vector2::new(0.0, 0.0);

        agent.set_vel(vel);

        self.agents.push(agent);
    }

    pub fn draw(&self, draw: &Draw) {
        for a in &self.agents {
            a.draw(draw);
        }
    }

    pub fn update(&mut self) {
        let agents_copy = self.agents.clone();

        for a in &mut self.agents {
            a.update(&agents_copy);
        }
    }


    pub fn step(&mut self, dt: f32) {
        for a in &mut self.agents {
            a.step(dt);
        }
    }

}
