# Verified features

Working CS2 cheat features. Each entry lists the schema fields, convars and hooks needed to wire it up. Addresses come from the neighbouring `offsets.*` and `signatures.*` files.

Features: 6

---

## ESP

Iterate dwEntityList (CEntitySystem* in client.dll) every frame. For each pawn read m_iHealth (0 = dead), m_lifeState (0 = ALIVE), m_iTeamNum (2=T, 3=CT). Reach the world position through m_pGameSceneNode â†’ CGameSceneNode::m_vecAbsOrigin (+0xC8). For named ESP follow m_hController â†’ CCSPlayerController and read m_iszPlayerName. Held weapon comes from m_hActiveWeapon â†’ C_BasePlayerWeapon::m_iItemDefinitionIndex (CSWeaponID). Project with the engine's view matrix (dwViewMatrix, 4Ã—4 row-major) for world-to-screen. 

Skeleton: m_pGameSceneNode is actually a CSkeletonInstance for animated entities. Inside lives a CModelState at +0x150 whose m_modelSceneNode resolves to a CBoneState array (32 B per bone â€” vec3 pos at +0x00, quat at +0x20). Bone 6 = head, bone 5 = chest on the standard CSPlayer skeleton. 

Info-panel extras (money / armor / scoreboard / rank) come from the controller-side service blocks listed below.

### Fields

| Class | Field | Offset | Type | Note |
|---|---|---|---|---|
| `C_BaseEntity` | `m_pGameSceneNode` | `0x330` | `CSkeletonInstance*` | deref â†’ bone matrix + abs origin |
| `C_BaseEntity` | `m_iHealth` | `0x34C` | `int32` | 0 == dead |
| `C_BaseEntity` | `m_lifeState` | `0x354` | `uint8` | 0 == ALIVE |
| `C_BaseEntity` | `m_iTeamNum` | `0x3EB` | `uint8` | 2 = T, 3 = CT |
| `CGameSceneNode` | `m_vecAbsOrigin` | `0xC8` | `Vector3` | world position (ESP root) |
| `CSkeletonInstance` | `m_modelState` | `0x150` | `CModelState` | embedded; live bone array lives inside |
| `CCSPlayerController` | `m_iszPlayerName` | `0x6F0` | `char[128]` | UTF-8 nickname |
| `CCSPlayerController` | `m_hPawn` | `0x6BC` | `CHandle` | controller â†’ pawn handle |
| `CCSPlayerController` | `m_iCompetitiveRanking` | `0x878` | `int32` | Premier rating (revealed pre-warmup) |
| `C_CSPlayerPawnBase` | `m_pWeaponServices` | `0x11E0` | `ptr` | â†’ active weapon handle |
| `C_BasePlayerWeapon` | `m_iItemDefinitionIndex` | `0x1BA` | `uint16` | CSWeaponID for the held weapon |
| `C_BasePlayerWeapon` | `m_iClip1` | `0x16D8` | `int32` | current magazine count |
| `CCSPlayerController_InGameMoneyServices` | `m_iAccount` | `0x40` | `int32` | current cash |
| `C_CSPlayerPawn` | `m_ArmorValue` | `0x1C74` | `int32` | armor 0..100 |
| `CCSPlayer_ItemServices` | `m_bHasHelmet` | `0x49` | `bool` | kevlar+helmet flag |
| `CCSPlayerController_ActionTrackingServices` | `m_iKills` | `0x30` | `int32` | scoreboard kills |
| `CCSPlayerController_ActionTrackingServices` | `m_iDeaths` | `0x34` | `int32` | scoreboard deaths |
| `EntitySpottedState_t` | `m_bSpotted` | `0x8` | `bool` | force true to reveal on radar |
| `EntitySpottedState_t` | `m_bSpottedByMask` | `0xC` | `uint32[2]` | OR with 0xFFFFFFFF to spot for everyone |

---

## FOV Changer

Two-prong approach. (1) Hook GetWorldFov in client.dll (signature SetWorldFov, E8-CALL @ +1) and return the desired value when the local pawn is not scoped â€” keeps the value sticky against engine resets. (2) Every tick write the desired FOV into m_iFOV and m_iFOVStart on the camera services AND into m_iDesiredFOV on the local controller. The controller-level field is the canonical source the renderer reads; without it the camera-services side gets clobbered back to default.

### Fields

