use std::collections::HashMap;
use std::sync::Arc;
use egui::{CollapsingHeader, Color32, Pos2, Stroke, Ui, Vec2, Widget};
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

    pub enemy_name: bool,
    pub team_name: bool,
    pub team_name_color: Color32,
    pub team_name_size: f32,
    pub enemy_name_color: Color32,
    pub enemy_name_size: f32,
    pub enemy_name_placeholder: String,
    pub team_name_placeholder: String,

    pub team_hp_text: bool,
    pub enemy_hp_text: bool,
    pub team_hp_text_color: Color32,
    pub enemy_hp_text_color: Color32,
    pub team_hp_text_size: f32,
    pub enemy_hp_text_size: f32,

    pub team_distance: bool,
    pub enemy_distance: bool,
    pub team_distance_color: Color32,
    pub enemy_distance_color: Color32,
    pub team_distance_size: f32,
    pub enemy_distance_size: f32,
    //pub name_maps: HashMap<(Template, EntityName), Arc<String>>,
}


//type Template = Arc<String>;
//type EntityName = Arc<String>;

impl OverlayTab for Esp {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.enabled, "Enable");
        if !self.enabled { return; }

        self.esp_box(ui);
        self.health_bar(ui);
        self.bones(ui);
        self.name(ui);
        self.hp_text(ui);
        self.distance(ui);
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
            .show(ui, |ui| {
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
    fn name(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("name")
            .default_open(false)
            .show(ui, |ui| {
                ui.checkbox(&mut self.enemy_name, "enemy name");
                if self.enemy_name {
                    Self::render_name_placeholder(ui, 1);
                    ui.text_edit_singleline(&mut self.enemy_name_placeholder);

                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("color:");
                            ui.color_edit_button_srgba(&mut self.enemy_name_color);
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("size:");
                        ui.add(egui::Slider::new(&mut self.enemy_name_size, 0.5..=25.0));
                    });
                }
                ui.separator();

                ui.checkbox(&mut self.team_name, "team name");

                if self.team_name {
                    Self::render_name_placeholder(ui, 2);
                    ui.text_edit_singleline(&mut self.team_name_placeholder);

                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("color:");
                            ui.color_edit_button_srgba(&mut self.team_name_color);
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("size:");
                        ui.add(egui::Slider::new(&mut self.team_name_size, 0.5..=25.0));
                    });
                }
            });
    }
    fn hp_text(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("HP text")
            .default_open(false)
            .show(ui, |ui| {
                ui.checkbox(&mut self.enemy_hp_text, "enemy hp text");
                if self.enemy_hp_text {
                    ui.horizontal(|ui| {
                        ui.label("color:");
                        ui.color_edit_button_srgba(&mut self.enemy_hp_text_color);
                    });
                    ui.horizontal(|ui| {
                        ui.label("size:");
                        ui.add(egui::Slider::new(&mut self.enemy_hp_text_size, 0.5..=25.0));
                    });
                }
                ui.separator();
                ui.checkbox(&mut self.team_hp_text, "team hp text");
                if self.team_hp_text {
                    ui.horizontal(|ui| {
                        ui.label("color:");
                        ui.color_edit_button_srgba(&mut self.team_hp_text_color);
                    });
                    ui.horizontal(|ui| {
                        ui.label("size:");
                        ui.add(egui::Slider::new(&mut self.team_hp_text_size, 0.5..=25.0));
                    });
                }
            });
    }
    fn distance(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("distance (Experimental)")
            .default_open(false)
            .show(ui, |ui| {
                ui.horizontal(|ui|{
                    ui.separator();
                    ui.label(egui::RichText::new("this feature is experimental and may not work correctly or show false information!")
                        .color(Color32::DARK_GRAY)
                        .size(11.0))
                });

                ui.checkbox(&mut self.enemy_distance, "enemy distance");
                if self.enemy_distance {
                    ui.horizontal(|ui| {
                        ui.label("color:");
                        ui.color_edit_button_srgba(&mut self.enemy_distance_color);
                    });
                    ui.horizontal(|ui| {
                        ui.label("size:");
                        ui.add(egui::Slider::new(&mut self.enemy_distance_size, 0.5..=25.0));
                    });
                }
                ui.separator();
                ui.checkbox(&mut self.team_distance, "team distance");
                if self.team_distance {
                    ui.horizontal(|ui| {
                        ui.label("color:");
                        ui.color_edit_button_srgba(&mut self.team_distance_color);
                    });
                    ui.horizontal(|ui| {
                        ui.label("size:");
                        ui.add(egui::Slider::new(&mut self.team_distance_size, 0.5..=25.0));
                    });
                }
            });
    }
    fn render_name_placeholder(ui: &mut Ui, id: u8) {
        ui.horizontal(|ui| {
            ui.label("placeholder:");
            CollapsingHeader::new("?")
                .default_open(false)
                .id_source(id)
                .show(ui, |ui|
                    ui.label("using this feature you can customize the rendering process of name,\
                     simply write anything you want and put a / (slash) as a placeholder for the player name"),
                );
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
            team_name: true,
            enemy_name: true,
            enemy_name_color: Color32::RED,
            team_name_color: Color32::YELLOW,
            team_name_size: 12.0,
            enemy_name_size: 12.0,
            //name_maps: HashMap::default(),
            enemy_name_placeholder: String::from("(/)"),
            team_name_placeholder: String::from("(/)"),

            enemy_hp_text: false,
            team_hp_text: false,
            enemy_hp_text_size: 10.0,
            team_hp_text_size: 10.0,
            team_hp_text_color: Color32::RED,
            enemy_hp_text_color: Color32::DARK_BLUE,

            enemy_distance: false,
            team_distance: false,
            enemy_distance_size: 10.0,
            team_distance_size: 10.0,
            enemy_distance_color: Color32::RED,
            team_distance_color: Color32::DARK_BLUE,
        }
    }
}