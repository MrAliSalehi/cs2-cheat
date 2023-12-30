use std::thread::sleep;
use std::time::Duration;
use egui::Ui;
use egui_overlay::egui_render_three_d::three_d::Zero;
use process_memory::{DataMember, Memory, ProcessHandleExt};
use crate::globals::{ENTITY_LIST_PTR, LOCAL_PLAYER};
use crate::offsets;

#[derive(Clone)]
pub struct Trigger {
    pub team: bool,
    pub enemy: bool,

}

impl Trigger {
    pub fn render_ui(&mut self, ui: &mut Ui, show_ui: bool) {
        if show_ui {
            ui.checkbox(&mut self.enemy, "enemy");
            ui.checkbox(&mut self.team, "team");
        }
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
        }
    }
}

impl Default for Trigger {
    fn default() -> Self {
        Self {
            team: false,
            enemy: true,
        }
    }
}