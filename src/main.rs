extern crate nannou;

mod agent;
mod arena;

use nannou::prelude::*;
use nannou::ui::prelude::*;
use nannou::event::SimpleWindowEvent;

use arena::Arena;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
    arena: Arena,

    ui: Ui,
    fps_id: widget::Id,
    ui_last_update: f32,
}

fn model(app: &App) -> Model {
    let _ = app.new_window().with_title("Flocking").build().unwrap();
    let mut ui = app.new_ui().build().unwrap();

    let (width, height) = app.main_window().inner_size_points();

    let fps_id = ui.generate_widget_id();

    let mut arena = Arena::new(width, height);

    for _ in 0..200 {
        arena.add_agent();
    }

    Model { arena: arena, ui: ui, fps_id: fps_id, ui_last_update: 0.0 }
}

fn event(_: &App, mut model: Model, event: Event) -> Model {
    let ui_update_interval = 0.5;

    match event {
        Event::Update(update) => {
            let dt = update.since_last.secs() as f32;

            model.arena.update();
            model.arena.step(dt);

            model.ui_last_update += dt;

            if model.ui_last_update > ui_update_interval {
                model.ui_last_update = 0.0;

                let ui = &mut model.ui.set_widgets();

                let mut fps = (1.0/dt).round().to_string();

                widget::Text::new(&fps)
                    .right_justify()
                    .top_right_with_margin(5.0)
                    .color(ui::Color::Rgba(0.0, 0.0, 0.0, 1.0))
                    .set(model.fps_id, ui);
            }


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

    model.arena.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(app, &frame).unwrap();
    frame
}