| Class | Field | Offset | Type | Note |
|---|---|---|---|---|
| `CBasePlayerController` | `m_iDesiredFOV` | `0x784` | `uint32` | a2x-named m_iDesiredFOV_OnController â€” the canonical write |
| `CCSPlayer_CameraServices` | `m_iFOV` | `0x290` | `uint32` | current camera FOV |
| `CCSPlayer_CameraServices` | `m_iFOVStart` | `0x294` | `uint32` | target camera FOV |
| `C_CSPlayerPawn` | `m_pCameraServices` | `0x1218` | `CCSPlayer_CameraServices*` | deref to reach m_iFOV / m_iFOVStart |

### Hooks

| Function | Module | Signature | Action |
|---|---|---|---|
| `GetWorldFov` | `client.dll` | `SetWorldFov` | Return cfg.fovValue when not scoped, else delegate to original. |

---

## Aimbot

Per-tick phase machine driven from CCSGOInput::CreateMove (IDLE â†’ REACTING â†’ ATTACKING â†’ CORRECTING â†’ LOCKED). Target selection scores enemies on FOV delta, distance, visibility (engine trace) and weighting flags. Final angle delivery happens via a hooked CSGOInputHistoryEntry::WriteSubtick (signature `48 89 5C 24 ? 55 57 41 56 48 8D 6C 24 ? 48 81 EC B0 00 00 00 8B 01 48 8B F9 81 4A 10 00 02`, unique match @ 0x180C53DB0 in build 14152) so the override only touches per-subtick shoot angles (fe[7..9]) on attack subticks (a3 != 0). The real view-angle stream replayed for spectators / GOTV / Overwatch (fe[4..6]) is NEVER touched â€” public internals overwrite both and trip every demo-review heuristic. 

Hard-suppress gates (any true â‡’ skip the angle write entirely): freeze / warmup, m_bWaitForNoAttack (post-respawn / weapon-switch lockout), no-scope sniper, m_bNeedsBoltAction (AWP/SSG bolt cycle), m_bInReload, m_iClip1 == 0, m_nNextPrimaryAttackTick > tickBase + 1, m_bIsDefusing, m_bIsGrabbingHostage, MoveType not WALK / FLYGRAVITY. Soft throttle scales the 28Â°/subtick flick cap: m_bIsValveDS Ã— 0.55, observerCount Ã— 0.55, m_bSpotted+observers Ã— 0.65, horizontal speed > 80 u/s linearly ramps to 0.5 (caps at 180 u/s), Â±0.10Â° LCG jitter. Crosshair-aligned bypass: when local m_iIDEntIndex resolves (slot â†’ controller â†’ m_hPlayerPawn) to the silent-aim target, the throttle is bypassed (the player legitimately had the crosshair on them).

### Fields

| Class | Field | Offset | Type | Note |
|---|---|---|---|---|
| `C_CSGameRules` | `m_bFreezePeriod` | `0x40` | `bool` | freeze â€” no attacks possible |
| `C_CSGameRules` | `m_bWarmupPeriod` | `0x41` | `bool` | warmup â€” no attacks possible |
| `C_CSGameRules` | `m_bIsValveDS` | `0xA4` | `bool` | TRUE on Valve official MM (Overwatch + VAC Live live here) â€” soft 0.55Ã— |
| `C_CSGameRules` | `m_bHasMatchStarted` | `0xB0` | `bool` | match-state gate |
| `C_CSPlayerPawn` | `m_bWaitForNoAttack` | `0x1C68` | `bool` | post-respawn / weapon-switch attack-release lockout |
| `C_CSPlayerPawn` | `m_bIsDefusing` | `0x1C4A` | `bool` | server forbids attack while defusing |
| `C_CSPlayerPawn` | `m_bIsGrabbingHostage` | `0x1C4B` | `bool` | server forbids attack while grabbing hostage |
| `C_BaseEntity` | `m_MoveType` | `0x525` | `MoveType_t` | only WALK(2) / FLYGRAVITY(4) are normal play |
| `C_CSWeaponBaseGun` | `m_zoomLevel` | `0x1CB0` | `int32` | 0 = unscoped â€” refuse silent fire on snipers when zoom == 0 |
| `C_CSWeaponBaseGun` | `m_bNeedsBoltAction` | `0x1CCD` | `bool` | AWP/SSG/Scout bolt-cycle lockout |
| `C_CSWeaponBase` | `m_bInReload` | `0x17F4` | `bool` | weapon mid-reload |
| `C_BasePlayerWeapon` | `m_iClip1` | `0x16D8` | `int32` | 0 â‡’ no bullet possible |
| `C_BasePlayerWeapon` | `m_nNextPrimaryAttackTick` | `0x16C8` | `int32` | absolute server tick when next attack allowed |
| `CBasePlayerController` | `m_nTickBase` | `0x6B8` | `int32` | local tick counter; compare against m_nNextPrimaryAttackTick |
| `EntitySpottedState_t` | `m_bSpottedByMask` | `0xC` | `uint32[2]` | bit per spotter â€” when a real enemy has us in PVS, throttle 0.65Ã— |
| `C_CSPlayerPawn` | `m_iIDEntIndex` | `0x33DC` | `int32` | entity local crosshair rests on â€” bypass throttle when matches target |
| `C_BaseEntity` | `m_vecVelocity` | `0x430` | `Vector3` | soft throttle 1.0 â†’ 0.5 linear from 80 â†’ 180 u/s horizontal speed |
| `C_CSPlayerPawn` | `m_pAimPunchServices` | `0x1490` | `CCSPlayer_AimPunchServices*` | owns aim-punch cache vector |
| `C_CSPlayerPawn` | `m_iShotsFired` | `0x1C5C` | `int32` | consecutive shots this trigger pull (drives spread) |
| `C_CSWeaponBase` | `m_flRecoilIndex` | `0x17E0` | `float` | recoil pattern index |
| `C_EconEntity` | `m_iItemDefinitionIndex (abs)` | `0x15C2` | `uint16` | abs on weapon = m_AttributeManager(0x13B8) + m_Item(0x50) + 0x1BA. Snipers: AWP=9, SSG08=40, G3SG1=11, SCAR20=38 |

