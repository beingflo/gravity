use nannou::prelude::*;
use nannou::ui::prelude::*;

pub struct UserInterface {
    pub ui: Ui,
    fps_id: widget::Id,
    last_update: f32,
}

impl UserInterface {
    pub fn new(mut ui: Ui) -> Self {
        let fps_id = ui.generate_widget_id();
        UserInterface { ui: ui, fps_id: fps_id, last_update: 0.0 }
    }

    pub fn update(&mut self, dt: f32) {
        let ui_update_interval = 0.5;

        self.last_update += dt;

        if self.last_update > ui_update_interval {
            self.last_update = 0.0;

            let ui = &mut self.ui.set_widgets();

            let fps = (1.0/dt).round().to_string();

            widget::Text::new(&fps)
                .right_justify()
                .top_right_with_margin(5.0)
                .color(ui::Color::Rgba(0.0, 0.0, 0.0, 1.0))
                .set(self.fps_id, ui);
        }
    }
}
