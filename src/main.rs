use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use egui_overlay::start;
use lazy_static::lazy_static;
use nalgebra::{SMatrix, Vector3};
use proc_mem::Process;
use process_memory::{DataMember, Memory, Pid, ProcessHandle, TryIntoProcessHandle};
use winapi::shared::windef::RECT;
use winapi::um::winuser::FindWindowW;

mod offsets;
pub mod prelude;
mod entity;
mod gui;

pub use prelude::*;
use crate::entity::{Entity, LocalPlayer};
use crate::gui::CsOverlay;

lazy_static!(
    pub static ref LOCAL_PLAYER: Arc<Mutex<LocalPlayer>> = Arc::new(Mutex::new(LocalPlayer::default()));
    pub static ref ENTITY_LIST: Arc<Mutex<Vec<Entity>>> = Arc::new(Mutex::new(vec![]));
    pub static ref WINDOW_POS: Arc<Mutex<RECT>> = Arc::new(Mutex::new(RECT { left: 0, top: 0, right: 0, bottom: 0 }));
);

#[cfg(not(target_pointer_width = "64"))]
compile_error!("compilation is only allowed for 64-bit targets");

fn main() -> Res {
    std::thread::spawn(|| {
        unsafe {
            //Counter-Strike 2
            //cs2-cheat – main.rs
            let name: Vec<u16> = OsStr::new("Counter-Strike 2").encode_wide().chain(once(0)).collect();

            let mut h_wnd = FindWindowW(null(), name.as_ptr());
            loop {
                if h_wnd.is_null() {
                    sleep(Duration::from_millis(100));
                    println!("cs2 window not found");
                    h_wnd = FindWindowW(null(), name.as_ptr());
                    continue;
                }
                let mut rect = WINDOW_POS.lock().unwrap();
                winapi::um::winuser::GetWindowRect(h_wnd, &mut *rect);
                drop(rect);
                sleep(Duration::from_secs(2));
            }
        }
    });

    let proc = Arc::new(Process::with_name("cs2.exe").unwrap());
    let client = proc.module("client.dll").unwrap();
    let base = client.base_address();
    let pid = proc.process_id;
    let handle = process_memory::Pid::from(pid).try_into_process_handle().unwrap();

    let m_entity_list = DataMember::<usize>::new_offset(handle, vec![base + offsets::dwEntityList]);

    let entity_list = unsafe { m_entity_list.read().unwrap() };

    let m_entry = DataMember::<usize>::new_offset(handle, vec![entity_list + 0x10]);

    let list_entry = unsafe { m_entry.read().unwrap() };

    println!("entityList {} , entry: {}, base: {}", entity_list, list_entry, base);


    std::thread::spawn(move || {
        let handle = Pid::from(pid).try_into_process_handle().unwrap();
        let m_matrix = DataMember
            ::<SMatrix<f32, 4, 4>>::new_offset(handle, vec![base + offsets::dwViewMatrix]);

        loop {
            let matrix = unsafe { m_matrix.read().unwrap() };
            //let matrix = proc_cl.read_mem::<SMatrix<f32, 4, 4>>(base + offsets::dwViewMatrix).unwrap_or_default();
            *LOCAL_PLAYER.lock().unwrap().view_matrix = *matrix;
            //sleep(Duration::from_millis(1));
        }
    });

    std::thread::Builder::new()
        .stack_size(100 * 1024)
        .spawn(move || {
            let handle = Pid::from(pid).try_into_process_handle().unwrap();
            loop {
                entities(handle, list_entry, entity_list).unwrap();
                sleep(Duration::from_millis(20));
            }
        }).unwrap();


    start(CsOverlay { frame: 0, show_borders: true });

    let mut i = 0;
    loop {
        if i >= 100_000 { break; }
        println!("local player: {:?}\n", &LOCAL_PLAYER.lock().unwrap(), );
        let list = ENTITY_LIST.lock().unwrap();
        for e in list.iter() {
            println!("{:?}", e);
        }
        sleep(Duration::from_millis(2000));
        clearscreen::clear().unwrap();
        i += 1;
    }



    Ok(())
}


fn entities(handle: ProcessHandle, list_entry: usize, entity_list: usize) -> Res {
    let mut entities = vec![];


    //println!("entry: {} , entity: {}", list_entry, entity_list);
    for i in 0..64 {
        if list_entry == 0 { break; }
        if entity_list == 0 { break; }
        //println!("a");

        let m_controller = DataMember::<usize>::new_offset(handle, vec![list_entry + (i * 0x78)]);
        /*let mut controller = 0;
        proc.read_ptr(&mut controller, , 8);
         println!("b");*/

        let controller = unsafe { m_controller.read().unwrap() };

        continue_if!(controller == 0);

        let m_pawn_handle = DataMember::<usize>::new_offset(handle,
                                                            vec![controller + offsets::m_hPlayerPawn]);

        let pawn_handle = unsafe { m_pawn_handle.read().unwrap() };
        //proc.read_mem::<i32>(controller + offsets::m_hPlayerPawn).unwrap_or(0);
        //println!("c");
//DataMember::<usize>::new_offset(handle, vec![]);
        continue_if!(pawn_handle == 0);

        let m_entry2 = DataMember::<usize>::new_offset(handle,
                                                       vec![entity_list + (0x8 * ((pawn_handle & 0x7FFF) >> 9) + 0x10)]);

        let entry2 = unsafe { m_entry2.read().unwrap() };
        /*  let mut entry2 = 0;
        proc.read_ptr(&mut entry2, entity_list + (0x8 * ((pawn_handle & 0x7FFF) >> 9) + 0x10) as usize, 8);
        //continue_if!(entry2.is_zero());
        println!("d");

        let mut new_pawn = 0;
        proc.read_ptr(&mut new_pawn, entry2 + (0x78 * (pawn_handle & 0x1FF)) as usize, 8);
        continue_if!(new_pawn.is_zero());*/

        let m_new_pawn = DataMember::<usize>::new_offset(handle,
                                                         vec![entry2 + (0x78 * (pawn_handle & 0x1FF)) ]);

        let new_pawn = unsafe { m_new_pawn.read().unwrap() };

        let Ok(entity) = Entity::new(controller, new_pawn, handle) else { continue; };
        //  println!("e");

        if i == 1 { //first one is the local player
            *LOCAL_PLAYER.lock().unwrap() = LocalPlayer { entity, ..Default::default() };
            continue;
        }
        //   println!("f");

        entities.push(entity);
    }
    // println!("g");

    *ENTITY_LIST.lock().unwrap() = entities;

    Ok(())
}

fn read_vector3_from_bytes(bytes: &[u8]) -> Vector3<f32> {
    let floats: &[f32; 3] = bytemuck::from_bytes(bytes);
    Vector3::from_column_slice(floats)
}