### Hooks

| Function | Module | Signature | Action |
|---|---|---|---|
| `CCSGOInput::CreateMove` | `client.dll` | `CreateMove` | Aimbot's primary tick â€” runs phase machine, performs target select + angle snap, sets fire latch. |
| `CSGOInputHistoryEntry::WriteSubtick` | `client.dll` | `48 89 5C 24 ? 55 57 41 56 48 8D 6C 24 ? 48 81 EC B0 00 00 00 8B 01 48 8B F9 81 4A 10 00 02` | Per-subtick angle override. (1) BAIL if a3 == 0. (2) NEVER touch fe[4..6]. (3) Save fe[7..9], evaluate hard-suppress; if any gate true call original unmodified, else apply target angle + soft-throttle + Â±0.10Â° LCG jitter to fe[7..9] only, call original, restore. |

---

## Triggerbot (Seeded)

Per-tick seeded prediction. Reads the live spread seed (m_iShotsFired + m_aimPunchAngle) and re-runs Valve's spread RNG via SpreadSeedGen + CalcSpread to compute exactly where the next bullet would fly, then traces from local eye to that point through the EngineTrace pipeline (TraceInitData / Info / Filter + TraceCreate + TraceGetInfo + TraceHandleBulletPen). Strict-window mode tests only ticks {0, +1} and ALL must hit before firing; wide-window mode accepts ANY hit in {0, +1, -1, +2}. Local eye position is projected by localVel Ã— leadTime so the test geometry matches engine fire-time eye pos. 

Uses the REAL m_fAccuracyPenalty + m_flTurningInaccuracy in the predictor â€” the earlier 'perfect-shot' override that zeroed client spread caused server desync (kill sound but no damage); that path is OFF by default. 

Cooperates with aimbot: trigger defers whenever Aimbot::state.phase âˆˆ {REACTING, ATTACKING, CORRECTING} and only fires in PHASE_LOCKED or PHASE_IDLE. Trigger's SendInput LBUTTON edges are masked out of aimbot's rawAim filter via Triggerbot::g_synthClickUntilMs (80 ms window) so the synthetic click can't wake aimbot uninvited. 120 ms debounce on rawAim release.

### Fields

| Class | Field | Offset | Type | Note |
|---|---|---|---|---|
| `C_CSPlayerPawn` | `m_iIDEntIndex` | `0x33DC` | `int32` | entity local crosshair currently rests on â€” primary target signal |
| `C_CSPlayerPawn` | `m_iShotsFired` | `0x1C5C` | `int32` | drives spread seed |
| `C_CSPlayerPawn` | `m_pAimPunchServices` | `0x1490` | `CCSPlayer_AimPunchServices*` | deref for live aim-punch vec used in the seed |

### Hooks

