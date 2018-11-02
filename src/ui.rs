use nannou::prelude::*;
use nannou::ui::prelude::*;

pub struct UserInterface {
    pub ui: Ui,
    fps_id: widget::Id,
    console_id: widget::Id,
    console_text: String,
    last_update: f32,
}

impl UserInterface {
    pub fn new(mut ui: Ui) -> Self {
        let fps_id = ui.generate_widget_id();
        let console_id = ui.generate_widget_id();
        UserInterface { ui: ui, fps_id: fps_id, console_id: console_id, console_text: String::new(), last_update: 0.0 }
    }

    pub fn update(&mut self, dt: f32) {
        let ui_update_interval = 0.5;
        let ui = &mut self.ui.set_widgets();

        self.last_update += dt;

        if self.last_update > ui_update_interval {
            self.last_update = 0.0;

            let fps = (1.0/dt).round().to_string();

            widget::Text::new(&fps)
                .right_justify()
                .top_right_with_margin(5.0)
                .color(ui::Color::Rgba(0.0, 0.0, 0.0, 1.0))
                .set(self.fps_id, ui);

        }

        for text in widget::TextEdit::new(&self.console_text)
            .left_justify()
            .top_left_with_margin(5.0)
            .w_h(200.0, 20.0)
            .color(ui::Color::Rgba(0.0, 0.0, 0.0, 1.0))
            .set(self.console_id, ui) {

            self.console_text = text.into();
        }
    }
}
