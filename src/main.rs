#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg(not(target_pointer_width = "64"))]
compile_error!("compilation is only allowed for 64-bit targets");

use std::{thread::sleep, time::Duration, ffi::OsStr, iter::once, os::windows::ffi::OsStrExt, sync::Arc};
use crossbeam_channel::Receiver;
use egui_overlay::{egui_render_three_d::three_d::Zero, start};
use process_memory::{DataMember, Memory, Pid, TryIntoProcessHandle};
pub use prelude::*;
use crate::{gui::{cs2_overlay::CsOverlay, trigger::Trigger}, globals::{ENTITY_LIST, LOCAL_PLAYER, ENTITY_LIST_PTR}, entity::{Entity}, models::{local_player::LocalPlayer, process_handle::ProcHandle}};

mod offsets;
pub mod prelude;
mod entity;
mod gui;
mod models;
mod globals;

fn main() -> Res {
    let name = OsStr::new("Counter-Strike 2").encode_wide().chain(once(0)).collect::<Vec<u16>>();
    //gui::update_cs2_coordination(name.clone());

    let (app_state_sender, a_s_receiver) = crossbeam_channel::bounded::<u8>(1);
    let app_state_receiver = Arc::new(a_s_receiver);


    std::thread::spawn(|| start(CsOverlay::new(app_state_sender, name)));

    let recv_cl = Arc::clone(&app_state_receiver);
    let Some(proc) = get_game_process(&recv_cl) else { return Ok(()); };

    let Some(client) = get_client_module(&recv_cl, &proc) else { return Ok(()); };

    sleep(Duration::from_secs(5));

    let base = client.base_address();

    let handle = ProcHandle(Pid::from(proc.process_id).try_into_process_handle().unwrap());

    let mut entity_list = unsafe {
        loop {
            let res = DataMember::<usize>::new_offset(handle.0, vec![base + offsets::client_dll::dwEntityList]).read();
            if let Ok(r) = res { break r; }
            sleep(Duration::from_secs(2));
        }
    };
    *ENTITY_LIST_PTR.lock().unwrap() = entity_list;

    let mut list_entry = unsafe {
        loop {
            let res = DataMember::<usize>::new_offset(handle.0, vec![entity_list + 0x10]).read();
            if let Ok(r) = res { break r; }
            sleep(Duration::from_secs(2));
        }
    };

    LocalPlayer::update_view_matrix(base, handle, Arc::clone(&recv_cl));

    loop {
        //todo: if the game ends do something
        if let Ok(_) = recv_cl.try_recv() {
            return Ok(());
        }
        let game_rules = unsafe { DataMember::<usize>::new_offset(handle.0, vec![base + offsets::client_dll::dwGameRules]).read().unwrap_or(0) };
        if game_rules.is_zero() {
            sleep(Duration::from_secs(2));
            continue;
        }
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

    let recv_cl2 = Arc::clone(&recv_cl);
    std::thread::spawn(move || {
        loop {
            if let Ok(_) = recv_cl2.try_recv() { return; }
            let len = get_entities(handle, list_entry, entity_list, false).unwrap();
            if len.is_zero() {
                entity_list = unsafe { DataMember::<usize>::new_offset(handle.0, vec![base + offsets::client_dll::dwEntityList]).read().unwrap() };
                *ENTITY_LIST_PTR.lock().unwrap() = entity_list;

                list_entry = unsafe { DataMember::<usize>::new_offset(handle.0, vec![entity_list + 0x10]).read().unwrap() };
                println!("entity list is empty");
            }
            sleep(Duration::from_secs(5));
        }
    });

    Trigger::run_thread(Arc::clone(&recv_cl), handle);

    let recv_cl3 = Arc::clone(&recv_cl);

    update_entities_blocking(recv_cl3, handle, list_entry, entity_list);

    Ok(())
}

fn update_entities_blocking(recv_cl3: Arc<Receiver<u8>>, handle: ProcHandle, list_entry: usize, entity_list: usize) {
    std::thread::spawn(move || {
        loop {
            if let Ok(_) = recv_cl3.try_recv() { return; }
            let mut rf = ENTITY_LIST.lock().unwrap();
            if rf.len().is_zero() {
                drop(rf);
                sleep(Duration::from_secs(5));
                println!("waiting for the game to begin");
                continue;
            }
            rf.iter_mut().for_each(|f| f.update());
            drop(rf);
            get_entities(handle, list_entry, entity_list, true).unwrap();
            sleep(Duration::from_millis(21));
        }
    }).join().unwrap();
}

fn get_entities(proc_handle: ProcHandle, list_entry: usize, entity_list: usize, local_player_only: bool) -> eyre::Result<usize> {
    let mut entities = vec![];

    let handle = proc_handle.0;
    for i in 0..64 {
        if list_entry == 0 { break; }
        if entity_list == 0 { break; }


        let controller = unsafe {
            DataMember::<usize>::new_offset(handle, vec![list_entry + (i * 0x78)]).read().unwrap_or(0)
        };

        continue_if!(controller == 0);


        let pawn_handle = unsafe {
            DataMember::<usize>::new_offset(handle, vec![controller + offsets::CCSPlayerController::m_hPlayerPawn]).read().unwrap_or(0)
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
            if local_player_only { return Ok(1); }
            continue;
        }

        entities.push(entity);
    }
    let len = entities.len();
    *ENTITY_LIST.lock().unwrap() = entities;

    Ok(len)
}