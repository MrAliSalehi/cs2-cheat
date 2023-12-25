use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null;
use std::slice::Iter;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crossbeam_channel::{Receiver, Sender};
use egui::{Align2, Color32, Context, FontId, Order, Painter, Pos2, Rect, Rgba, Rounding, Sense, Stroke, Vec2, vec2, Window};
use egui::epaint::TextShape;
use egui_overlay::{EguiOverlay};
use egui_overlay::egui_render_three_d::ThreeDBackend;
use egui_overlay::egui_window_glfw_passthrough::GlfwBackend;
use winapi::um::winuser::FindWindowW;
use crate::continue_if;
use crate::entity::Entity;
use crate::globals::{BONE_CONNECTIONS, ENTITY_LIST, LOCAL_PLAYER, WINDOW_POS};

use crate::gui::{OverlayTab, Tabs, world_to_screen};
use crate::gui::esp::Esp;
use crate::gui::misc::Misc;
use crate::gui::setting::GeneralSetting;

#[derive(Clone)]
pub struct CsOverlay {
    pub frame: u32,
    pub misc: Misc,
    pub general_settings: GeneralSetting,
    pub esp: Esp,
    pub current_tab: Tabs,
    open: bool,
    pub found_game: bool,
    pub process_name: Vec<u16>,
    pub first_frame: bool,
    pub waiting_icon: String,
    pub abortion_signal: Sender<u8>,
}

impl CsOverlay {
    pub fn new(abortion_signal: Sender<u8>, process_name: Vec<u16>) -> Self {
        Self {
            abortion_signal,
            frame: 0,
            esp: Esp::default(),
            misc: Misc {},
            general_settings: GeneralSetting::default(),
            current_tab: Tabs::Esp,
            open: true,
            found_game: false,
            process_name,
            first_frame: true,
            waiting_icon: String::from(egui_phosphor::regular::CLOCK_COUNTDOWN),
        }
    }
    pub fn game_running(&self, name: *const u16) -> bool {
        let h_wnd = unsafe { FindWindowW(null(), name) };
        !h_wnd.is_null()
    }
    pub fn waiting_ui(&mut self, context: &Context, glfw_backend: &mut GlfwBackend) {
        self.if_closed(glfw_backend);
        Window::new("waiting")
            .open(&mut self.open)
            .resizable(false)
            .default_size(Vec2::new(250.0, 150.0))
            .collapsible(false)
            .show(context, |ui| {
                ui.label(egui::RichText::new(format!("waiting for game process... {}", self.waiting_icon)).size(17.0));
                if context.wants_pointer_input() || context.wants_keyboard_input() {
                    glfw_backend.window.set_mouse_passthrough(false);
                } else {
                    glfw_backend.window.set_mouse_passthrough(true);
                }
                context.request_repaint();
            });
    }

    fn if_closed(&mut self, glfw_backend: &mut GlfwBackend) {
        if !self.open {
            self.abortion_signal.send(1).unwrap();
            self.abortion_signal.send(1).unwrap(); // if it sent only once it will not break the app
            glfw_backend.window.set_should_close(true);
        }
    }
    fn draw_visuals(&self, entities: Iter<Entity>, local_player_team: u8, painter: &Painter) {
        for entity in entities {
            continue_if!(entity.health == 0);
            let Some(screen_pos) = world_to_screen(entity.origin, &self.esp.game_rect) else { continue; };
            let Some(screen_head) = world_to_screen(entity.head, &self.esp.game_rect) else { continue; };

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

            let color = if entity.team_number != local_player_team {
                Color32::from_rgba_premultiplied((255 - entity.health) as u8, (55 + entity.health * 2) as u8, (140 - entity.health) as u8, 255)
            } else {
                if !self.esp.team_box { continue; }
                Color32::WHITE
            };

            //esp border position
            painter.rect_stroke(Rect::from_min_max((x, y).into(), (x + w, y + h).into()), self.esp.rounding, Stroke::new(3.0, color));

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
            painter.rect_stroke(Rect::from_min_max((x - 2.0, y - 2.0).into(), (x, y + h).into()), self.esp.rounding, Stroke::new(3.0, color));

            for (from, to) in BONE_CONNECTIONS.iter() {
                let Some(from) = entity.bones.get(*from) else { continue; };
                let Some(to) = entity.bones.get(*to) else { continue; };

                painter.line_segment([Pos2::new(from.x, from.y), Pos2::new(to.x, to.y)], Stroke::new(2.0, Color32::RED));
            }
        }
    }
}

impl EguiOverlay for CsOverlay {
    fn gui_run(&mut self, egui_context: &Context, _: &mut ThreeDBackend, glfw_backend: &mut GlfwBackend) {
        /* self.if_closed(glfw_backend);

         if self.first_frame {
             let mut fonts = egui::FontDefinitions::default();
             egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
             egui_context.set_fonts(fonts);
             catppuccin_egui::set_theme(egui_context, catppuccin_egui::MOCHA);
             self.first_frame = false;
         }
         if !self.found_game {
             glfw_backend.window.set_pos(0, 0);
             glfw_backend.window.set_size(500, 500);
             self.waiting_ui(egui_context, glfw_backend);
             self.found_game = self.game_running(self.process_name.as_ptr());
             return;
         }*/
        let cs_size = WINDOW_POS.lock().unwrap();
        let game_bound_y = 0;
        let game_bound_x = 0;

        let game_bound_right = cs_size.right;
        let game_bound_bottom = cs_size.bottom;
        self.esp.game_rect = *cs_size;
        drop(cs_size);

        glfw_backend.window.set_pos(game_bound_x, game_bound_y);
        glfw_backend.window.set_size(game_bound_right, game_bound_bottom);

        self.esp.area_pos = Pos2::new(game_bound_x as f32, game_bound_y as f32);
        self.esp.area_size = vec2(game_bound_right as f32, game_bound_bottom as f32);

        sleep(Duration::from_millis(4));

        egui::Area::new("overlay")
            .interactable(false)
            .fixed_pos(self.esp.area_pos)
            .order(Order::Background)
            .show(egui_context, |ui| {
                let (rect, _) = ui.allocate_at_least(self.esp.area_size, Sense { focusable: false, drag: false, click: false });
                let painter = ui.painter();
                if self.general_settings.show_borders {
                    painter.rect_stroke(rect, Rounding::from(3.0), Stroke::new(3.0, Color32::YELLOW));
                }

                let g_entities = ENTITY_LIST.lock().unwrap();
                let entities = g_entities.iter();

                let g_local_player = LOCAL_PLAYER.lock().unwrap();
                let local_player_team = g_local_player.entity.team_number;
                drop(g_local_player);

                if self.esp.enabled {
                    self.draw_visuals(entities, local_player_team, painter);
                }
                drop(g_entities);
            });

        Window::new("cs2 external cheat")
            .resizable(true)
            .vscroll(true)
            .hscroll(true)
            .open(&mut self.open)
            .default_size([250.0, 150.0])
            .show(egui_context, |ui|
                {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.current_tab, Tabs::Esp, "esp");
                        ui.selectable_value(&mut self.current_tab, Tabs::Misc, "Misc");
                        ui.selectable_value(&mut self.current_tab, Tabs::Gsettings, "General Options");
                    });

                    ui.separator();


                    match self.current_tab {
                        Tabs::Esp => self.esp.render_ui(ui),
                        Tabs::Gsettings => self.general_settings.render_ui(ui),
                        Tabs::Misc => self.misc.render_ui(ui),
                        Tabs::Aim => {}
                    }
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