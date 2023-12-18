use std::thread::sleep;
use std::time::Duration;
use egui::{Color32, Order, Pos2, Rect, Rounding, Sense, Stroke, vec2, Window};
use egui_overlay::{egui_window_glfw_passthrough, EguiOverlay};
use egui_overlay::egui_render_three_d::ThreeDBackend;

use nalgebra::Vector3;
use crate::{ENTITY_LIST, LOCAL_PLAYER, WINDOW_POS};

pub struct CsOverlay {
    pub frame: u32,
    pub show_borders: bool,
}

impl EguiOverlay for CsOverlay {
    fn gui_run(&mut self, egui_context: &egui::Context, _: &mut ThreeDBackend, glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend) {
        sleep(Duration::from_millis(10));
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

                let (rounding, stroke) = (Rounding::from(3.0), Stroke::new(3.0, Color32::YELLOW));
                let g_entities = ENTITY_LIST.lock().unwrap();
                let entities = g_entities.iter();

                for entity in entities {
                    let Some(screen_pos) = world_to_screen(entity.origin) else { continue; };
                    let Some(screen_head) = world_to_screen(entity.head) else { continue; };

                    let height = screen_pos.y - screen_head.y;
                    let width = height / 2.4f32;

                    let g_localp = LOCAL_PLAYER.lock().unwrap();
                    let _distance = g_localp.calc_distance_rounded(entity.origin);
                    drop(g_localp);

                    //draw visuals
                    let x = screen_head.x - width / 2.0;
                    let y = screen_head.y;
                    let w = width;
                    let h = height;

                    let rect = Rect::from_min_max((x, y).into(), (x + w, y + h).into());

                    painter.rect_stroke(rect, rounding, stroke);
                }

                drop(g_entities);

                //painter.rect_stroke(Rect::from_two_pos(Pos2::new(1.0, 1.0), Pos2::new(100.0, 100.0)), );
            });
        Window::new("egui panel")
            .resizable(true)
            .vscroll(true)
            .hscroll(true)
            .default_size([250.0, 150.0])
            .show(egui_context, |ui|
                {
                    //glfw_backend.window.set_decorated(false);

                    ui.checkbox(&mut self.show_borders, "show border");

                    ui.label(format!(
                        "pixels_per_virtual_unit: {}",
                        glfw_backend.physical_pixels_per_virtual_unit
                    ));
                    ui.label(format!("window scale: {}", glfw_backend.scale));
                    ui.label(format!("cursor pos x: {}", glfw_backend.cursor_pos[0]));
                    ui.label(format!("cursor pos y: {}", glfw_backend.cursor_pos[1]));

                    ui.label(format!(
                        "passthrough: {}",
                        glfw_backend.window.is_mouse_passthrough()
                    ));
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

fn world_to_screen(v: Vector3<f32>) -> Option<Vector3<f32>> {
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


