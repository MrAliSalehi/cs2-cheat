use egui::{CollapsingHeader, Color32, Pos2, Rounding, Stroke, Ui, Vec2};
use winapi::shared::windef::RECT;
use crate::gui::{OverlayTab};

#[derive(Clone)]
pub struct Esp {
    pub box_rounding: f32,
    pub health_bar_rounding: f32,

    pub team_box: bool,
    pub area_pos: Pos2,
    pub area_size: Vec2,
    pub game_rect: RECT,
    pub enabled: bool,
    pub bones: bool,

    pub enemy_box_color_by_health: bool,
    pub enemy_health_bar_color_by_health: bool,

    pub team_box_stroke: Stroke,
    pub enemy_box_stroke: Stroke,

    pub enemy_health_bar: bool,
    pub team_health_bar: bool,

    pub team_health_bar_stroke: Stroke,
    pub enemy_health_bar_stroke: Stroke,
    pub show_box: bool,
}


impl OverlayTab for Esp {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.enabled, "Enable");
        if !self.enabled { return; }

        self.features(ui);
        self.roundings(ui);
        self.colors(ui);
        self.thickness(ui);
    }
}

impl Esp {
    fn colors(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("colors")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("team box color:");
                    ui.color_edit_button_srgba(&mut self.team_box_stroke.color);
                });
                ui.horizontal(|ui| {
                    ui.label("team health bar color:");
                    ui.color_edit_button_srgba(&mut self.team_health_bar_stroke.color);
                });

                ui.separator();

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("enemy box color:");
                        ui.checkbox(&mut self.enemy_box_color_by_health, "relative to health");
                    });

                    if !self.enemy_box_color_by_health {
                        ui.horizontal(|ui| {
                            ui.label("pick a color:");
                            ui.color_edit_button_srgba(&mut self.enemy_box_stroke.color);
                        });
                    }
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("enemy health bar color:");
                        ui.checkbox(&mut self.enemy_health_bar_color_by_health, "relative to health");
                    });

                    if !self.enemy_health_bar_color_by_health {
                        ui.horizontal(|ui| {
                            ui.label("pick a color:");
                            ui.color_edit_button_srgba(&mut self.enemy_health_bar_stroke.color);
                        });
                    }
                });
            });
    }

    fn roundings(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("roundings")
            .default_open(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("box rounding: ");
                    ui.add(egui::Slider::new(&mut self.box_rounding, 0.0..=10.0));
                });
                ui.horizontal(|ui| {
                    ui.label("health bar rounding: ");
                    ui.add(egui::Slider::new(&mut self.health_bar_rounding, 0.0..=10.0));
                });
            });
    }

    fn features(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("features")
            .default_open(true)
            .show(ui, |ui| {
                ui.checkbox(&mut self.show_box, "show box");
                if self.show_box {
                    ui.checkbox(&mut self.team_box, "show teammate");
                }
                ui.checkbox(&mut self.bones, "show bones");
                ui.separator();
                ui.checkbox(&mut self.team_health_bar, "show team health bar");
                ui.checkbox(&mut self.enemy_health_bar, "show enemy health bar");
            });
    }
    pub fn thickness(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("thickness")
            .default_open(false)
            .show(ui, |ui| {

                ui.horizontal(|ui| {
                    ui.label("team box thickness: ");
                    ui.add(egui::Slider::new(&mut self.team_box_stroke.width, 0.1..=15.0));
                });

                ui.horizontal(|ui| {
                    ui.label("team health bar thickness: ");
                    ui.add(egui::Slider::new(&mut self.team_health_bar_stroke.width, 0.1..=15.0));
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("enemy box thickness: ");
                    ui.add(egui::Slider::new(&mut self.enemy_box_stroke.width, 0.1..=15.0));
                });
                ui.horizontal(|ui| {
                    ui.label("enemy health bar thickness: ");
                    ui.add(egui::Slider::new(&mut self.enemy_health_bar_stroke.width, 0.1..=15.0));
                });

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
            bones: true,
            box_rounding: 1.0,
            health_bar_rounding: 1.0,
            enemy_box_color_by_health: true,
            enemy_box_stroke: Stroke::new(1.0,Color32::RED),
            team_box_stroke: Stroke::new(1.0,Color32::GREEN),
            enemy_health_bar: true,
            team_health_bar: true,
            enemy_health_bar_stroke: Stroke::new(1.8,Color32::RED),
            team_health_bar_stroke: Stroke::new(1.8,Color32::GREEN),
            enemy_health_bar_color_by_health: true,
        }
    }
}