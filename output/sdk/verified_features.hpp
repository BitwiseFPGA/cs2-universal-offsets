// =====================================================================
// verified_features.hpp â€” hand-curated working CS2 cheat features
// =====================================================================
//
// Each entry is a known-good offset / hook / ConVar trick that has been
// verified live against the current CS2 build. Constants are static
// constexpr so they can be used at compile time.
//
// =====================================================================

#pragma once

#include <cstdint>

namespace cs2::verified
{
    // ----- ESP [working] -----
    namespace ESP
    {
        constexpr const char* status = "working";
        constexpr std::ptrdiff_t C_BaseEntity__m_pGameSceneNode = 0x330; // CSkeletonInstance* â€” deref â†’ bone matrix + abs origin
        constexpr std::ptrdiff_t C_BaseEntity__m_iHealth = 0x34C; // int32 â€” 0 == dead
        constexpr std::ptrdiff_t C_BaseEntity__m_lifeState = 0x354; // uint8 â€” 0 == ALIVE
        constexpr std::ptrdiff_t C_BaseEntity__m_iTeamNum = 0x3EB; // uint8 â€” 2 = T, 3 = CT
        constexpr std::ptrdiff_t CGameSceneNode__m_vecAbsOrigin = 0xC8; // Vector3 â€” world position (ESP root)
        constexpr std::ptrdiff_t CSkeletonInstance__m_modelState = 0x150; // CModelState â€” embedded; live bone array lives inside
        constexpr std::ptrdiff_t CCSPlayerController__m_iszPlayerName = 0x6F0; // char[128] â€” UTF-8 nickname
        constexpr std::ptrdiff_t CCSPlayerController__m_hPawn = 0x6BC; // CHandle â€” controller â†’ pawn handle
        constexpr std::ptrdiff_t CCSPlayerController__m_iCompetitiveRanking = 0x878; // int32 â€” Premier rating (revealed pre-warmup)
        constexpr std::ptrdiff_t C_CSPlayerPawnBase__m_pWeaponServices = 0x11E0; // ptr â€” â†’ active weapon handle
        constexpr std::ptrdiff_t C_BasePlayerWeapon__m_iItemDefinitionIndex = 0x1BA; // uint16 â€” CSWeaponID for the held weapon
        constexpr std::ptrdiff_t C_BasePlayerWeapon__m_iClip1 = 0x16D8; // int32 â€” current magazine count
        constexpr std::ptrdiff_t CCSPlayerController_InGameMoneyServices__m_iAccount = 0x40; // int32 â€” current cash
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_ArmorValue = 0x1C74; // int32 â€” armor 0..100
        constexpr std::ptrdiff_t CCSPlayer_ItemServices__m_bHasHelmet = 0x49; // bool â€” kevlar+helmet flag
        constexpr std::ptrdiff_t CCSPlayerController_ActionTrackingServices__m_iKills = 0x30; // int32 â€” scoreboard kills
        constexpr std::ptrdiff_t CCSPlayerController_ActionTrackingServices__m_iDeaths = 0x34; // int32 â€” scoreboard deaths
        constexpr std::ptrdiff_t EntitySpottedState_t__m_bSpotted = 0x8; // bool â€” force true to reveal on radar
        constexpr std::ptrdiff_t EntitySpottedState_t__m_bSpottedByMask = 0xC; // uint32[2] â€” OR with 0xFFFFFFFF to spot for everyone
    }

    // ----- FOV Changer [working] -----
    namespace FOV_Changer
    {
        constexpr const char* status = "working";
        constexpr std::ptrdiff_t CBasePlayerController__m_iDesiredFOV = 0x784; // uint32 â€” a2x-named m_iDesiredFOV_OnController â€” the canonical write
        constexpr std::ptrdiff_t CCSPlayer_CameraServices__m_iFOV = 0x290; // uint32 â€” current camera FOV
        constexpr std::ptrdiff_t CCSPlayer_CameraServices__m_iFOVStart = 0x294; // uint32 â€” target camera FOV
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_pCameraServices = 0x1218; // CCSPlayer_CameraServices* â€” deref to reach m_iFOV / m_iFOVStart
    }