| Function | Module | Signature | Action |
|---|---|---|---|
| `CCSPlayerAnimGraphState::CalcSpread` | `client.dll` | `CalcSpread` | Cache (mode, baseSpread, inaccuracy) per itemDef so the predictor can re-run the same math against the live seed. |
| `TraceCreate` | `client.dll` | `TraceCreate` | Pass-through; entry point for predicted-shot vischecks. |
| `NoSpread1` | `client.dll` | `NoSpread1` | Optional client-spread suppress for the perfect-shot path. DISABLED by default â€” server desync risk. |

---

## Skin Changer

Writes m_nFallbackPaintKit / m_nFallbackSeed / m_flFallbackWear / m_iEntityQuality on each weapon then forces the modern paint-apply path: ApplyEconCustomization(weapon, 1) â†’ sub_181079790 â†’ sub_18105AAF0 (which actually consumes the fallback fields and queues 'clientside_reload_custom_econ' to rebuild the composite material). RegenerateWeaponSkin alone is INSUFFICIENT â€” it only handles the legacy static paint table. GetCustomPaintKitIndex is polled to detect rejection and gate re-apply work instead of hammering ApplyEconCustomization every tick. Setting m_iItemIDLow/High to 0xFFFFFFFF forces the EconItemView lookup to fail â†’ fallback path taken.

### Fields

| Class | Field | Offset | Type | Note |
|---|---|---|---|---|
| `C_EconItemView` | `m_iItemDefinitionIndex` | `0x1BA` | `uint16` | weapon definition (CSWeaponID) |
| `C_EconItemView` | `m_iItemIDLow` | `0x1C4` | `uint32` | set 0xFFFFFFFF to force EconItemView lookup miss â†’ fallback path |
| `C_EconItemView` | `m_iItemIDHigh` | `0x1C8` | `uint32` | set 0xFFFFFFFF (paired with m_iItemIDLow) |
| `C_EconItemView` | `m_iEntityQuality` | `0x1C0` | `int32` | quality slot used by the composite shader |
| `C_EconItemView` | `m_nFallbackPaintKit` | `0x1D0` | `uint32` | paint kit ID (the actual 'skin') |
| `C_EconItemView` | `m_nFallbackSeed` | `0x1D4` | `int32` | pattern seed |
| `C_EconItemView` | `m_flFallbackWear` | `0x1D8` | `float` | 0.0 = factory new, 1.0 = battle-scarred |
| `C_EconItemView` | `m_nFallbackStatTrak` | `0x1DC` | `int32` | StatTrak counter (-1 disables) |

### Hooks

| Function | Module | Signature | Action |
|---|---|---|---|
| `ApplyEconCustomization` | `client.dll` | `ApplyEconCustomization` | Modern paint-apply entry; consumes m_nFallback* and queues composite rebuild. |
| `RegenerateWeaponSkin` | `client.dll` | `RegenerateWeaponSkin` | Legacy static-paint pass; called for completeness. |
| `GetCustomPaintKitIndex` | `client.dll` | `CEconItemView::GetCustomPaintKitIndex` | Read live paint kit to detect rejection and gate re-apply. |

---

## Knife Changer

Spoofs m_nSubclassID on the knife entity, calls UpdateSubclass to re-bind the subclass-data ptr at weapon+0x388 (the per-knife sequence set), then AnimGraphRebuild(controller, 2) to tear down the existing CNmGraphInstance at controller+0x448 and let the manager re-bind from the (now-updated) vdata's animgraph. Without the rebuild the knife mesh swaps but inspect / deploy / swing animations stay on the OLD subclass's sequences (Emerald Butterfly mesh + default-knife inspect anim was the symptom). SetMeshGroupMask refreshes the visible mesh after the subclass change.

### Fields

| Class | Field | Offset | Type | Note |
|---|---|---|---|---|
| `C_BasePlayerWeapon` | `m_nSubclassID` | `0x36C` | `uint32` | knife subclass key (drives mesh + sequences + animgraph) |
| `C_BasePlayerWeapon` | `m_iItemDefinitionIndex` | `0x1BA` | `uint16` | must match the knife type for the chosen subclass |

### Hooks

| Function | Module | Signature | Action |
|---|---|---|---|
| `UpdateSubclass` | `client.dll` | `48 8B 41 10 48 8B D9 8B 50 30` | Re-bind subclass-data ptr at weapon+0x388 after writing m_nSubclassID. |
| `AnimGraphRebuild` | `client.dll` | `AnimGraphRebuild` | Mode = 2: destroy CNmGraphInstance and re-bind from new vdata's animgraph. |
| `SetMeshGroupMask` | `client.dll` | `SetMeshGroupMask` | Refresh visible mesh after subclass change. |

---

