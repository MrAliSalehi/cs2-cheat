use egui::Ui;
use crate::gui::OverlayTab;
#[derive(Clone)]
pub struct Misc {

}

impl OverlayTab for Misc {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.label("misc features");
    }
}