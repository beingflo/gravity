use nannou::prelude::*;
use nannou::event::{ SimpleWindowEvent, MouseScrollDelta, MouseButton };

pub struct Camera {
    pub lookat: Vector2,
    pub zoom: f32,

    last_press: Option<Vector2>,
    pos: Vector2,
    pub following: Option<usize>,
}

impl Camera {
    pub fn new() -> Self {
        Camera { lookat: Vector2::new(0.0, 0.0), zoom: 1.0, last_press: None, pos: Vector2::new(0.0, 0.0), following: None }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match &event {
            Event::WindowEvent { simple: Some(SimpleWindowEvent::MouseWheel(MouseScrollDelta::LineDelta(_, y), _)), .. } => {
                let y = y * 0.1 + 1.0;

                self.zoom *= y;
            },

            Event::WindowEvent { simple: Some(SimpleWindowEvent::MouseMoved(pos)), .. } => {
                self.pos = *pos;

                if let Some(pos) = self.last_press {
                    self.lookat += (pos - self.pos) * (1.0 / self.zoom);
                    self.last_press = Some(self.pos);
                }
            },

            Event::WindowEvent { simple: Some(SimpleWindowEvent::MousePressed(MouseButton::Left)), .. } => {
                if self.last_press.is_none() {
                    self.last_press = Some(self.pos);
                }
            },

            Event::WindowEvent { simple: Some(SimpleWindowEvent::MouseReleased(MouseButton::Left)), .. } => {
                self.last_press = None;
            },

            _ => (),
        }
    }

    pub fn follow(&mut self, pos: Vector2) {
        self.lookat = pos;
    }
}
