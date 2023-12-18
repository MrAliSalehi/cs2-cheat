#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
pub const dwEntityList: usize = 0x17C18E0;
pub const dwForceAttack: usize = 0x16C2190;
pub const dwViewMatrix: usize = 0x1820100;

pub const m_iHealth: usize = 0x32C;
// int32_t
pub const m_hPlayerPawn: usize = 0x7EC;
// CHandle<C_CSPlayerPawn>
pub const m_iszPlayerName: usize = 0x640; // char[128]
// char[128]
pub const m_iTeamNum: usize = 0x3BF;
// uint8_t
pub const m_iIDEntIndex: usize = 0x1544; // CEntityIndex

pub const m_vOldOrigin: usize = 0x1224; // Vector
// Vector
//pub const origin:usize = 0xCD8;
//pub const m_vecAbsOrigin: usize = 0xC8; // Vector
pub const m_pClippingWeapon: usize = 0x12B0;
// C_CSWeaponBase*
pub const m_szName: usize = 0xC18;
// CUtlString
pub const m_pEntity: usize = 0x10; // CEntityIdentity*

pub const m_designerName: usize = 0x20; // CUtlSymbolLarge

pub const m_pInGameMoneyServices: usize = 0x700; // CCSPlayerController_InGameMoneyServices*

pub const m_iAccount: usize = 0x40; // int32_t

pub const m_iTotalCashSpent: usize = 0x48; // int32_t
pub const m_iCashSpentThisRound: usize = 0x4C; // int32_t