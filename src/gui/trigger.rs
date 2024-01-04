use std::sync::{Arc};
use std::thread::sleep;
use std::time::Duration;
use crossbeam_channel::{Receiver};
use egui::Ui;
use egui_overlay::egui_render_three_d::three_d::Zero;
use mouce::common::{MouseButton, ScrollDirection};
use mouce::Mouse;
use process_memory::{DataMember, Memory, ProcessHandle};
use winapi::shared::minwindef::UINT;
use winapi::um::winuser::{INPUT, LPINPUT};
use crate::globals::{ENTITY_LIST_PTR, LOCAL_PLAYER, TRIGGER_SETTING};
use crate::gui::OverlayTab;
use crate::models::process_handle::ProcHandle;
use crate::offsets;

#[derive(Clone)]
pub struct Trigger {
    pub team: bool,
    pub enemy: bool,
    pub zoom: bool,
    pub swap_gun_after_shoot: bool,
    pub delay_ms: u64,
}

impl OverlayTab for Trigger {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.enemy, "enemy");
        ui.checkbox(&mut self.team, "team");

        ui.horizontal(|ui| {
            ui.label("delay (ms):");
            ui.add(egui::Slider::new(&mut self.delay_ms, 0..=350).drag_value_speed(1.0).suffix(" MS").step_by(1.0));
        });
        ui.separator();

        ui.checkbox(&mut self.zoom, "zoom");
        ui.checkbox(&mut self.swap_gun_after_shoot, "swap after sniper shoot");
    }
}

impl Trigger {
    pub fn run_thread(abortion_signal: Arc<Receiver<u8>>, handle: ProcHandle) {
        std::thread::spawn(move || {
            let handle = handle.clone().0;
            let mut local_player_pawn = 0;
            let mut local_player_team = 0;
            let mut entity_list = 0;
            let mouse = Mouse::new();
            loop {
                if local_player_pawn.is_zero() {
                    let l = LOCAL_PLAYER.lock().unwrap();
                    local_player_pawn = l.entity.pawn;
                    local_player_team = l.entity.team_number;
                    drop(l);
                    sleep(Duration::from_secs(1));
                    continue;
                }

                if entity_list.is_zero() {
                    entity_list = *ENTITY_LIST_PTR.lock().unwrap();
                    sleep(Duration::from_secs(1));
                    continue;
                }

                if let Ok(_) = abortion_signal.try_recv() {
                    println!("aborting the trigger");
                    return;
                }

                if (unsafe { winapi::um::winuser::GetAsyncKeyState(0xA4) } & 0x8000u16 as i16) == 0 { //todo custom shortcut
                    sleep(Duration::from_millis(4));
                    continue;
                }
                let trigger = TRIGGER_SETTING.lock().unwrap();
                let trigger_team = trigger.team;
                let trigger_enemy = trigger.enemy;
                let trigger_zoom = trigger.zoom;
                let delay_ms = trigger.delay_ms;
                drop(trigger);

                if !trigger_team && !trigger_enemy { continue; }

                sleep(Duration::from_millis(delay_ms));

                let Some(entity) = Self::find_entity(handle, local_player_pawn, entity_list) else { continue; };

                let entity_team = unsafe {
                    DataMember::<i32>::new_offset(handle, vec![entity + offsets::C_BaseEntity::m_iTeamNum])
                        .read().unwrap_or(-1)
                };
                if entity_team == -1 { continue; }
                let is_teammate = entity_team == local_player_team as i32;
                if (is_teammate && !trigger_team) || (!is_teammate && !trigger_enemy) { continue; }
                let l_l = LOCAL_PLAYER.lock().unwrap();
                let current_gun = l_l.entity.weapon.clone();
                drop(l_l);

                let mut is_snip = false;
                if trigger_zoom && (current_gun.contains("ssg") || current_gun.contains("awp") || current_gun.contains("scar")) {
                    is_snip = true;
                    mouse.click_button(&MouseButton::Right).unwrap();
                    sleep(Duration::from_millis(150));
                }

                unsafe { winapi::um::winuser::mouse_event(0x00000002, 0, 0, 0, 0) };

                /* todo: click doesnt support delay
                  mouse.click_button(&MouseButton::Left).unwrap();
                    sleep(Duration::from_millis(30));*/
                if current_gun.contains("revolver") {
                    sleep(Duration::from_millis(210));
                } else {
                    sleep(Duration::from_nanos(150));
                }
                unsafe { winapi::um::winuser::mouse_event(0x00000004, 0, 0, 0, 0) };
                sleep(Duration::from_millis(20));

                if is_snip {
                  //TODO swap to knife

                    mouse.scroll_wheel(&ScrollDirection::Down).unwrap();

                    sleep(Duration::from_millis(200));

                    sleep(Duration::from_millis(3));
                    mouse.scroll_wheel(&ScrollDirection::Up).unwrap();
                    sleep(Duration::from_millis(1350));

                }
            }
        });
    }

    fn find_entity(handle: ProcessHandle, local_player_pawn: usize, entity_list: usize) -> Option<usize> {
        let aimed_entity_id = unsafe {
            DataMember::<i32>::new_offset(handle, vec![local_player_pawn + offsets::C_CSPlayerPawnBase::m_iIDEntIndex])
                .read().unwrap_or(-1)
        };
        if aimed_entity_id == -1 { return None; } // aiming noting
        let aimed_entity_id = aimed_entity_id as usize;
        let entry = unsafe {
            DataMember::<usize>::new_offset(handle, vec![entity_list + 0x8 * (aimed_entity_id >> 9) + 0x10])
                .read().unwrap_or(0)
        };
        if entry == 0 { return None; } // aiming noting

        let entity = unsafe {
            DataMember::<usize>::new_offset(handle, vec![entry + 120 * (aimed_entity_id & 0x1FF)])
                .read().unwrap_or(0)
        };
        if entry == 0 { return None; }
        Some(entity)
    }
    pub fn new() -> Self {
        Self {
            team: true,
            enemy: true,
            zoom: true,
            swap_gun_after_shoot: true,
            delay_ms: 10,
        }
    }
}