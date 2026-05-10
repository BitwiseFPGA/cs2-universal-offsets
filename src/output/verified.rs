//! Verified working features â€” hand-curated catalogue of offsets, hooks
//! and ConVar tricks that have been **confirmed working in a live CS2
//! internal cheat** against the current build.
//!
//! Most of this data is already present in the auto-extracted offset
//! / signature / schema files this tool produces. The point of this
//! module is to give consumers a single place that says, in plain
//! English, *"yes, this offset on this entity does this exact thing,
//! and here is the gotcha you need to know to make it work."*
//!
//! a2x's pelite-based pipeline gives us the canonical RVAs and schema
//! offsets â€” that data is the source of truth and lives in the regular
//! `offsets.*` / `client_dll.*` / `signatures.*` files. This catalogue
//! cross-references those values with verified live-engine notes so a
//! cheat developer can copy-paste a feature and have it work first try.

use serde_json::json;

#[derive(Clone, Copy)]
pub struct VerifiedField {
    /// e.g. "C_SmokeGrenadeProjectile"
    pub class: &'static str,
    /// e.g. "m_nSmokeEffectTickBegin"
    pub field: &'static str,
    /// hex offset relative to the class base (entity)
    pub offset: u32,
    /// e.g. "int32" / "Vector3" / "bool"
    pub ty: &'static str,
    /// short note about what to write / how to read
    pub note: &'static str,
}

#[derive(Clone, Copy)]
pub struct VerifiedFeature {
    /// e.g. "No Smoke"
    pub name: &'static str,
    /// "working" / "broken" / "partial" â€” at-a-glance status from the
    /// last live confirmation of this feature in the internal cheat.
    pub status: &'static str,
    /// short paragraph explaining what we tested + where to write
    pub summary: &'static str,
    /// fields touched
    pub fields: &'static [VerifiedField],
    /// ConVar tricks (name + flags-to-strip + value-slot offset) â€” empty if N/A
    pub convars: &'static [VerifiedConVar],
    /// Hooks installed (function + module + signature key in database.rs)
    pub hooks: &'static [VerifiedHook],
}

#[derive(Clone, Copy)]
pub struct VerifiedConVar {
    /// ConVar name
    pub name: &'static str,
    /// flags to strip from cvar+0x30 (e.g. FCVAR_CHEAT=0x400)
    pub strip_flags: u32,
    /// modern Source 2 ConVar<T> value lives at cvar+0x58 â€” set true
    /// if we have to write *both* the legacy +0x40 union AND the
    /// modern +0x58 slot
    pub write_both_slots: bool,
    /// what value(s) we write
    pub value: &'static str,
}

#[derive(Clone, Copy)]
pub struct VerifiedHook {
    /// e.g. "DrawSkyboxArray"
    pub function: &'static str,
    /// e.g. "scenesystem.dll"
    pub module: &'static str,
    /// signature database key (look up in src/signatures/database.rs)
    pub signature: &'static str,
    /// what we do once hooked
    pub action: &'static str,
}

