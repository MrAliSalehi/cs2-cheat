use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, TryRecvError};
use egui::Ui;
use egui_overlay::egui_render_three_d::three_d::Zero;
use process_memory::{DataMember, Memory};
use crate::globals::{ENTITY_LIST_PTR, LOCAL_PLAYER, TRIGGER_SETTING};
use crate::gui::OverlayTab;
use crate::models::process_handle::ProcHandle;
use crate::offsets;

#[derive(Clone)]
pub struct Trigger {
    pub team: bool,
    pub enemy: bool,
}

impl OverlayTab for Trigger {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.enemy, "enemy");
        ui.checkbox(&mut self.team, "team");
        /*
        if !self.team && !self.enemy { return; }
        let l = LOCAL_PLAYER.lock().unwrap();
        let e = ENTITY_LIST_PTR.lock().unwrap();
        let local_player_pawn = l.entity.pawn;
        let local_player_team = l.entity.team_number;
        let entity_list = e.clone();
        if entity_list.is_zero() { return; }
        let handle = l.process_handle.0;
        drop(l);
        drop(e);
        if handle.0.is_null() { println!("handle null"); return; }


        //println!("entity_list {}, local pawn: {}", entity_list, local_player_pawn);
        let aimed_entity_id = unsafe {
            DataMember::<i32>::new_offset(handle, vec![local_player_pawn + offsets::C_CSPlayerPawnBase::m_iIDEntIndex])
                .read().unwrap_or(0)
        } ;
        if aimed_entity_id < 0 { return; } // aiming noting

        let entry = unsafe {
            DataMember::<i64>::new_offset(handle, vec![entity_list, 0x8 * (aimed_entity_id as usize >> 9) + 0x10])
                .read().unwrap()
        };
        if entry == 0 { return; } // aiming noting

        let aimed_entity = unsafe {
            DataMember::<usize>::new_offset(handle, vec![entry as usize + (120 * (aimed_entity_id as usize & 0x1FF))])
                .read().unwrap_or(0)
        };
        if aimed_entity == 0 { return; } // aiming noting

        let entity_team = unsafe {
            DataMember::<u8>::new_offset(handle, vec![aimed_entity + offsets::C_BaseEntity::m_iTeamNum])
                .read().unwrap_or(0)
        };
        let is_teammate = entity_team == local_player_team;
        if (is_teammate && !self.team) || (!is_teammate && !self.enemy) { return; }
        if unsafe { winapi::um::winuser::GetAsyncKeyState(0xA4) } < 0 {
            println!("shoot");
            sleep(Duration::from_millis(10));
        }*/
    }
}

impl Trigger {
    pub fn run_thread(abortion_signal: Arc<Receiver<u8>>, handle: ProcHandle) {
        std::thread::spawn(move || {
            let handle = handle.clone().0;
            let mut local_player_pawn = 0;
            let mut local_player_team = 0;
            let mut entity_list = 0;
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
                sleep(Duration::from_millis(10));
                if let Ok(_) = abortion_signal.try_recv() { println!("aborting the trigger"); return; }

                if (unsafe { winapi::um::winuser::GetAsyncKeyState(0xA4) } & 0x8000u16 as i16) == 0 { //todo custom shortcut
                    sleep(Duration::from_millis(1));
                    continue;
                }
                let trigger = TRIGGER_SETTING.lock().unwrap();
                let trigger_team = trigger.team;
                let trigger_enemy = trigger.enemy;
                drop(trigger);
                if !trigger_team && !trigger_enemy { continue; }

                let aimed_entity_id = unsafe {
                    DataMember::<i32>::new_offset(handle, vec![local_player_pawn + offsets::C_CSPlayerPawnBase::m_iIDEntIndex])
                        .read().unwrap_or(-1)
                };
                if aimed_entity_id == -1 { continue; } // aiming noting
                let aimed_entity_id = aimed_entity_id as usize;
                let entry = unsafe {
                    DataMember::<usize>::new_offset(handle, vec![entity_list + 0x8 * (aimed_entity_id >> 9) + 0x10])
                        .read().unwrap()
                };
                //if entry == 0 { continue; } // aiming noting

                let entity = unsafe {
                    DataMember::<usize>::new_offset(handle, vec![entry + 120 * (aimed_entity_id & 0x1FF)])
                        .read().unwrap()
                };
                if entity == 0 { continue; } // aiming noting

                let entity_team = unsafe {
                    DataMember::<i32>::new_offset(handle, vec![entity + offsets::C_BaseEntity::m_iTeamNum])
                        .read().unwrap()
                };
                let is_teammate = entity_team == local_player_team as i32;
                if (is_teammate && !trigger_team) || (!is_teammate && !trigger_enemy) { continue; }
                unsafe{ winapi::um::winuser::mouse_event(0x00000002,0,0,0,0)};
                sleep(Duration::from_nanos(100));
                unsafe{ winapi::um::winuser::mouse_event(0x00000004,0,0,0,0)};
                sleep(Duration::from_nanos(100));
            }
        });
    }
    pub fn new() -> Self {
        Self {
            team: false,
            enemy: true,
        }
    }
}