    // ----- Aimbot [working] -----
    namespace Aimbot
    {
        constexpr const char* status = "working";
        constexpr std::ptrdiff_t C_CSGameRules__m_bFreezePeriod = 0x40; // bool â€” freeze â€” no attacks possible
        constexpr std::ptrdiff_t C_CSGameRules__m_bWarmupPeriod = 0x41; // bool â€” warmup â€” no attacks possible
        constexpr std::ptrdiff_t C_CSGameRules__m_bIsValveDS = 0xA4; // bool â€” TRUE on Valve official MM (Overwatch + VAC Live live here) â€” soft 0.55Ã—
        constexpr std::ptrdiff_t C_CSGameRules__m_bHasMatchStarted = 0xB0; // bool â€” match-state gate
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_bWaitForNoAttack = 0x1C68; // bool â€” post-respawn / weapon-switch attack-release lockout
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_bIsDefusing = 0x1C4A; // bool â€” server forbids attack while defusing
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_bIsGrabbingHostage = 0x1C4B; // bool â€” server forbids attack while grabbing hostage
        constexpr std::ptrdiff_t C_BaseEntity__m_MoveType = 0x525; // MoveType_t â€” only WALK(2) / FLYGRAVITY(4) are normal play
        constexpr std::ptrdiff_t C_CSWeaponBaseGun__m_zoomLevel = 0x1CB0; // int32 â€” 0 = unscoped â€” refuse silent fire on snipers when zoom == 0
        constexpr std::ptrdiff_t C_CSWeaponBaseGun__m_bNeedsBoltAction = 0x1CCD; // bool â€” AWP/SSG/Scout bolt-cycle lockout
        constexpr std::ptrdiff_t C_CSWeaponBase__m_bInReload = 0x17F4; // bool â€” weapon mid-reload
        constexpr std::ptrdiff_t C_BasePlayerWeapon__m_iClip1 = 0x16D8; // int32 â€” 0 â‡’ no bullet possible
        constexpr std::ptrdiff_t C_BasePlayerWeapon__m_nNextPrimaryAttackTick = 0x16C8; // int32 â€” absolute server tick when next attack allowed
        constexpr std::ptrdiff_t CBasePlayerController__m_nTickBase = 0x6B8; // int32 â€” local tick counter; compare against m_nNextPrimaryAttackTick
        constexpr std::ptrdiff_t EntitySpottedState_t__m_bSpottedByMask = 0xC; // uint32[2] â€” bit per spotter â€” when a real enemy has us in PVS, throttle 0.65Ã—
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_iIDEntIndex = 0x33DC; // int32 â€” entity local crosshair rests on â€” bypass throttle when matches target
        constexpr std::ptrdiff_t C_BaseEntity__m_vecVelocity = 0x430; // Vector3 â€” soft throttle 1.0 â†’ 0.5 linear from 80 â†’ 180 u/s horizontal speed
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_pAimPunchServices = 0x1490; // CCSPlayer_AimPunchServices* â€” owns aim-punch cache vector
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_iShotsFired = 0x1C5C; // int32 â€” consecutive shots this trigger pull (drives spread)
        constexpr std::ptrdiff_t C_CSWeaponBase__m_flRecoilIndex = 0x17E0; // float â€” recoil pattern index
        constexpr std::ptrdiff_t C_EconEntity__m_iItemDefinitionIndex__abs_ = 0x15C2; // uint16 â€” abs on weapon = m_AttributeManager(0x13B8) + m_Item(0x50) + 0x1BA. Snipers: AWP=9, SSG08=40, G3SG1=11, SCAR20=38
    }

    // ----- Triggerbot (Seeded) [working] -----
    namespace Triggerbot__Seeded_
    {
        constexpr const char* status = "working";
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_iIDEntIndex = 0x33DC; // int32 â€” entity local crosshair currently rests on â€” primary target signal
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_iShotsFired = 0x1C5C; // int32 â€” drives spread seed
        constexpr std::ptrdiff_t C_CSPlayerPawn__m_pAimPunchServices = 0x1490; // CCSPlayer_AimPunchServices* â€” deref for live aim-punch vec used in the seed
    }

    // ----- Skin Changer [working] -----
    namespace Skin_Changer
    {
        constexpr const char* status = "working";
        constexpr std::ptrdiff_t C_EconItemView__m_iItemDefinitionIndex = 0x1BA; // uint16 â€” weapon definition (CSWeaponID)
        constexpr std::ptrdiff_t C_EconItemView__m_iItemIDLow = 0x1C4; // uint32 â€” set 0xFFFFFFFF to force EconItemView lookup miss â†’ fallback path
        constexpr std::ptrdiff_t C_EconItemView__m_iItemIDHigh = 0x1C8; // uint32 â€” set 0xFFFFFFFF (paired with m_iItemIDLow)
        constexpr std::ptrdiff_t C_EconItemView__m_iEntityQuality = 0x1C0; // int32 â€” quality slot used by the composite shader
        constexpr std::ptrdiff_t C_EconItemView__m_nFallbackPaintKit = 0x1D0; // uint32 â€” paint kit ID (the actual 'skin')
        constexpr std::ptrdiff_t C_EconItemView__m_nFallbackSeed = 0x1D4; // int32 â€” pattern seed
        constexpr std::ptrdiff_t C_EconItemView__m_flFallbackWear = 0x1D8; // float â€” 0.0 = factory new, 1.0 = battle-scarred
        constexpr std::ptrdiff_t C_EconItemView__m_nFallbackStatTrak = 0x1DC; // int32 â€” StatTrak counter (-1 disables)
    }

    // ----- Knife Changer [working] -----
    namespace Knife_Changer
    {
        constexpr const char* status = "working";
        constexpr std::ptrdiff_t C_BasePlayerWeapon__m_nSubclassID = 0x36C; // uint32 â€” knife subclass key (drives mesh + sequences + animgraph)
        constexpr std::ptrdiff_t C_BasePlayerWeapon__m_iItemDefinitionIndex = 0x1BA; // uint16 â€” must match the knife type for the chosen subclass
    }

} // namespace cs2::verified
