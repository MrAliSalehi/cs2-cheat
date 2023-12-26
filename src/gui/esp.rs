use egui::{Pos2, Rounding, Ui, Vec2};
use winapi::shared::windef::RECT;
use crate::gui::{OverlayTab};

#[derive(Clone)]
pub struct Esp {
    pub rounding: Rounding,
    pub team_box: bool,
    pub area_pos: Pos2,
    pub area_size: Vec2,
    pub game_rect: RECT,
    pub enabled: bool,
}

impl OverlayTab for Esp {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.label("esp");

        ui.checkbox(&mut self.enabled, "Enable");
        ui.checkbox(&mut self.team_box, "show teammate");
    }
}

impl Default for Esp {
    fn default() -> Self {
        Self {
            rounding: Rounding::from(2.0),
            team_box: false,
            area_pos: Pos2::default(),
            area_size: Vec2::default(),
            game_rect: RECT::default(),
            enabled: true,
        }
    }
}