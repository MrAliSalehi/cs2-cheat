use egui::{CollapsingHeader, Color32, Pos2, Rounding, Stroke, Ui, Vec2};
use winapi::shared::windef::RECT;
use crate::gui::{OverlayTab};

#[derive(Clone)]
pub struct Esp {
    pub area_pos: Pos2,
    pub area_size: Vec2,
    pub game_rect: RECT,

    pub box_rounding: f32,
    pub health_bar_rounding: f32,

    pub team_box: bool,
    pub enabled: bool,
    pub team_bones: bool,
    pub enemy_bones: bool,
    pub show_box: bool,

    pub enemy_box_color_by_health: bool,
    pub enemy_health_bar_color_by_health: bool,

    pub team_box_stroke: Stroke,
    pub enemy_box_stroke: Stroke,

    pub enemy_health_bar: bool,
    pub team_health_bar: bool,

    pub team_health_bar_stroke: Stroke,
    pub enemy_health_bar_stroke: Stroke,

    pub team_bone_stroke: Stroke,
    pub enemy_bone_stroke: Stroke,
}


impl OverlayTab for Esp {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.enabled, "Enable");
        if !self.enabled { return; }

        self.esp_box(ui);
        self.health_bar(ui);
        self.bones(ui);
    }
}

impl Esp {
    fn esp_box(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("box")
            .default_open(false)
            .show(ui, |ui| {
                ui.checkbox(&mut self.show_box, "show box");
                if !self.show_box {
                    return;
                }
                ui.checkbox(&mut self.team_box, "teammate");
                CollapsingHeader::new("options")
                    .default_open(false)
                    .show(ui, |ui| {
                        if self.team_box {
                            ui.horizontal(|ui| {
                                ui.label("team color:");
                                ui.color_edit_button_srgba(&mut self.team_box_stroke.color);
                            });
                        }

                        ui.horizontal(|ui| {
                            ui.label("enemy color:");
                            ui.checkbox(&mut self.enemy_box_color_by_health, "relative to health");
                        });

                        if !self.enemy_box_color_by_health {
                            ui.horizontal(|ui| {
                                ui.label("pick a color:");
                                ui.color_edit_button_srgba(&mut self.enemy_box_stroke.color);
                            });
                        }

                        ui.horizontal(|ui| {
                            ui.label("rounding:");
                            ui.add(egui::Slider::new(&mut self.box_rounding, 0.0..=10.0));
                        });
                    });
            });
    }
    fn health_bar(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("health bar")
            .default_open(false)
            .show(ui, |ui| {
                ui.checkbox(&mut self.team_health_bar, "team");
                if self.team_health_bar {
                    ui.horizontal(|ui| {
                        ui.label("color:");
                        ui.color_edit_button_srgba(&mut self.team_health_bar_stroke.color);
                    });
                    ui.horizontal(|ui| {
                        ui.label("thickness:");
                        ui.add(egui::Slider::new(&mut self.team_health_bar_stroke.width, 0.1..=15.0));
                    });
                }
                ui.separator();
                ui.checkbox(&mut self.enemy_health_bar, "enemy");
                if self.enemy_health_bar {
                    ui.horizontal(|ui| {
                        ui.label("thickness:");
                        ui.add(egui::Slider::new(&mut self.enemy_health_bar_stroke.width, 0.1..=15.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label("color:");
                        ui.checkbox(&mut self.enemy_health_bar_color_by_health, "relative to health");
                    });

                    if !self.enemy_health_bar_color_by_health {
                        ui.horizontal(|ui| {
                            ui.label("pick a color:");
                            ui.color_edit_button_srgba(&mut self.enemy_health_bar_stroke.color);
                        });
                    }
                }
                ui.separator();
                if self.team_health_bar || self.enemy_health_bar {
                    ui.horizontal(|ui| {
                        ui.label("rounding:");
                        ui.add(egui::Slider::new(&mut self.health_bar_rounding, 0.0..=10.0));
                    });
                }
            });
    }
    fn bones(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("bones")
            .default_open(false)
            .show(ui,|ui| {
                ui.checkbox(&mut self.team_bones, "team");
                if self.team_bones {
                    ui.horizontal(|ui| {
                        ui.label("color:");
                        ui.color_edit_button_srgba(&mut self.team_bone_stroke.color);
                    });
                    ui.horizontal(|ui| {
                        ui.label("thickness:");
                        ui.add(egui::Slider::new(&mut self.team_bone_stroke.width, 0.1..=15.0));
                    });
                }
                ui.separator();

                ui.checkbox(&mut self.enemy_bones, "enemy");
                if self.enemy_bones {
                    ui.horizontal(|ui| {
                        ui.label("color:");
                        ui.color_edit_button_srgba(&mut self.enemy_bone_stroke.color);
                    });
                    ui.horizontal(|ui| {
                        ui.label("thickness:");
                        ui.add(egui::Slider::new(&mut self.enemy_bone_stroke.width, 0.1..=15.0));
                    });
                }
            });
    }
}

impl Default for Esp {
    fn default() -> Self {
        Self {
            enabled: true,
            show_box: true,
            team_box: false,
            area_pos: Pos2::default(),
            area_size: Vec2::default(),
            game_rect: RECT::default(),
            team_bones: true,
            enemy_bones: true,
            box_rounding: 1.0,
            health_bar_rounding: 1.0,
            enemy_box_color_by_health: true,
            enemy_box_stroke: Stroke::new(1.0, Color32::RED),
            team_box_stroke: Stroke::new(1.0, Color32::GREEN),
            enemy_health_bar: true,
            team_health_bar: true,
            enemy_health_bar_stroke: Stroke::new(1.8, Color32::RED),
            team_health_bar_stroke: Stroke::new(1.8, Color32::GREEN),
            enemy_health_bar_color_by_health: true,
            enemy_bone_stroke: Stroke::new(1.5, Color32::RED),
            team_bone_stroke: Stroke::new(1.5, Color32::GREEN),
        }
    }
}