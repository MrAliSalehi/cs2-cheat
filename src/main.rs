#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread::sleep, time::Duration};
use egui::{Rounding};
use egui_overlay::{egui_render_three_d::three_d::Zero, start};
use nalgebra::{Vector3};
use proc_mem::{Process, };
use process_memory::{DataMember, Memory, Pid, TryIntoProcessHandle};

mod offsets;
pub mod prelude;
mod entity;
mod gui;
mod models;
mod globals;

pub use prelude::*;
use crate::entity::{Entity};
use crate::globals::{ENTITY_LIST, LOCAL_PLAYER};
use crate::gui::CsOverlay;
use crate::models::local_player::LocalPlayer;
use crate::models::process_handle::ProcHandle;


#[cfg(not(target_pointer_width = "64"))]
compile_error!("compilation is only allowed for 64-bit targets");



fn main() -> Res {
    gui::update_cs2_coordination();

    std::thread::spawn(|| start(CsOverlay {
        frame: 0,
        show_borders: false,
        rounding: Rounding::from(2.0),
        team_box: false,
    }));

    let proc = loop {
        let proc = Process::with_name("cs2.exe");
        match proc {
            Ok(proc) => break proc,
            Err(_) => {
                println!("waiting for process");
                sleep(Duration::from_secs(3));
            }
        }
    };

    let client = proc.module("client.dll").unwrap();
    let base = client.base_address();

    let handle = ProcHandle(Pid::from(proc.process_id).try_into_process_handle().unwrap());

    let mut entity_list = unsafe { DataMember::<usize>::new_offset(handle.0, vec![base + offsets::dwEntityList]).read().unwrap() };

    let mut list_entry = unsafe { DataMember::<usize>::new_offset(handle.0, vec![entity_list + 0x10]).read().unwrap() };

    LocalPlayer::update_view_matrix(base, handle);


    let handle = handle;

    //get_entities(handle, list_entry, entity_list).unwrap();

    //todo: do something when the game ends
    loop {
        let game_rules = unsafe { DataMember::<usize>::new_offset(handle.0, vec![base + offsets::dwGameRules]).read().unwrap() };
        //let m_wm = DataMember::<bool>::new_offset(handle.0, vec![game_rules + offsets::C_CSGameRules::m_bWarmupPeriod]);
        let m_st = DataMember::<bool>::new_offset(handle.0, vec![game_rules + offsets::C_CSGameRules::m_bHasMatchStarted]);
        //let is_warmup = unsafe { m_wm.read().unwrap_or(false) };
        let is_started = unsafe { m_st.read().unwrap_or(false) };
        println!("state: st: {}", is_started);
        if is_started {
            break;
        }
        sleep(Duration::from_secs(4));
    }

    std::thread::spawn(move || {
        loop {
            let len = get_entities(handle, list_entry, entity_list).unwrap();
            if len.is_zero() {
                entity_list = unsafe { DataMember::<usize>::new_offset(handle.0, vec![base + offsets::dwEntityList]).read().unwrap() };

                list_entry = unsafe { DataMember::<usize>::new_offset(handle.0, vec![entity_list + 0x10]).read().unwrap() };
                println!("entity list is empty");
            }
            sleep(Duration::from_secs(7));
        }
    });

    std::thread::spawn(move || {
        loop {
            let mut rf = ENTITY_LIST.lock().unwrap();
            if rf.len().is_zero() {
                drop(rf);
                sleep(Duration::from_secs(5));
                println!("waiting for the game to begin");
                continue;
            }
            rf.iter_mut().for_each(|f| f.update());
            drop(rf);
            sleep(Duration::from_millis(21));
        }
    }).join().unwrap();


    Ok(())
}


fn get_entities(handle: ProcHandle, list_entry: usize, entity_list: usize) -> eyre::Result<usize> {
    let mut entities = vec![];

    let handle = handle.0;
    for i in 0..64 {
        if list_entry == 0 { break; }
        if entity_list == 0 { break; }


        let controller = unsafe {
            DataMember::<usize>::new_offset(handle, vec![list_entry + (i * 0x78)]).read().unwrap_or(0)
        };

        continue_if!(controller == 0);


        let pawn_handle = unsafe {
            DataMember::<usize>::new_offset(handle, vec![controller + offsets::m_hPlayerPawn]).read().unwrap_or(0)
        };

        continue_if!(pawn_handle == 0);


        let entry2 = unsafe {
            DataMember::<usize>::new_offset(handle,
                                            vec![entity_list + (0x8 * ((pawn_handle & 0x7FFF) >> 9) + 0x10)])
                .read().unwrap_or(0)
        };
        // continue_if!(entry2 == 0);

        let new_pawn = unsafe {
            DataMember::<usize>::new_offset(handle, vec![entry2 + (0x78 * (pawn_handle & 0x1FF))])
                .read().unwrap_or(0)
        };

        //continue_if!(new_pawn == 0);

        let Ok(entity) = Entity::new(controller, new_pawn, handle) else { continue; };

        if i == 1 { //first one is the local player
            *LOCAL_PLAYER.lock().unwrap() = LocalPlayer { entity, ..Default::default() };
            continue;
        }

        entities.push(entity);
    }
    let len = (&entities).len();
    *ENTITY_LIST.lock().unwrap() = entities;

    Ok(len)
}

fn read_vector3_from_bytes(bytes: &[u8]) -> Vector3<f32> {
    let floats: &[f32; 3] = bytemuck::from_bytes(bytes);
    Vector3::from_column_slice(floats)
}

