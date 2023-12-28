use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc};
use egui::Color32;

use egui_overlay::egui_render_three_d::three_d::Zero;
use nalgebra::{Vector3};

use process_memory::{DataMember, Memory, ProcessHandle, ProcessHandleExt};
use crate::{gui, offsets, read_vector3_from_bytes};
use crate::globals::{BONE_MAP, WEAPON_MAP, WINDOW_POS};


pub struct Entity {
    pub health: u32,
    pub weapon: Arc<String>,
    pub origin: Vector3<f32>,
    pub head: Vector3<f32>,
    pub team_number: u8,
    pub team_str: String,
    pub money: i32,
    pub spent_money: i32,

    pub pawn: usize,
    pub controller: usize,

    pub handle: ProcessHandle,
    pub name: Arc<String>,
    pub weapon_name_ptr: usize,
    pub money_service: usize,

    pub bones: HashMap<String, Vector3<f32>>,
    pub bone_arr_addr: usize,
}

impl Entity {
    fn empty(handle: ProcessHandle, pawn: usize, controller: usize) -> Self {
        Self { pawn, controller, handle, ..Default::default() }
    }
    pub fn update(&mut self) {
        let pawn = self.pawn;
        let handle = self.handle;
        self.health = unsafe {
            DataMember::<u32>::new_offset(handle, vec![pawn + offsets::C_BaseEntity::m_iHealth]).read().unwrap_or(0)
        };
        if self.health == 0 { return; }

        if self.weapon_name_ptr != 0 {
            let raw_w_name = unsafe {
                DataMember::<[u8; 32]>::new_offset(handle, vec![self.weapon_name_ptr])
                    .read().unwrap()
            };
            let mut map = WEAPON_MAP.lock().unwrap();
            if let Some(weapon) = map.get(&raw_w_name) {
                self.weapon = Arc::clone(weapon);
            } else {
                self.weapon = Arc::new(Self::fix_weapon_name(String::from_utf8(raw_w_name.to_vec()).unwrap_or(String::from(""))));
                map.insert(raw_w_name, Arc::clone(&self.weapon));
            }
            drop(map)
        }

        let raw_origin = unsafe {
            DataMember::<[u8; 12]>::new_offset(handle, vec![pawn + offsets::C_BasePlayerPawn::m_vOldOrigin])
                .read().unwrap()
        };
        self.origin = read_vector3_from_bytes(&raw_origin);
        self.head = Vector3::new(self.origin.x, self.origin.y, self.origin.z + 75f32); // 75 is the player height

        if self.money_service != 0 {
            self.spent_money = unsafe {
                DataMember::<i32>::new_offset(handle, vec![self.money_service + offsets::CCSPlayerController_InGameMoneyServices::m_iTotalCashSpent])
                    .read().unwrap()
            };
            self.money = unsafe {
                DataMember::<i32>::new_offset(handle, vec![self.money_service + offsets::CCSPlayerController_InGameMoneyServices::m_iAccount])
                    .read().unwrap()
            };
        }
        if self.bone_arr_addr != 0 {
            let g = WINDOW_POS.lock().unwrap();
            let game_bounds = *g;
            drop(g);
            for (bone_name, bone_index) in BONE_MAP.iter() {
                let bone_addr = self.bone_arr_addr + bone_index * 32;
                let position = unsafe {
                    DataMember::<Vector3<f32>>::new_offset(handle, vec![bone_addr]).read().unwrap_or_default()
                };
                self.bones.insert(bone_name.parse().unwrap(), gui::world_to_screen(position, &game_bounds).unwrap_or_default());
            }
        }
    }
    pub fn new(controller: usize, pawn: usize, handle: ProcessHandle) -> eyre::Result<Self> {
        //DataMember::<usize>::new_offset(handle, vec![]);
        let mut entity = Entity::empty(handle, pawn, controller);
        let health = unsafe {
            DataMember::<u32>::new_offset(handle, vec![pawn + offsets::C_BaseEntity::m_iHealth]).read().unwrap_or(0)
        };

        if health.is_zero() { return Err(eyre::Report::msg("player is dead")); }


        entity.name = Arc::new(String::from_utf8(unsafe {
            DataMember::<[u8; 16]>::new_offset(handle, vec![controller + offsets::CBasePlayerController::m_iszPlayerName])
                .read().unwrap().to_vec()
        }).unwrap_or(String::from("crappy name")));

        let clipping_weapon = unsafe {
            DataMember::<usize>::new_offset(handle, vec![pawn + offsets::C_CSPlayerPawnBase::m_pClippingWeapon])
                .read().unwrap_or(0)
        };
        if clipping_weapon.is_zero() { return Err(eyre::Report::msg("player is dead")); }

        let data = unsafe {
            DataMember::<usize>::new_offset(handle, vec![clipping_weapon + 0x360])
                .read().unwrap()
        };

        entity.weapon_name_ptr = unsafe {
            DataMember::<usize>::new_offset(handle, vec![data + offsets::CCSWeaponBaseVData::m_szName])
                .read().unwrap_or(0)
        };

        if entity.weapon_name_ptr == 0 { return Err(eyre::Report::msg("gun unavailable")); }

        entity.team_number = unsafe {
            DataMember::<u8>::new_offset(handle, vec![pawn + offsets::C_BaseEntity::m_iTeamNum])
                .read().unwrap()
        };

        entity.team_str = (if entity.team_number == 2 { "terrorist" } else { "ct" }).to_string();

        entity.money_service = unsafe {
            DataMember::<usize>::new_offset(handle, vec![controller + offsets::CCSPlayerController::m_pInGameMoneyServices])
                .read().unwrap()
        };


        let scene_node = unsafe {
            DataMember::<usize>::new_offset(handle, vec![pawn + offsets::C_BaseEntity::m_pGameSceneNode])
                .read().unwrap()
        };
        entity.bone_arr_addr = unsafe {
            DataMember::<usize>::new_offset(handle, vec![scene_node + offsets::CSkeletonInstance::m_modelState + 0x80])
                .read().unwrap()
        };

        entity.update();

        Ok(entity)
    }

    fn fix_weapon_name(mut weapon_name: String) -> String {
        if !weapon_name.is_empty() {
            let a = weapon_name.rfind("weapon_").unwrap_or(0);
            if a != 0 {
                weapon_name.replace_range(a..weapon_name.len(), "");
            }
            if weapon_name.contains("awp") {
                weapon_name = String::from("awp");
            }
        }
        weapon_name.replace("weapon_","")
    }
    pub fn calculate_color(&self) -> Color32 {
        Color32::from_rgba_premultiplied((255 - self.health) as u8, (55 + self.health * 2) as u8, (140 - self.health) as u8, 255)
    }
}


impl Debug for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("name:{},health:{},team ({}):{},weapon:{},origin:({},{},{}),money:{},spent:{}, bone: {:?}",
                             self.name, self.health, self.team_number, self.team_str, self.weapon,
                             self.origin.x, self.origin.y, self.origin.z,
                             self.money, self.spent_money, self.bones
        ))
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            name: Arc::new(String::default()),
            head: Vector3::<f32>::default(),
            health: 0,
            team_number: 0,
            weapon: Arc::new(String::default()),
            origin: Vector3::<f32>::default(),
            team_str: String::default(),
            pawn: 0,
            controller: 0,
            money: 0,
            spent_money: 0,
            weapon_name_ptr: 0,
            money_service: 0,
            handle: ProcessHandle::null_type(),
            bones: HashMap::default(),
            bone_arr_addr: 0,
        }
    }
}

unsafe impl Send for Entity {}

unsafe impl Sync for Entity {}
