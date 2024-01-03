use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use winapi::shared::windef::RECT;
use crate::entity::Entity;
use crate::gui::trigger::Trigger;
use crate::models::local_player::LocalPlayer;
lazy_static!(
    pub static ref BONE_MAP: HashMap<&'static str, usize> = HashMap::from([("head", 6),("neck_0", 5),("spine_1", 4),("spine_2", 2),("pelvis", 0),("arm_upper_L", 8),("arm_lower_L", 9),("hand_L", 10),("arm_upper_R", 13),("arm_lower_R", 14),("hand_R", 15),("leg_upper_L", 22),("leg_lower_L", 23),("ankle_L", 24),("leg_upper_R", 25),("leg_lower_R", 26),("ankle_R", 27)]);
    pub static ref BONE_CONNECTIONS: Vec<(&'static str,&'static str)> = Vec::from([("neck_0", "spine_1"),("spine_1", "spine_2"),("spine_2", "pelvis"),("spine_1", "arm_upper_L"),("arm_upper_L", "arm_lower_L"),("arm_lower_L", "hand_L"),("spine_1", "arm_upper_R"),("arm_upper_R", "arm_lower_R"),("arm_lower_R", "hand_R"),("pelvis", "leg_upper_L"),("leg_upper_L", "leg_lower_L"),("leg_lower_L", "ankle_L"),("pelvis", "leg_upper_R"),("leg_upper_R", "leg_lower_R"),("leg_lower_R", "ankle_R"),]);
    pub static ref WEAPON_MAP: Mutex<HashMap<[u8; 32],Arc<String>>> = Mutex::new(HashMap::default());
    pub static ref LOCAL_PLAYER: Arc<Mutex<LocalPlayer>> = Arc::new(Mutex::new(LocalPlayer::default()));
    pub static ref ENTITY_LIST: Arc<Mutex<Vec<Entity>>> = Arc::new(Mutex::new(vec![]));
    pub static ref WINDOW_POS: Arc<Mutex<RECT>> = Arc::new(Mutex::new(RECT { left: 700, top: 700, right: 700, bottom: 700 }));
    pub static ref ENTITY_LIST_PTR: Arc<Mutex<usize>> =Arc::new(Mutex::new(0));
    pub static ref TRIGGER_SETTING: Arc<Mutex<Trigger>> =Arc::new(Mutex::new(Trigger::new()));
   
);