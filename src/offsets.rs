#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]
pub const dwEntityList: usize = 0x17C1960;
pub const dwForceAttack: usize = 0x16C2190;
pub const dwViewMatrix: usize = 0x1820160;

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

pub const dwGameRules: usize = 0x181E058;

pub mod C_CSGameRules { // C_TeamplayRules
pub const __m_pChainEntity: usize = 0x8; // CNetworkVarChainer
pub const m_bFreezePeriod: usize = 0x30; // bool
pub const m_bWarmupPeriod: usize = 0x31; // bool
pub const m_fWarmupPeriodEnd: usize = 0x34; // GameTime_t
pub const m_fWarmupPeriodStart: usize = 0x38; // GameTime_t
pub const m_nTotalPausedTicks: usize = 0x3C; // int32_t
pub const m_nPauseStartTick: usize = 0x40; // int32_t
pub const m_bServerPaused: usize = 0x44; // bool
pub const m_bGamePaused: usize = 0x45; // bool
pub const m_bTerroristTimeOutActive: usize = 0x46; // bool
pub const m_bCTTimeOutActive: usize = 0x47; // bool
pub const m_flTerroristTimeOutRemaining: usize = 0x48; // float
pub const m_flCTTimeOutRemaining: usize = 0x4C; // float
pub const m_nTerroristTimeOuts: usize = 0x50; // int32_t
pub const m_nCTTimeOuts: usize = 0x54; // int32_t
pub const m_bTechnicalTimeOut: usize = 0x58; // bool
pub const m_bMatchWaitingForResume: usize = 0x59; // bool
pub const m_iRoundTime: usize = 0x5C; // int32_t
pub const m_fMatchStartTime: usize = 0x60; // float
pub const m_fRoundStartTime: usize = 0x64; // GameTime_t
pub const m_flRestartRoundTime: usize = 0x68; // GameTime_t
pub const m_bGameRestart: usize = 0x6C; // bool
pub const m_flGameStartTime: usize = 0x70; // float
pub const m_timeUntilNextPhaseStarts: usize = 0x74; // float
pub const m_gamePhase: usize = 0x78; // int32_t
pub const m_totalRoundsPlayed: usize = 0x7C; // int32_t
pub const m_nRoundsPlayedThisPhase: usize = 0x80; // int32_t
pub const m_nOvertimePlaying: usize = 0x84; // int32_t
pub const m_iHostagesRemaining: usize = 0x88; // int32_t
pub const m_bAnyHostageReached: usize = 0x8C; // bool
pub const m_bMapHasBombTarget: usize = 0x8D; // bool
pub const m_bMapHasRescueZone: usize = 0x8E; // bool
pub const m_bMapHasBuyZone: usize = 0x8F; // bool
pub const m_bIsQueuedMatchmaking: usize = 0x90; // bool
pub const m_nQueuedMatchmakingMode: usize = 0x94; // int32_t
pub const m_bIsValveDS: usize = 0x98; // bool
pub const m_bLogoMap: usize = 0x99; // bool
pub const m_bPlayAllStepSoundsOnServer: usize = 0x9A; // bool
pub const m_iSpectatorSlotCount: usize = 0x9C; // int32_t
pub const m_MatchDevice: usize = 0xA0; // int32_t
pub const m_bHasMatchStarted: usize = 0xA4; // bool
pub const m_nNextMapInMapgroup: usize = 0xA8; // int32_t
pub const m_szTournamentEventName: usize = 0xAC; // char[512]
pub const m_szTournamentEventStage: usize = 0x2AC; // char[512]
pub const m_szMatchStatTxt: usize = 0x4AC; // char[512]
pub const m_szTournamentPredictionsTxt: usize = 0x6AC; // char[512]
pub const m_nTournamentPredictionsPct: usize = 0x8AC; // int32_t
pub const m_flCMMItemDropRevealStartTime: usize = 0x8B0; // GameTime_t
pub const m_flCMMItemDropRevealEndTime: usize = 0x8B4; // GameTime_t
pub const m_bIsDroppingItems: usize = 0x8B8; // bool
pub const m_bIsQuestEligible: usize = 0x8B9; // bool
pub const m_bIsHltvActive: usize = 0x8BA; // bool
pub const m_nGuardianModeWaveNumber: usize = 0x8BC; // int32_t
pub const m_nGuardianModeSpecialKillsRemaining: usize = 0x8C0; // int32_t
pub const m_nGuardianModeSpecialWeaponNeeded: usize = 0x8C4; // int32_t
pub const m_nGuardianGrenadesToGiveBots: usize = 0x8C8; // int32_t
pub const m_nNumHeaviesToSpawn: usize = 0x8CC; // int32_t
pub const m_numGlobalGiftsGiven: usize = 0x8D0; // uint32_t
pub const m_numGlobalGifters: usize = 0x8D4; // uint32_t
pub const m_numGlobalGiftsPeriodSeconds: usize = 0x8D8; // uint32_t
pub const m_arrFeaturedGiftersAccounts: usize = 0x8DC; // uint32_t[4]
pub const m_arrFeaturedGiftersGifts: usize = 0x8EC; // uint32_t[4]
pub const m_arrProhibitedItemIndices: usize = 0x8FC; // uint16_t[100]
pub const m_arrTournamentActiveCasterAccounts: usize = 0x9C4; // uint32_t[4]
pub const m_numBestOfMaps: usize = 0x9D4; // int32_t
pub const m_nHalloweenMaskListSeed: usize = 0x9D8; // int32_t
pub const m_bBombDropped: usize = 0x9DC; // bool
pub const m_bBombPlanted: usize = 0x9DD; // bool
pub const m_iRoundWinStatus: usize = 0x9E0; // int32_t
pub const m_eRoundWinReason: usize = 0x9E4; // int32_t
pub const m_bTCantBuy: usize = 0x9E8; // bool
pub const m_bCTCantBuy: usize = 0x9E9; // bool
pub const m_flGuardianBuyUntilTime: usize = 0x9EC; // GameTime_t
pub const m_iMatchStats_RoundResults: usize = 0x9F0; // int32_t[30]
pub const m_iMatchStats_PlayersAlive_CT: usize = 0xA68; // int32_t[30]
pub const m_iMatchStats_PlayersAlive_T: usize = 0xAE0; // int32_t[30]
pub const m_TeamRespawnWaveTimes: usize = 0xB58; // float[32]
pub const m_flNextRespawnWave: usize = 0xBD8; // GameTime_t[32]
pub const m_nServerQuestID: usize = 0xC58; // int32_t
pub const m_vMinimapMins: usize = 0xC5C; // Vector
pub const m_vMinimapMaxs: usize = 0xC68; // Vector
pub const m_MinimapVerticalSectionHeights: usize = 0xC74; // float[8]
pub const m_bDontIncrementCoopWave: usize = 0xC94; // bool
pub const m_bSpawnedTerrorHuntHeavy: usize = 0xC95; // bool
pub const m_nEndMatchMapGroupVoteTypes: usize = 0xC98; // int32_t[10]
pub const m_nEndMatchMapGroupVoteOptions: usize = 0xCC0; // int32_t[10]
pub const m_nEndMatchMapVoteWinner: usize = 0xCE8; // int32_t
pub const m_iNumConsecutiveCTLoses: usize = 0xCEC; // int32_t
pub const m_iNumConsecutiveTerroristLoses: usize = 0xCF0; // int32_t
pub const m_bMarkClientStopRecordAtRoundEnd: usize = 0xD10; // bool
pub const m_nMatchAbortedEarlyReason: usize = 0xD68; // int32_t
pub const m_bHasTriggeredRoundStartMusic: usize = 0xD6C; // bool
pub const m_bHasTriggeredCoopSpawnReset: usize = 0xD6D; // bool
pub const m_bSwitchingTeamsAtRoundReset: usize = 0xD6E; // bool
pub const m_pGameModeRules: usize = 0xD88; // CCSGameModeRules*
pub const m_RetakeRules: usize = 0xD90; // C_RetakeGameRules
pub const m_nMatchEndCount: usize = 0xEA8; // uint8_t
pub const m_nTTeamIntroVariant: usize = 0xEAC; // int32_t
pub const m_nCTTeamIntroVariant: usize = 0xEB0; // int32_t
pub const m_bTeamIntroPeriod: usize = 0xEB4; // bool
pub const m_flLastPerfSampleTime: usize = 0x4EC0; // double
}