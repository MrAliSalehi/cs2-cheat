use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crossbeam_channel::{Receiver, Sender};
use egui::{Context, Vec2, Window};
use egui::epaint::TextShape;
use egui_overlay::{EguiOverlay};
use egui_overlay::egui_render_three_d::ThreeDBackend;
use egui_overlay::egui_window_glfw_passthrough::GlfwBackend;

use crate::gui::{OverlayTab, Tabs};
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
    pub fn new(abortion_signal: Sender<u8>) -> Self {
        Self {
            abortion_signal,
            frame: 0,
            esp: Esp::default(),
            misc: Misc {},
            general_settings: GeneralSetting::default(),
            current_tab: Tabs::Esp,
            open: true,
            found_game: false,
            process_name: OsStr::new("Counter-Strike 2").encode_wide().chain(once(0)).collect::<Vec<u16>>(),
            first_frame: true,
            waiting_icon: String::from(egui_phosphor::regular::CLOCK_COUNTDOWN),
        }
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
}

impl EguiOverlay for CsOverlay {
    fn gui_run(&mut self, egui_context: &Context, _: &mut ThreeDBackend, glfw_backend: &mut GlfwBackend) {
       self.if_closed(glfw_backend);

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
            self.found_game = self.esp.check_game_coordination(self.process_name.as_ptr());
            let (right, bottom) = if self.found_game { (self.esp.game_rect.right as i32, self.esp.game_rect.bottom as i32) } else {
                sleep(Duration::from_millis(10));
                (500, 500)
            };
            glfw_backend.window.set_size(right, bottom);

            return;
        }
        self.esp.check_game_coordination(self.process_name.as_ptr());
        glfw_backend.window.set_pos(0,0);
        glfw_backend.window.set_size(self.esp.game_rect.right, self.esp.game_rect.bottom);
        sleep(Duration::from_millis(4));

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

                    self.esp.render(ui, egui_context, self.general_settings.show_borders, self.current_tab == Tabs::Esp);
                    match self.current_tab {
                        Tabs::Esp => {},
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