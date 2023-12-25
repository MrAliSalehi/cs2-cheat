use std::{os::windows::ffi::OsStrExt, iter::once, ffi::OsStr, ptr::null, thread::sleep, time::Duration};
use egui::Ui;
use egui_overlay::egui_render_three_d::three_d::Zero;
use nalgebra::Vector3;
use winapi::shared::windef::RECT;
use winapi::um::winuser::FindWindowW;
use crate::globals::{LOCAL_PLAYER, };

pub mod cs2_overlay;
pub mod esp;
pub mod misc;
pub mod setting;
pub trait OverlayTab {
    fn render_ui(&mut self, ui: &mut Ui);
}

#[derive(PartialEq,Clone)]
pub enum Tabs {
    Esp,
    Aim,
    Misc,
    Gsettings,
}

pub fn world_to_screen(v: Vector3<f32>,game_rect: &RECT) -> Option<Vector3<f32>> {
    let g_matrix = LOCAL_PLAYER.lock().unwrap();
    let matrix = g_matrix.view_matrix.data.0;
    drop(g_matrix);

    let (right, bottom) = (game_rect.right as f32, game_rect.bottom as f32);
    let w = matrix[3][0] * v.x + matrix[3][1] * v.y + matrix[3][2] * v.z + matrix[3][3];
    if w < 0.01f32 {
        return None;
    }

    let mut _x = matrix[0][0] * v.x + matrix[0][1] * v.y + matrix[0][2] * v.z + matrix[0][3];
    let mut _y = matrix[1][0] * v.x + matrix[1][1] * v.y + matrix[1][2] * v.z + matrix[1][3];

    let inv_w = 1f32 / w;

    _x *= inv_w;
    _y *= inv_w;


    let mut x = right * 0.5f32;
    let mut y = bottom * 0.5f32;

    x += 0.5f32 * _x * right + 0.5f32;
    y -= 0.5f32 * _y * bottom + 0.5f32;

    Some(Vector3::new(x, y, w))
}

/*pub fn cs2_coordination() -> RECT {

}*/