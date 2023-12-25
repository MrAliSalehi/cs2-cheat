use egui::Ui;
use crate::gui::OverlayTab;
#[derive(Clone)]
pub struct GeneralSetting {
    pub show_borders: bool,
}

impl OverlayTab for GeneralSetting {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.label("general settings");
        ui.checkbox(&mut self.show_borders, "show border");
    }
}

impl Default for GeneralSetting {
    fn default() -> Self {
        Self { show_borders: false }
    }
}