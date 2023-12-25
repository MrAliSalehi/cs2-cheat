use std::ptr::null;
use std::slice::Iter;
use egui::{Align2, Color32, FontId, Order, Painter, Pos2, Rect, Rgba, Rounding, Sense, Stroke, Ui, vec2, Vec2};
use winapi::shared::windef::RECT;
use winapi::um::winuser::FindWindowW;
use crate::continue_if;
use crate::entity::Entity;
use crate::globals::{BONE_CONNECTIONS, ENTITY_LIST, LOCAL_PLAYER};
use crate::gui::{world_to_screen};

#[derive(Clone)]
pub struct Esp {
    pub rounding: Rounding,
    pub team_box: bool,
    pub area_pos: Pos2,
    pub area_size: Vec2,
    pub game_rect: RECT,
    pub enabled: bool,
}

impl Esp {
    pub fn check_game_coordination(&mut self, process_name: *const u16) -> bool {
        let mut cs_size = RECT::default();

        let h_wnd = unsafe { FindWindowW(null(), process_name) };
        if h_wnd.is_null() {
            return false;
        }
        if unsafe { winapi::um::winuser::GetWindowRect(h_wnd, &mut cs_size) } == 0 { //function failed
            return false;
        }

        let game_bound_y = cs_size.top;
        let game_bound_x = cs_size.left;

        let game_bound_right = cs_size.right;
        let game_bound_bottom = cs_size.bottom;

        self.area_pos = Pos2::new(game_bound_x as f32, game_bound_y as f32);
        self.area_size = vec2(game_bound_right as f32, game_bound_bottom as f32);
        self.game_rect = cs_size;
        true
    }
    pub fn render(&mut self, ui: &mut Ui, egui_context: &egui::Context, show_borders: bool, render_ui: bool) {
        if self.enabled {
            egui::Area::new("overlay")
                .interactable(false)
                .fixed_pos(self.area_pos)
                .order(Order::Background)
                .show(egui_context, |ui| {
                    let (rect, _) = ui.allocate_at_least(self.area_size, Sense { focusable: false, drag: false, click: false });
                    let painter = ui.painter();
                    if show_borders {
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
        }
        if render_ui {
            ui.label("esp");

            ui.checkbox(&mut self.enabled, "Enable");
            ui.checkbox(&mut self.team_box, "show teammate");
        }

    }
    fn draw_visuals(&self, entities: Iter<Entity>, local_player_team: u8, painter: &Painter) {
        for entity in entities {
            continue_if!(entity.health == 0);
            let Some(screen_pos) = world_to_screen(entity.origin, &self.game_rect) else { continue; };
            let Some(screen_head) = world_to_screen(entity.head, &self.game_rect) else { continue; };

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
            painter.rect_stroke(Rect::from_min_max((x - 2.0, y - 2.0).into(), (x, y + h).into()), self.rounding, Stroke::new(3.0, color));

            for (from, to) in BONE_CONNECTIONS.iter() {
                let Some(from) = entity.bones.get(*from) else { continue; };
                let Some(to) = entity.bones.get(*to) else { continue; };

                painter.line_segment([Pos2::new(from.x, from.y), Pos2::new(to.x, to.y)], Stroke::new(2.0, Color32::RED));
            }
        }
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
            enabled: false,
        }
    }
}