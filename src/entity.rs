use std::fmt::{Debug, Formatter};

use egui_overlay::egui_render_three_d::three_d::Zero;
use nalgebra::{SMatrix, Vector3};

use process_memory::{DataMember, Memory, ProcessHandle};
use crate::{offsets, read_vector3_from_bytes};

#[derive(Default)]
pub struct Entity {
    //pawn: usize,
    //controller: usize,
    pub name: String,
    pub health: u32,
    pub weapon: String,
    pub origin: Vector3<f32>,
    pub head: Vector3<f32>,
    pub team_number: u8,
    pub team_str: String,
    pub money: i32,
    pub spent_money: i32,
}

#[derive(Debug,Default)]
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
    //pub fn update_entity(&mut self, proc: &Process) {}
    pub fn new(controller: usize, pawn: usize, handle: ProcessHandle) -> eyre::Result<Self> {
        //DataMember::<usize>::new_offset(handle, vec![]);
        let health = unsafe {
            DataMember::<u32>::new_offset(handle, vec![pawn + offsets::m_iHealth]).read().unwrap_or(0)
        };
        if health.is_zero() { return Err(eyre::Report::msg("player is dead")); }
        let m_name = DataMember::<[u8; 16]>::new_offset(handle, vec![controller + offsets::m_iszPlayerName]);
        let name = String::from_utf8(unsafe { m_name.read().unwrap().to_vec() })
            .unwrap_or(String::from("crappy name"));

        let clipping_weapon = unsafe {
            DataMember::<usize>::new_offset(handle, vec![pawn + offsets::m_pClippingWeapon])
                .read().unwrap()
        };
        let data = unsafe {
            DataMember::<usize>::new_offset(handle, vec![clipping_weapon + 0x360])
                .read().unwrap()
        };
        let w_name_ptr = unsafe {
            DataMember::<usize>::new_offset(handle, vec![data + offsets::m_szName])
                .read().unwrap_or(0)
        };
        if w_name_ptr == 0 { return Err(eyre::Report::msg("gun unavailable")); }
        let raw_w_name = unsafe {
            DataMember::<[u8; 32]>::new_offset(handle, vec![w_name_ptr])
                .read().unwrap()
        };
        let weapon_name = Self::fix_weapon_name(String::from_utf8(raw_w_name.to_vec()).unwrap_or(String::from("")));

        /*
        let mut clipping_weapon = 0;
        proc.read_ptr(&mut clipping_weapon, pawn + offsets::m_pClippingWeapon, 8);
        //    println!("x2");
        let mut data = 0;
        proc.read_ptr(&mut data, clipping_weapon + 0x360, 8);
        proc.read_ptr(&mut data, data + offsets::m_szName, 8);
        //println!("x3.1");

        let raw = proc.read_mem::<[u8; 32]>(data).unwrap_or_default();
        //       println!("x3.2");

        let mut weapon_name = String::from_utf8(raw.to_vec()).unwrap_or(String::from(""));
        if weapon_name != "" {
            let a = weapon_name.rfind("weapon_").unwrap_or(0);
            //    println!("x3.3");

            if a != 0 {
                weapon_name.replace_range(a..weapon_name.len(), "");
            }
            if weapon_name.contains("awp") {
                weapon_name = String::from("awp");
            }
        }*/

        let team_number = unsafe {
            DataMember::<u8>::new_offset(handle, vec![pawn + offsets::m_iTeamNum])
                .read().unwrap()
        };

        let team_str = (if team_number == 2 { "terrorist" } else { "ct" }).to_string();

        let raw_origin = unsafe {
            DataMember::<[u8; 12]>::new_offset(handle, vec![pawn + offsets::m_vOldOrigin])
                .read().unwrap()
        };
        let origin = read_vector3_from_bytes(&raw_origin);
        let head = Vector3::new(origin.x, origin.y, origin.z + 75f32); // 75 is the player height

        let money_service = unsafe {
            DataMember::<usize>::new_offset(handle, vec![controller + offsets::m_pInGameMoneyServices])
                .read().unwrap()
        };
        let mut money = -98;
        let mut spent_money = -98;

        if money_service != 0 {
            spent_money = unsafe {
                DataMember::<i32>::new_offset(handle, vec![money_service + offsets::m_iTotalCashSpent])
                    .read().unwrap()
            };
            money = unsafe {
                DataMember::<i32>::new_offset(handle, vec![money_service + offsets::m_iAccount])
                    .read().unwrap()
            };
        }
        /*
        let team_number = proc.read_mem::<u8>(pawn + offsets::m_iTeamNum).unwrap_or(2);

        let team_str = (if team_number == 2 { "terrorist" } else { "ct" }).to_string();

        let raw = proc.read_mem::<[u8; 12]>(pawn + offsets::m_vOldOrigin).unwrap_or_default();
        let origin = read_vector3_from_bytes(&raw);
        let head = Vector3::new(origin.x, origin.y, origin.z + 75f32); // 75 is the player height
        //     println!("x6");

        let mut money_service = 0;
        proc.read_ptr(&mut money_service, controller + offsets::m_pInGameMoneyServices, 8);

        let mut money = -98;
        let mut spent_money = -98;

        if money_service != 0 {
            spent_money = proc.read_mem::<i32>(money_service + offsets::m_iTotalCashSpent).unwrap_or(-99);
            money = proc.read_mem::<i32>(money_service + offsets::m_iAccount).unwrap_or(-99);
        }
        //  println!("x7");
*/
        Ok(
            Self {
                name,
                health,
                weapon: weapon_name,
                origin,
                head,
                team_str,
                money,
                spent_money,
                team_number,
            //    pawn,
               // controller,
            }
        )
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
        f.write_str(&format!("name:{},health:{},team:{},weapon:{},origin:({},{},{}),money:{},spent:{}",
                             self.name, self.health, self.team_str, self.weapon, self.origin.x, self.origin.y, self.origin.z, self.money, self.spent_money))
    }
}