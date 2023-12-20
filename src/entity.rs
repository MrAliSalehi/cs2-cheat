use std::fmt::{Debug, Formatter};

use egui_overlay::egui_render_three_d::three_d::Zero;
use nalgebra::{SMatrix, Vector3};

use process_memory::{DataMember, Memory, ProcessHandle, ProcessHandleExt};
use crate::{offsets, read_vector3_from_bytes};

pub struct Entity {
    pub health: u32,
    pub weapon: String,
    pub origin: Vector3<f32>,
    pub head: Vector3<f32>,
    pub team_number: u8,
    pub team_str: String,
    pub money: i32,
    pub spent_money: i32,

    pub pawn: usize,
    pub controller: usize,

    pub handle: ProcessHandle,
    pub name: String,
    pub weapon_name_ptr: usize,
    pub money_service: usize,
}

unsafe impl Send for Entity {}

unsafe impl Sync for Entity {}

#[derive(Debug, Default)]
pub struct LocalPlayer {
    pub entity: Entity,
    pub view_matrix: SMatrix<f32, 4, 4>,
}

impl LocalPlayer {
    pub fn calc_distance_rounded(&self, second: Vector3<f32>) -> f32 {
        let pov = self.entity.origin;
        let dx = second.x - pov.x;
        let dy = second.y - pov.y;
        let dz = second.z - pov.z;

        ((dx * dx + dy * dy + dz * dz).sqrt()) / 10.0
    }
}

impl Entity {
    fn empty(handle: ProcessHandle, pawn: usize, controller: usize) -> Self {
        Self { pawn, controller, handle, ..Default::default() }
    }
    pub fn update(&mut self) {
        let pawn = self.pawn;
        let handle = self.handle;
        self.health = unsafe {
            DataMember::<u32>::new_offset(handle, vec![pawn + offsets::m_iHealth]).read().unwrap_or(0)
        };
        if self.health == 0 { return; }

        if self.weapon_name_ptr != 0 {
            let raw_w_name = unsafe {
                DataMember::<[u8; 32]>::new_offset(handle, vec![self.weapon_name_ptr])
                    .read().unwrap()
            };
            self.weapon = Self::fix_weapon_name(String::from_utf8(raw_w_name.to_vec()).unwrap_or(String::from("")));
        }

        let raw_origin = unsafe {
            DataMember::<[u8; 12]>::new_offset(handle, vec![pawn + offsets::m_vOldOrigin])
                .read().unwrap()
        };
        self.origin = read_vector3_from_bytes(&raw_origin);
        self.head = Vector3::new(self.origin.x, self.origin.y, self.origin.z + 75f32); // 75 is the player height

        if self.money_service != 0 {
            self.spent_money = unsafe {
                DataMember::<i32>::new_offset(handle, vec![self.money_service + offsets::m_iTotalCashSpent])
                    .read().unwrap()
            };
            self.money = unsafe {
                DataMember::<i32>::new_offset(handle, vec![self.money_service + offsets::m_iAccount])
                    .read().unwrap()
            };
        }
    }
    pub fn new(controller: usize, pawn: usize, handle: ProcessHandle) -> eyre::Result<Self> {
        //DataMember::<usize>::new_offset(handle, vec![]);
        let mut entity = Entity::empty(handle, pawn, controller);
        let health = unsafe {
            DataMember::<u32>::new_offset(handle, vec![pawn + offsets::m_iHealth]).read().unwrap_or(0)
        };

        if health.is_zero() { return Err(eyre::Report::msg("player is dead")); }


        entity.name = String::from_utf8(unsafe {
            DataMember::<[u8; 16]>::new_offset(handle, vec![controller + offsets::m_iszPlayerName])
                .read().unwrap().to_vec()
        }).unwrap_or(String::from("crappy name"));

        let clipping_weapon = unsafe {
            DataMember::<usize>::new_offset(handle, vec![pawn + offsets::m_pClippingWeapon])
                .read().unwrap()
        };
        let data = unsafe {
            DataMember::<usize>::new_offset(handle, vec![clipping_weapon + 0x360])
                .read().unwrap()
        };
        entity.weapon_name_ptr = unsafe {
            DataMember::<usize>::new_offset(handle, vec![data + offsets::m_szName])
                .read().unwrap_or(0)
        };

        if entity.weapon_name_ptr == 0 { return Err(eyre::Report::msg("gun unavailable")); }

        entity.team_number = unsafe {
            DataMember::<u8>::new_offset(handle, vec![pawn + offsets::m_iTeamNum])
                .read().unwrap()
        };

        entity.team_str = (if entity.team_number == 2 { "terrorist" } else { "ct" }).to_string();

        entity.money_service = unsafe {
            DataMember::<usize>::new_offset(handle, vec![controller + offsets::m_pInGameMoneyServices])
                .read().unwrap()
        };

        entity.update();

        Ok(entity)
    }

    fn fix_weapon_name(weapon_name: String) -> String {
        let mut new_name = weapon_name.clone();
        if !new_name.is_empty() {
            let a = new_name.rfind("weapon_").unwrap_or(0);
            //    println!("x3.3");

            if a != 0 {
                new_name.replace_range(a..new_name.len(), "");
            }
            if new_name.contains("awp") {
                new_name = String::from("awp");
            }
        }
        new_name
    }
}


impl Debug for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("name:{},health:{},team ({}):{},weapon:{},origin:({},{},{}),money:{},spent:{}",
                             self.name, self.health, self.team_number, self.team_str, self.weapon, self.origin.x, self.origin.y, self.origin.z, self.money, self.spent_money))
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            name: String::default(),
            head: Vector3::<f32>::default(),
            health: 0,
            team_number: 0,
            weapon: String::default(),
            origin: Vector3::<f32>::default(),
            team_str: String::default(),
            pawn: 0,
            controller: 0,
            money: 0,
            spent_money: 0,
            weapon_name_ptr: 0,
            money_service: 0,
            handle: ProcessHandle::null_type(),
        }
    }
}