use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null;
use std::slice::Iter;
use std::thread::sleep;
use std::time::Duration;
use egui::{Align2, Color32, FontId, Order, Painter, Pos2, Rect, Rgba, Rounding, Sense, Stroke, vec2, Window};
use egui_overlay::{egui_window_glfw_passthrough, EguiOverlay};
use egui_overlay::egui_render_three_d::ThreeDBackend;

use nalgebra::Vector3;
use winapi::um::winuser::FindWindowW;
use crate::{continue_if, ENTITY_LIST, LOCAL_PLAYER};
use crate::entity::Entity;
use crate::globals::{BONE_CONNECTIONS, WINDOW_POS};

pub struct CsOverlay {
    pub frame: u32,
    pub show_borders: bool,
    pub rounding: Rounding,
    pub team_box: bool,
}
impl EguiOverlay for CsOverlay {
    fn gui_run(&mut self, egui_context: &egui::Context, _: &mut ThreeDBackend, glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend) {
        sleep(Duration::from_millis(4));
        catppuccin_egui::set_theme(egui_context,catppuccin_egui::MOCHA);
        let cs_size = WINDOW_POS.lock().unwrap();
        let game_bound_y = cs_size.top;
        let game_bound_x = cs_size.left;

        let game_bound_right = cs_size.right;
        let game_bound_bottom = cs_size.bottom;

        drop(cs_size);

        glfw_backend.window.set_pos(0, 0);
        glfw_backend.window.set_size(game_bound_right, game_bound_bottom);

        let area_pos = Pos2::new(game_bound_x as f32, game_bound_y as f32);
        let area_size = vec2(game_bound_right as f32, game_bound_bottom as f32);
        egui::Area::new("overlay")
            .interactable(false)
            .fixed_pos(area_pos)
            .order(Order::Background)
            .show(egui_context, |ui| {
                let (rect, _) = ui.allocate_at_least(area_size, Sense { focusable: false, drag: false, click: false });
                let painter = ui.painter();
                if self.show_borders {
                    painter.rect_stroke(rect, Rounding::from(3.0), Stroke::new(3.0, Color32::YELLOW));
                }

                let g_entities = ENTITY_LIST.lock().unwrap();
                let entities = g_entities.iter();

                let g_local_player = LOCAL_PLAYER.lock().unwrap();
                let local_player_team = g_local_player.entity.team_number;
                drop(g_local_player);

                self.draw_visuals(entities, local_player_team, painter);

                drop(g_entities);
            });

        Window::new("cs2 external cheat")
            .resizable(true)
            .vscroll(true)
            .hscroll(true)
            .default_size([250.0, 150.0])
            .show(egui_context, |ui|
                {

                    ui.group(|ui|{
                        ui.label("esp");
                        ui.checkbox(&mut self.team_box, "show teammate");
                    });
                    ui.checkbox(&mut self.show_borders, "show border");

                    ui.allocate_space(ui.available_size());
                });


        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            glfw_backend.window.set_mouse_passthrough(false);
        } else {
            glfw_backend.window.set_mouse_passthrough(true);
        }
        egui_context.request_repaint();
    }
}

impl CsOverlay {
    fn draw_visuals(&self, entities: Iter<Entity>, local_player_team: u8, painter: &Painter) {
        for entity in entities {
            continue_if!(entity.health == 0);
            let Some(screen_pos) = world_to_screen(entity.origin) else { continue; };
            let Some(screen_head) = world_to_screen(entity.head) else { continue; };

            let height = screen_pos.y - screen_head.y;
            let width = height / 2.4f32;

            let g_local = LOCAL_PLAYER.lock().unwrap();
            let _distance = g_local.calc_distance_rounded(entity.origin);
            drop(g_local);

            //draw visuals
            let x = screen_head.x - width / 2.0;
            let y = screen_head.y;
            let w = width;
            let h = height;

            let color = if entity.team_number == local_player_team {
                Color32::from_rgba_premultiplied((255 - entity.health) as u8, (55 + entity.health * 2) as u8, (140 - entity.health) as u8, 255)
            } else {
                if !self.team_box { continue; }
                Color32::WHITE
            };

            //esp border position
            painter.rect_stroke(Rect::from_min_max((x, y).into(), (x + w, y + h).into()), self.rounding, Stroke::new(3.0, color));

            //esp name
            painter.text(Pos2::from((screen_head.x + (width / 2.5), screen_head.y)),
                         Align2::CENTER_BOTTOM,
                         format!("({})", entity.name),
                         FontId::monospace(10.0),
                         Color32::from(Rgba::BLUE));

            //health bar

            let x = screen_head.x - (width / 2.0 + 5.0);
            let y = screen_head.y + (height * (100 - entity.health) as f32 / 100.0);
            let h = height - (height * (100 - entity.health) as f32 / 100.0);
            painter.rect_stroke(Rect::from_min_max((x - 2.0, y - 2.0).into(), (x , y + h).into()), self.rounding, Stroke::new(3.0, color));

            for (from, to) in BONE_CONNECTIONS.iter() {
                let Some(from) = entity.bones.get(*from) else { continue; };
                let Some(to) = entity.bones.get(*to) else { continue; };

                painter.line_segment([Pos2::new(from.x, from.y), Pos2::new(to.x, to.y)], Stroke::new(2.0, Color32::RED));
            }
        }
    }
}

pub fn world_to_screen(v: Vector3<f32>) -> Option<Vector3<f32>> {
    let g_matrix = LOCAL_PLAYER.lock().unwrap();
    let matrix = g_matrix.view_matrix.data.0;
    drop(g_matrix);

    let g_window = WINDOW_POS.lock().unwrap();
    let (right, bottom) = (g_window.right as f32, g_window.bottom as f32);
    drop(g_window);
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

pub fn update_cs2_coordination() {
    std::thread::spawn(|| {
        unsafe {
            //Counter-Strike 2
            //cs2-cheat – main.rs
            let name: Vec<u16> = OsStr::new("Counter-Strike 2").encode_wide().chain(once(0)).collect();

            let mut h_wnd = FindWindowW(null(), name.as_ptr());
            loop {
                if h_wnd.is_null() {
                    sleep(Duration::from_secs(4));
                    h_wnd = FindWindowW(null(), name.as_ptr());
                    continue;
                }
                let mut rect = WINDOW_POS.lock().unwrap();
                winapi::um::winuser::GetWindowRect(h_wnd, &mut *rect);
                drop(rect);
                sleep(Duration::from_secs(10));
            }
        }
    });
}
