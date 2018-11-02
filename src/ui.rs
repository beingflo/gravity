use nannou::prelude::*;
use nannou::ui::prelude::*;

pub struct UserInterface {
    pub ui: Ui,
    fps_id: widget::Id,
    fps_text: String,
    console_id: widget::Id,
    console_text: String,
    show_console: bool,
    last_update: f32,
}

impl UserInterface {
    pub fn new(mut ui: Ui) -> Self {
        let fps_id = ui.generate_widget_id();
        let console_id = ui.generate_widget_id();
        UserInterface { ui: ui, fps_id: fps_id, fps_text: String::new(), console_id: console_id, console_text: String::new(), show_console: false, last_update: 0.0 }
    }

    pub fn toggle_console(&mut self) {
        self.show_console = !self.show_console;
    }

    pub fn update(&mut self, dt: f32) {
        let ui_update_interval = 0.5;
        let ui = &mut self.ui.set_widgets();

        self.last_update += dt;

        if self.last_update > ui_update_interval {
            self.last_update = 0.0;

            self.fps_text = (1.0/dt).round().to_string();
        }

        widget::Text::new(&self.fps_text)
            .right_justify()
            .top_right_with_margin(5.0)
            .color(ui::Color::Rgba(0.0, 0.0, 0.0, 1.0))
            .set(self.fps_id, ui);

        if self.show_console {
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
}
