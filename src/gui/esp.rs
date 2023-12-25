use std::ptr::null;
use std::slice::Iter;
use egui::{Align2, Color32, FontId, Order, Painter, Pos2, Rect, Rgba, Rounding, Sense, Stroke, Ui, vec2, Vec2};
use winapi::shared::windef::RECT;
use winapi::um::winuser::FindWindowW;
use crate::continue_if;
use crate::entity::Entity;
use crate::globals::{BONE_CONNECTIONS, ENTITY_LIST, LOCAL_PLAYER};
use crate::gui::{OverlayTab, world_to_screen};

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