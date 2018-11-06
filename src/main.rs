extern crate nannou;

mod particle;
mod arena;
mod camera;
mod ui;
mod barneshut;

use nannou::prelude::*;
use nannou::event::SimpleWindowEvent;

use arena::Arena;
use camera::Camera;
use ui::UserInterface;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
    arena: Arena,
    camera: Camera,
    ui: UserInterface,
}

fn model(app: &App) -> Model {
    let _ = app.new_window().with_title("Flocking").build().unwrap();
    let (width, height) = app.main_window().inner_size_points();

    let ui = app.new_ui().build().unwrap();
    let ui = UserInterface::new(ui);

    let mut arena = Arena::new(width, height);
    arena.big_bang(1000);

    Model { arena: arena, camera: Camera::new(), ui: ui }
}

fn event(_: &App, mut model: Model, event: Event) -> Model {
    if let Some(id) = model.camera.following {
        model.camera.follow(model.arena.particles[id].pos);
    }

    model.camera.handle_event(&event);

    match event {
        Event::Update(update) => {
            let dt = update.since_last.secs() as f32;

            model.arena.update();
            model.arena.step(dt);
            model.ui.update(dt);
        },

        Event::WindowEvent { simple: Some(SimpleWindowEvent::KeyPressed(nannou::VirtualKeyCode::Tab)), .. } => {
            model.ui.toggle_console();
        },

        // Follow a particle
        Event::WindowEvent { simple: Some(SimpleWindowEvent::KeyPressed(nannou::VirtualKeyCode::F)), .. } => {
            if model.camera.following.is_none() {
                let mut min_dist = std::f32::MAX;
                let mut min_idx = 0;
                for (i, p) in model.arena.particles.iter().enumerate() {
                    let diff = p.pos - model.camera.lookat;
                    let dist = diff.x * diff.x + diff.y * diff.y;
                    if dist < min_dist {
                        min_dist = dist;
                        min_idx = i;
                    }
                }

                model.camera.following = Some(min_idx);
            } else {
                model.camera.following = None;
            }
        },

        Event::WindowEvent { simple: Some(SimpleWindowEvent::KeyPressed(nannou::VirtualKeyCode::R)), .. } => {
            model.arena.reset();
        },

        Event::WindowEvent { simple: Some(SimpleWindowEvent::KeyPressed(nannou::VirtualKeyCode::Space)), .. } => {
            model.arena.toggle_freeze();
        },

        Event::WindowEvent { simple: Some(SimpleWindowEvent::KeyPressed(nannou::VirtualKeyCode::V)), .. } => {
            model.arena.toggle_velocity_indicator();
        },

        Event::WindowEvent { simple: Some(SimpleWindowEvent::KeyPressed(nannou::VirtualKeyCode::A)), .. } => {
            model.arena.toggle_acceleration_indicator();
        },

        Event::WindowEvent { simple: Some(SimpleWindowEvent::Resized(size)), .. } => {
            model.arena.update_size(size);
        },

        _ => (),
    }
    model
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(WHITE);

    model.arena.draw(&draw, &model.camera);

    draw.to_frame(app, &frame).unwrap();
    model.ui.ui.draw_to_frame(app, &frame).unwrap();
    frame
}
