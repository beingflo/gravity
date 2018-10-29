use nannou::prelude::*;
use nannou::draw::Draw;

use agent::Agent;

pub const MAX_SPEED: f32 = 50.0;
pub const MAX_FORCE: f32 = 100.0;

pub struct Arena {
    agents: Vec<Agent>,

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

    pub fn add_agent(&mut self) {
        let mut agent = Agent::new(self.id_counter);
        self.id_counter += 1;

        let x = (random_f32() * self.width) - (self.width / 2.0);
        let y = (random_f32() * self.height) - (self.height / 2.0);

        agent.set_pos(x, y);

        let v_x = (random_f32() * MAX_SPEED) - (MAX_SPEED / 2.0);
        let v_y = (random_f32() * MAX_SPEED) - (MAX_SPEED / 2.0);

        let vel = Vector2::new(v_x, v_y);

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
            a.update(&agents_copy, self.width, self.height);
        }
    }


    pub fn step(&mut self, dt: f32) {
        for a in &mut self.agents {
            a.step(dt, self.width, self.height);
        }
    }

}