// ----------------------------------------------------------------------
// Catalogue. Add entries as new features are verified working in-game.
// Build target: CS2 14152+ (April 2026).
// ----------------------------------------------------------------------
pub static FEATURES: &[VerifiedFeature] = &[
    // ------------------------------------------------------------------
    // ESP â€” entity walking, world-to-screen, bones, info panel data.
    // ------------------------------------------------------------------
    VerifiedFeature {
        name: "ESP",
        status: "working",
        summary: "Iterate dwEntityList (CEntitySystem* in client.dll) every frame. \
                  For each pawn read m_iHealth (0 = dead), m_lifeState (0 = ALIVE), \
                  m_iTeamNum (2=T, 3=CT). Reach the world position through \
                  m_pGameSceneNode â†’ CGameSceneNode::m_vecAbsOrigin (+0xC8). For \
                  named ESP follow m_hController â†’ CCSPlayerController and read \
                  m_iszPlayerName. Held weapon comes from m_hActiveWeapon â†’ \
                  C_BasePlayerWeapon::m_iItemDefinitionIndex (CSWeaponID). Project \
                  with the engine's view matrix (dwViewMatrix, 4Ã—4 row-major) for \
                  world-to-screen. \n\nSkeleton: m_pGameSceneNode is actually a \
                  CSkeletonInstance for animated entities. Inside lives a \
                  CModelState at +0x150 whose m_modelSceneNode resolves to a \
                  CBoneState array (32 B per bone â€” vec3 pos at +0x00, quat at \
                  +0x20). Bone 6 = head, bone 5 = chest on the standard CSPlayer \
                  skeleton. \n\nInfo-panel extras (money / armor / scoreboard / \
                  rank) come from the controller-side service blocks listed below.",
        fields: &[
            // Core pawn fields
            VerifiedField { class: "C_BaseEntity",            field: "m_pGameSceneNode",     offset: 0x330,  ty: "CSkeletonInstance*", note: "deref â†’ bone matrix + abs origin" },
            VerifiedField { class: "C_BaseEntity",            field: "m_iHealth",            offset: 0x34C,  ty: "int32",              note: "0 == dead" },
            VerifiedField { class: "C_BaseEntity",            field: "m_lifeState",          offset: 0x354,  ty: "uint8",              note: "0 == ALIVE" },
            VerifiedField { class: "C_BaseEntity",            field: "m_iTeamNum",           offset: 0x3EB,  ty: "uint8",              note: "2 = T, 3 = CT" },
            VerifiedField { class: "CGameSceneNode",          field: "m_vecAbsOrigin",       offset: 0xC8,   ty: "Vector3",            note: "world position (ESP root)" },
            VerifiedField { class: "CSkeletonInstance",       field: "m_modelState",         offset: 0x150,  ty: "CModelState",        note: "embedded; live bone array lives inside" },
            // Controller (named ESP, scoreboard, rank)
            VerifiedField { class: "CCSPlayerController",     field: "m_iszPlayerName",      offset: 0x6F0,  ty: "char[128]",          note: "UTF-8 nickname" },
            VerifiedField { class: "CCSPlayerController",     field: "m_hPawn",              offset: 0x6BC,  ty: "CHandle",            note: "controller â†’ pawn handle" },
            VerifiedField { class: "CCSPlayerController",     field: "m_iCompetitiveRanking", offset: 0x878, ty: "int32",              note: "Premier rating (revealed pre-warmup)" },
            // Weapon (held-item ESP)
            VerifiedField { class: "C_CSPlayerPawnBase",      field: "m_pWeaponServices",    offset: 0x11E0, ty: "ptr",                note: "â†’ active weapon handle" },
            VerifiedField { class: "C_BasePlayerWeapon",      field: "m_iItemDefinitionIndex", offset: 0x1BA, ty: "uint16",           note: "CSWeaponID for the held weapon" },
            VerifiedField { class: "C_BasePlayerWeapon",      field: "m_iClip1",             offset: 0x16D8, ty: "int32",              note: "current magazine count" },
            // Money / armor / scoreboard for the info panel
            VerifiedField { class: "CCSPlayerController_InGameMoneyServices", field: "m_iAccount", offset: 0x40,  ty: "int32", note: "current cash" },
            VerifiedField { class: "C_CSPlayerPawn",          field: "m_ArmorValue",         offset: 0x1C74, ty: "int32",              note: "armor 0..100" },
            VerifiedField { class: "CCSPlayer_ItemServices",  field: "m_bHasHelmet",         offset: 0x49,   ty: "bool",               note: "kevlar+helmet flag" },
            VerifiedField { class: "CCSPlayerController_ActionTrackingServices", field: "m_iKills",   offset: 0x30, ty: "int32", note: "scoreboard kills" },
            VerifiedField { class: "CCSPlayerController_ActionTrackingServices", field: "m_iDeaths",  offset: 0x34, ty: "int32", note: "scoreboard deaths" },
            // Radar / spotted force
            VerifiedField { class: "EntitySpottedState_t",    field: "m_bSpotted",           offset: 0x8,    ty: "bool",               note: "force true to reveal on radar" },
            VerifiedField { class: "EntitySpottedState_t",    field: "m_bSpottedByMask",     offset: 0xC,    ty: "uint32[2]",          note: "OR with 0xFFFFFFFF to spot for everyone" },
        ],
        convars: &[],
        hooks: &[],
    },

    // ------------------------------------------------------------------
    // FOV Changer â€” hook + per-tick controller / camera-services write.
    // ------------------------------------------------------------------
    VerifiedFeature {
        name: "FOV Changer",
        status: "working",
        summary: "Two-prong approach. (1) Hook GetWorldFov in client.dll (signature \
                  SetWorldFov, E8-CALL @ +1) and return the desired value when the \
                  local pawn is not scoped â€” keeps the value sticky against engine \
                  resets. (2) Every tick write the desired FOV into m_iFOV and \
                  m_iFOVStart on the camera services AND into m_iDesiredFOV on the \
                  local controller. The controller-level field is the canonical \
                  source the renderer reads; without it the camera-services side \
                  gets clobbered back to default.",
        fields: &[
            VerifiedField { class: "CBasePlayerController",     field: "m_iDesiredFOV", offset: 0x784,  ty: "uint32",                       note: "a2x-named m_iDesiredFOV_OnController â€” the canonical write" },
            VerifiedField { class: "CCSPlayer_CameraServices",  field: "m_iFOV",        offset: 0x290,  ty: "uint32",                       note: "current camera FOV" },
            VerifiedField { class: "CCSPlayer_CameraServices",  field: "m_iFOVStart",   offset: 0x294,  ty: "uint32",                       note: "target camera FOV" },
            VerifiedField { class: "C_CSPlayerPawn",            field: "m_pCameraServices", offset: 0x1218, ty: "CCSPlayer_CameraServices*", note: "deref to reach m_iFOV / m_iFOVStart" },
        ],
        convars: &[],
        hooks: &[
            VerifiedHook {
                function: "GetWorldFov",
                module: "client.dll",
                signature: "SetWorldFov",
                action: "Return cfg.fovValue when not scoped, else delegate to original.",
            },
        ],
    },

    // ------------------------------------------------------------------
    // Aimbot â€” phase machine + WriteSubtick anti-detection gate stack.
    // ------------------------------------------------------------------
    VerifiedFeature {
        name: "Aimbot",
        status: "working",
        summary: "Per-tick phase machine driven from CCSGOInput::CreateMove (IDLE â†’ \
                  REACTING â†’ ATTACKING â†’ CORRECTING â†’ LOCKED). Target selection \
                  scores enemies on FOV delta, distance, visibility (engine trace) \
                  and weighting flags. Final angle delivery happens via a hooked \
                  CSGOInputHistoryEntry::WriteSubtick (signature \
                  `48 89 5C 24 ? 55 57 41 56 48 8D 6C 24 ? 48 81 EC B0 00 00 00 8B \
                  01 48 8B F9 81 4A 10 00 02`, unique match @ 0x180C53DB0 in build \
                  14152) so the override only touches per-subtick shoot angles \
                  (fe[7..9]) on attack subticks (a3 != 0). The real view-angle \
                  stream replayed for spectators / GOTV / Overwatch (fe[4..6]) is \
                  NEVER touched â€” public internals overwrite both and trip every \
                  demo-review heuristic. \n\nHard-suppress gates (any true â‡’ skip \
                  the angle write entirely): freeze / warmup, m_bWaitForNoAttack \
                  (post-respawn / weapon-switch lockout), no-scope sniper, \
                  m_bNeedsBoltAction (AWP/SSG bolt cycle), m_bInReload, \
                  m_iClip1 == 0, m_nNextPrimaryAttackTick > tickBase + 1, \
                  m_bIsDefusing, m_bIsGrabbingHostage, MoveType not WALK / \
                  FLYGRAVITY. Soft throttle scales the 28Â°/subtick flick cap: \
                  m_bIsValveDS Ã— 0.55, observerCount Ã— 0.55, m_bSpotted+observers \
                  Ã— 0.65, horizontal speed > 80 u/s linearly ramps to 0.5 (caps at \
                  180 u/s), Â±0.10Â° LCG jitter. Crosshair-aligned bypass: when \
                  local m_iIDEntIndex resolves (slot â†’ controller â†’ m_hPlayerPawn) \
                  to the silent-aim target, the throttle is bypassed (the player \
                  legitimately had the crosshair on them).",
        fields: &[
            // Hard-suppress
            VerifiedField { class: "C_CSGameRules",         field: "m_bFreezePeriod",          offset: 0x40,   ty: "bool",            note: "freeze â€” no attacks possible" },
            VerifiedField { class: "C_CSGameRules",         field: "m_bWarmupPeriod",          offset: 0x41,   ty: "bool",            note: "warmup â€” no attacks possible" },
            VerifiedField { class: "C_CSGameRules",         field: "m_bIsValveDS",             offset: 0xA4,   ty: "bool",            note: "TRUE on Valve official MM (Overwatch + VAC Live live here) â€” soft 0.55Ã—" },
            VerifiedField { class: "C_CSGameRules",         field: "m_bHasMatchStarted",       offset: 0xB0,   ty: "bool",            note: "match-state gate" },
            VerifiedField { class: "C_CSPlayerPawn",        field: "m_bWaitForNoAttack",       offset: 0x1C68, ty: "bool",            note: "post-respawn / weapon-switch attack-release lockout" },
            VerifiedField { class: "C_CSPlayerPawn",        field: "m_bIsDefusing",            offset: 0x1C4A, ty: "bool",            note: "server forbids attack while defusing" },
            VerifiedField { class: "C_CSPlayerPawn",        field: "m_bIsGrabbingHostage",     offset: 0x1C4B, ty: "bool",            note: "server forbids attack while grabbing hostage" },
            VerifiedField { class: "C_BaseEntity",          field: "m_MoveType",               offset: 0x525,  ty: "MoveType_t",      note: "only WALK(2) / FLYGRAVITY(4) are normal play" },
            VerifiedField { class: "C_CSWeaponBaseGun",     field: "m_zoomLevel",              offset: 0x1CB0, ty: "int32",           note: "0 = unscoped â€” refuse silent fire on snipers when zoom == 0" },
            VerifiedField { class: "C_CSWeaponBaseGun",     field: "m_bNeedsBoltAction",       offset: 0x1CCD, ty: "bool",            note: "AWP/SSG/Scout bolt-cycle lockout" },
            VerifiedField { class: "C_CSWeaponBase",        field: "m_bInReload",              offset: 0x17F4, ty: "bool",            note: "weapon mid-reload" },
            VerifiedField { class: "C_BasePlayerWeapon",    field: "m_iClip1",                 offset: 0x16D8, ty: "int32",           note: "0 â‡’ no bullet possible" },
            VerifiedField { class: "C_BasePlayerWeapon",    field: "m_nNextPrimaryAttackTick", offset: 0x16C8, ty: "int32",           note: "absolute server tick when next attack allowed" },
            VerifiedField { class: "CBasePlayerController", field: "m_nTickBase",              offset: 0x6B8,  ty: "int32",           note: "local tick counter; compare against m_nNextPrimaryAttackTick" },
            // Soft-throttle inputs
            VerifiedField { class: "EntitySpottedState_t",  field: "m_bSpottedByMask",         offset: 0xC,    ty: "uint32[2]",       note: "bit per spotter â€” when a real enemy has us in PVS, throttle 0.65Ã—" },
            VerifiedField { class: "C_CSPlayerPawn",        field: "m_iIDEntIndex",            offset: 0x33DC, ty: "int32",           note: "entity local crosshair rests on â€” bypass throttle when matches target" },
            VerifiedField { class: "C_BaseEntity",          field: "m_vecVelocity",            offset: 0x430,  ty: "Vector3",         note: "soft throttle 1.0 â†’ 0.5 linear from 80 â†’ 180 u/s horizontal speed" },
            // Spread / aim-punch services
            VerifiedField { class: "C_CSPlayerPawn",        field: "m_pAimPunchServices",      offset: 0x1490, ty: "CCSPlayer_AimPunchServices*", note: "owns aim-punch cache vector" },
            VerifiedField { class: "C_CSPlayerPawn",        field: "m_iShotsFired",            offset: 0x1C5C, ty: "int32",           note: "consecutive shots this trigger pull (drives spread)" },
            VerifiedField { class: "C_CSWeaponBase",        field: "m_flRecoilIndex",          offset: 0x17E0, ty: "float",           note: "recoil pattern index" },
            // Weapon ID for sniper-specific gates
            VerifiedField { class: "C_EconEntity",          field: "m_iItemDefinitionIndex (abs)", offset: 0x15C2, ty: "uint16",      note: "abs on weapon = m_AttributeManager(0x13B8) + m_Item(0x50) + 0x1BA. Snipers: AWP=9, SSG08=40, G3SG1=11, SCAR20=38" },
        ],
        convars: &[],
        hooks: &[
            VerifiedHook {
                function: "CCSGOInput::CreateMove",
                module: "client.dll",
                signature: "CreateMove",
                action: "Aimbot's primary tick â€” runs phase machine, performs target select + angle snap, sets fire latch.",
            },
            VerifiedHook {
                function: "CSGOInputHistoryEntry::WriteSubtick",
                module: "client.dll",
                signature: "48 89 5C 24 ? 55 57 41 56 48 8D 6C 24 ? 48 81 EC B0 00 00 00 8B 01 48 8B F9 81 4A 10 00 02",
                action: "Per-subtick angle override. (1) BAIL if a3 == 0. (2) NEVER touch fe[4..6]. (3) Save fe[7..9], evaluate hard-suppress; if any gate true call original unmodified, else apply target angle + soft-throttle + Â±0.10Â° LCG jitter to fe[7..9] only, call original, restore.",
            },
        ],
    },

    // ------------------------------------------------------------------
    // Triggerbot â€” seeded predictor + cooperation with aimbot.
    // ------------------------------------------------------------------
    VerifiedFeature {
        name: "Triggerbot (Seeded)",
        status: "working",
        summary: "Per-tick seeded prediction. Reads the live spread seed \
                  (m_iShotsFired + m_aimPunchAngle) and re-runs Valve's spread RNG \
                  via SpreadSeedGen + CalcSpread to compute exactly where the next \
                  bullet would fly, then traces from local eye to that point through \
                  the EngineTrace pipeline (TraceInitData / Info / Filter + \
                  TraceCreate + TraceGetInfo + TraceHandleBulletPen). Strict-window \
                  mode tests only ticks {0, +1} and ALL must hit before firing; \
                  wide-window mode accepts ANY hit in {0, +1, -1, +2}. Local eye \
                  position is projected by localVel Ã— leadTime so the test geometry \
                  matches engine fire-time eye pos. \n\nUses the REAL \
                  m_fAccuracyPenalty + m_flTurningInaccuracy in the predictor â€” \
                  the earlier 'perfect-shot' override that zeroed client spread \
                  caused server desync (kill sound but no damage); that path is \
                  OFF by default. \n\nCooperates with aimbot: trigger defers \
                  whenever Aimbot::state.phase âˆˆ {REACTING, ATTACKING, CORRECTING} \
                  and only fires in PHASE_LOCKED or PHASE_IDLE. Trigger's \
                  SendInput LBUTTON edges are masked out of aimbot's rawAim \
                  filter via Triggerbot::g_synthClickUntilMs (80 ms window) so the \
                  synthetic click can't wake aimbot uninvited. 120 ms debounce on \
                  rawAim release.",
        fields: &[
            VerifiedField { class: "C_CSPlayerPawn",        field: "m_iIDEntIndex",       offset: 0x33DC, ty: "int32", note: "entity local crosshair currently rests on â€” primary target signal" },
            VerifiedField { class: "C_CSPlayerPawn",        field: "m_iShotsFired",       offset: 0x1C5C, ty: "int32", note: "drives spread seed" },
            VerifiedField { class: "C_CSPlayerPawn",        field: "m_pAimPunchServices", offset: 0x1490, ty: "CCSPlayer_AimPunchServices*", note: "deref for live aim-punch vec used in the seed" },
        ],
        convars: &[],
        hooks: &[
            VerifiedHook { function: "CCSPlayerAnimGraphState::CalcSpread", module: "client.dll", signature: "CalcSpread",  action: "Cache (mode, baseSpread, inaccuracy) per itemDef so the predictor can re-run the same math against the live seed." },
            VerifiedHook { function: "TraceCreate",                          module: "client.dll", signature: "TraceCreate", action: "Pass-through; entry point for predicted-shot vischecks." },
            VerifiedHook { function: "NoSpread1",                            module: "client.dll", signature: "NoSpread1",   action: "Optional client-spread suppress for the perfect-shot path. DISABLED by default â€” server desync risk." },
        ],
    },

    // ------------------------------------------------------------------
    // Skin Changer â€” paint kits + composite material rebuild.
    // ------------------------------------------------------------------
    VerifiedFeature {
        name: "Skin Changer",
        status: "working",
        summary: "Writes m_nFallbackPaintKit / m_nFallbackSeed / m_flFallbackWear / \
                  m_iEntityQuality on each weapon then forces the modern paint-apply \
                  path: ApplyEconCustomization(weapon, 1) â†’ sub_181079790 â†’ \
                  sub_18105AAF0 (which actually consumes the fallback fields and \
                  queues 'clientside_reload_custom_econ' to rebuild the composite \
                  material). RegenerateWeaponSkin alone is INSUFFICIENT â€” it only \
                  handles the legacy static paint table. GetCustomPaintKitIndex is \
                  polled to detect rejection and gate re-apply work instead of \
                  hammering ApplyEconCustomization every tick. Setting \
                  m_iItemIDLow/High to 0xFFFFFFFF forces the EconItemView lookup \
                  to fail â†’ fallback path taken.",
        fields: &[
            VerifiedField { class: "C_EconItemView", field: "m_iItemDefinitionIndex", offset: 0x1BA, ty: "uint16",  note: "weapon definition (CSWeaponID)" },
            VerifiedField { class: "C_EconItemView", field: "m_iItemIDLow",           offset: 0x1C4, ty: "uint32",  note: "set 0xFFFFFFFF to force EconItemView lookup miss â†’ fallback path" },
            VerifiedField { class: "C_EconItemView", field: "m_iItemIDHigh",          offset: 0x1C8, ty: "uint32",  note: "set 0xFFFFFFFF (paired with m_iItemIDLow)" },
            VerifiedField { class: "C_EconItemView", field: "m_iEntityQuality",       offset: 0x1C0, ty: "int32",   note: "quality slot used by the composite shader" },
            VerifiedField { class: "C_EconItemView", field: "m_nFallbackPaintKit",    offset: 0x1D0, ty: "uint32",  note: "paint kit ID (the actual 'skin')" },
            VerifiedField { class: "C_EconItemView", field: "m_nFallbackSeed",        offset: 0x1D4, ty: "int32",   note: "pattern seed" },
            VerifiedField { class: "C_EconItemView", field: "m_flFallbackWear",       offset: 0x1D8, ty: "float",   note: "0.0 = factory new, 1.0 = battle-scarred" },
            VerifiedField { class: "C_EconItemView", field: "m_nFallbackStatTrak",    offset: 0x1DC, ty: "int32",   note: "StatTrak counter (-1 disables)" },
        ],
        convars: &[],
        hooks: &[
            VerifiedHook { function: "ApplyEconCustomization",   module: "client.dll", signature: "ApplyEconCustomization",                action: "Modern paint-apply entry; consumes m_nFallback* and queues composite rebuild." },
            VerifiedHook { function: "RegenerateWeaponSkin",     module: "client.dll", signature: "RegenerateWeaponSkin",                  action: "Legacy static-paint pass; called for completeness." },
            VerifiedHook { function: "GetCustomPaintKitIndex",   module: "client.dll", signature: "CEconItemView::GetCustomPaintKitIndex", action: "Read live paint kit to detect rejection and gate re-apply." },
        ],
    },

    // ------------------------------------------------------------------
    // Knife Changer â€” model swap with full subclass + animgraph rebuild.
    // ------------------------------------------------------------------
    VerifiedFeature {
        name: "Knife Changer",
        status: "working",
        summary: "Spoofs m_nSubclassID on the knife entity, calls UpdateSubclass to \
                  re-bind the subclass-data ptr at weapon+0x388 (the per-knife \
                  sequence set), then AnimGraphRebuild(controller, 2) to tear \
                  down the existing CNmGraphInstance at controller+0x448 and let \
                  the manager re-bind from the (now-updated) vdata's animgraph. \
                  Without the rebuild the knife mesh swaps but inspect / deploy / \
                  swing animations stay on the OLD subclass's sequences (Emerald \
                  Butterfly mesh + default-knife inspect anim was the symptom). \
                  SetMeshGroupMask refreshes the visible mesh after the subclass \
                  change.",
        fields: &[
            VerifiedField { class: "C_BasePlayerWeapon", field: "m_nSubclassID",        offset: 0x36C, ty: "uint32", note: "knife subclass key (drives mesh + sequences + animgraph)" },
            VerifiedField { class: "C_BasePlayerWeapon", field: "m_iItemDefinitionIndex", offset: 0x1BA, ty: "uint16", note: "must match the knife type for the chosen subclass" },
        ],
        convars: &[],
        hooks: &[
            VerifiedHook { function: "UpdateSubclass",   module: "client.dll", signature: "48 8B 41 10 48 8B D9 8B 50 30", action: "Re-bind subclass-data ptr at weapon+0x388 after writing m_nSubclassID." },
            VerifiedHook { function: "AnimGraphRebuild", module: "client.dll", signature: "AnimGraphRebuild",              action: "Mode = 2: destroy CNmGraphInstance and re-bind from new vdata's animgraph." },
            VerifiedHook { function: "SetMeshGroupMask", module: "client.dll", signature: "SetMeshGroupMask",              action: "Refresh visible mesh after subclass change." },
        ],
    },
];

