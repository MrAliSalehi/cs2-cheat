use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crossbeam_channel::Receiver;
use nalgebra::Vector3;
use proc_mem::{Module, Process};

pub type Res = eyre::Result<()>;


pub fn get_client_module(recv_cl: &Arc<Receiver<u8>>, proc: &Process) -> Option<Module> {
    let client = loop {
        if let Ok(_) = recv_cl.try_recv() {
            return None
        }
        if let Ok(module) = proc.module("client.dll") {
            break module;
        }
        println!("waiting for modules");
        sleep(Duration::from_secs(1));
    };
    Some(client)
}

pub fn get_game_process(recv_cl: &Arc<Receiver<u8>>) -> Option<Process> {
    let proc = loop {
        if let Ok(_) = recv_cl.try_recv() {
            return None;
        }
        match Process::with_name("cs2.exe") {
            Ok(proc) => break proc,
            Err(_) => {
                println!("waiting for process");
                sleep(Duration::from_secs(1));
            }
        }
    };
    Some(proc)
}
pub fn read_vector3_from_bytes(bytes: &[u8]) -> Vector3<f32> {
    let floats: &[f32; 3] = bytemuck::from_bytes(bytes);
    Vector3::from_column_slice(floats)
}

#[macro_export]
macro_rules! continue_if {
    ($cond:expr) => {
        if $cond { continue; }
    };
}