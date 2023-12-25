use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crossbeam_channel::Receiver;
use nalgebra::{SMatrix, Vector3};
use process_memory::{DataMember, Memory};
use crate::entity::Entity;
use crate::models::process_handle::ProcHandle;
use crate::{LOCAL_PLAYER, offsets};

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
    pub fn update_view_matrix(base: usize, handle: ProcHandle,abort_sig: Arc<Receiver<u8>>) {
        std::thread::spawn(move || {
            let handle = handle;
            let m_matrix = DataMember
                ::<SMatrix<f32, 4, 4>>::new_offset(handle.0, vec![base + offsets::client_dll::dwViewMatrix]);
            loop {
                if let Ok(_) = abort_sig.try_recv() {
                    return;
                }
                let matrix = unsafe { m_matrix.read().unwrap() };
                *LOCAL_PLAYER.lock().unwrap().view_matrix = *matrix;
                sleep(Duration::from_nanos(600));
            }
        });
    }
}