// ----------------------------------------------------------------------
// Renderers
// ----------------------------------------------------------------------

pub fn render_json(build_number: Option<u32>) -> String {
    let working: Vec<&VerifiedFeature> = FEATURES.iter().filter(|f| f.status == "working").collect();
    let features: Vec<_> = working
        .iter()
        .map(|f| {
            json!({
                "name":    f.name,
                "summary": f.summary,
                "fields":  f.fields.iter().map(|fld| json!({
                    "class":  fld.class,
                    "field":  fld.field,
                    "offset": format!("0x{:X}", fld.offset),
                    "type":   fld.ty,
                    "note":   fld.note,
                })).collect::<Vec<_>>(),
                "convars": f.convars.iter().map(|c| json!({
                    "name":             c.name,
                    "strip_flags":      format!("0x{:X}", c.strip_flags),
                    "write_both_slots": c.write_both_slots,
                    "value":            c.value,
                })).collect::<Vec<_>>(),
                "hooks":   f.hooks.iter().map(|h| json!({
                    "function":  h.function,
                    "module":    h.module,
                    "signature": h.signature,
                    "action":    h.action,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();

    let doc = json!({
        "cs2_build":      build_number,
        "feature_count":  working.len(),
        "features":       features,
    });

    serde_json::to_string_pretty(&doc).unwrap_or_else(|_| String::from("{}"))
}

pub fn render_md(build_number: Option<u32>) -> String {
    let mut out = String::new();
    out.push_str("# Verified features\n\n");
    out.push_str(
        "Working CS2 cheat features. Each entry lists the schema fields, \
         convars and hooks needed to wire it up. Addresses come from the \
         neighbouring `offsets.*` and `signatures.*` files.\n\n",
    );
    if let Some(b) = build_number {
        out.push_str(&format!("Build: `{}`\n\n", b));
    }

    let working: Vec<&VerifiedFeature> = FEATURES.iter().filter(|f| f.status == "working").collect();
    out.push_str(&format!("Features: {}\n\n---\n\n", working.len()));

    for f in working {
        out.push_str(&format!("## {}\n\n", f.name));
        out.push_str(f.summary);
        out.push_str("\n\n");

        if !f.fields.is_empty() {
            out.push_str("### Fields\n\n");
            out.push_str("| Class | Field | Offset | Type | Note |\n");
            out.push_str("|---|---|---|---|---|\n");
            for fld in f.fields.iter() {
                out.push_str(&format!(
                    "| `{}` | `{}` | `0x{:X}` | `{}` | {} |\n",
                    fld.class, fld.field, fld.offset, fld.ty, fld.note,
                ));
            }
            out.push('\n');
        }

        if !f.convars.is_empty() {
            out.push_str("### ConVars\n\n");
            out.push_str("| Name | Strip Flags | Write Both Slots | Value |\n");
            out.push_str("|---|---|---|---|\n");
            for c in f.convars.iter() {
                out.push_str(&format!(
                    "| `{}` | `0x{:X}` | {} | {} |\n",
                    c.name, c.strip_flags, c.write_both_slots, c.value,
                ));
            }
            out.push('\n');
        }

        if !f.hooks.is_empty() {
            out.push_str("### Hooks\n\n");
            out.push_str("| Function | Module | Signature | Action |\n");
            out.push_str("|---|---|---|---|\n");
            for h in f.hooks.iter() {
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}` | {} |\n",
                    h.function, h.module, h.signature, h.action,
                ));
            }
            out.push('\n');
        }

        out.push_str("---\n\n");
    }

    out
}

pub fn render_hpp(build_number: Option<u32>) -> String {
    let mut out = String::new();
    out.push_str("// =====================================================================\n");
    out.push_str("// verified_features.hpp â€” hand-curated working CS2 cheat features\n");
    out.push_str("// =====================================================================\n");
    out.push_str("//\n");
    out.push_str("// Each entry is a known-good offset / hook / ConVar trick that has been\n");
    out.push_str("// verified live against the current CS2 build. Constants are static\n");
    out.push_str("// constexpr so they can be used at compile time.\n");
    out.push_str("//\n");
    if let Some(b) = build_number {
        out.push_str(&format!("// CS2_BUILD: {}\n", b));
    }
    out.push_str("// =====================================================================\n\n");
    out.push_str("#pragma once\n\n");
    out.push_str("#include <cstdint>\n\n");
    out.push_str("namespace cs2::verified\n{\n");

    for f in FEATURES.iter() {
        let ns = sanitize_ident(f.name);
        out.push_str(&format!("    // ----- {} [{}] -----\n", f.name, f.status));
        out.push_str(&format!("    namespace {}\n    {{\n", ns));
        out.push_str(&format!("        constexpr const char* status = \"{}\";\n", f.status.replace('\\', "\\\\").replace('"', "\\\"")));
        for fld in f.fields.iter() {
            let cls = sanitize_ident(fld.class);
            let fname = sanitize_ident(fld.field);
            out.push_str(&format!(
                "        constexpr std::ptrdiff_t {}__{} = 0x{:X}; // {} â€” {}\n",
                cls, fname, fld.offset, fld.ty, fld.note,
            ));
        }
        for c in f.convars.iter() {
            let cn = sanitize_ident(c.name);
            out.push_str(&format!(
                "        constexpr uint32_t convar_{}__strip_flags = 0x{:X};\n",
                cn, c.strip_flags,
            ));
            out.push_str(&format!(
                "        constexpr bool     convar_{}__write_both  = {};\n",
                cn, c.write_both_slots,
            ));
        }
        out.push_str("    }\n\n");
    }

    out.push_str("} // namespace cs2::verified\n");
    out
}

fn sanitize_ident(input: &str) -> String {
    let mut s = String::with_capacity(input.len());
    for c in input.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            s.push(c);
        } else {
            s.push('_');
        }
    }
    if s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        s.insert(0, '_');
    }
    s
}