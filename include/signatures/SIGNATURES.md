# CS2 Signatures

_This file is regenerated on every successful run of `cs2-sdk`._

**582/649 signatures resolved across 17 module(s).**

## `animationsystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `FrameUpdate` | `void __fastcall FrameUpdate(__int64 a1)` | `raw` | `0x7FFA7F74B480` | `0x8B480` | `48 89 4C 24 08 55 53 56 57 41 54 41 55 41 56 41 57 48 8D AC 24 C8 EB FF FF B8 38 15 00 00` |
| `ShouldUpdateSequences` | `__int64 __fastcall ShouldUpdateSequences(__int64 a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFA7F80EFF0` | `0x14EFF0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 20 49 8B 40 48` |

## `client.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `ActionTrackingServices` | `__int64 __fastcall ActionTrackingServices(int a1, _QWORD *a2)` | `stringref` | `0x7FFA70A47F50` | `0x7E7F50` | `"CCSPlayerController_ActionTrackingServices"` |
| `AddListener` | `__int64 __fastcall AddListener(__int64 a1, __int64 a2, const char *a3, unsigned __int8 a4)` | `raw` | `0x7FFA70B9C8A0` | `0x93C8A0` | `48 89 5C 24 10 48 89 6C 24 18 56 57 41 56 48 83 EC 50 41 0F B6 E9 48 8D 99 E0 00 00 00 49 8B F0` |
| `AddNametagEntity` | `char __fastcall AddNametagEntity(__int64 a1, __int64 a2)` | `raw` | `0x7FFA709ECAA0` | `0x78CAA0` | `40 55 53 56 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 48 8B DA` |
| `AddStattrakEntity` | `void __fastcall AddStattrakEntity(__int64 a1, unsigned int a2)` | `raw` | `0x7FFA70CAF540` | `0xA4F540` | `48 8B C4 48 89 58 08 48 89 70 10 57 48 83 EC 50 33 F6 8B FA 48 8B D1` |
| `AddToTail` | `__int64 __fastcall AddToTail(int *a1, __int64 a2)` | `raw` | `0x7FFA709EB680` | `0x78B6D2` | `41 B9 88 02 00 00 8B 57 14 81 E2 FF FF FF 3F 8D 71 01 44 8B C6 FF 15` |
| `AnimGraphRebuild` | `__int64 __fastcall AnimGraphRebuild(__int64 a1, char a2)` | `raw` | `0x7FFA70B11600` | `0x8B1600` | `40 55 56 48 83 EC 28 4C 89 74 24 58 48 8B F1 80 FA FF 75 04 0F B6 51 18` |
| `ApplyEconCustomization` | `__int64 __fastcall ApplyEconCustomization(__int64 a1, char a2)` | `raw` | `0x7FFA70A0A590` | `0x7AA590` | `48 89 5C 24 ? 57 48 83 EC ? 8B FA 48 8B D9 E8 ? ? ? ? 48 8B CB E8 ? ? ? ? 48 85 C0 74` |
| `AutowallInit` | `__int64 __fastcall AutowallInit(__int64 a1)` | `raw` | `0x7FFA70B447C0` | `0x8E47C0` | `40 53 48 83 EC ? 48 8B D9 48 81 C1 ? ? ? ? E8 ? ? ? ?` |
| `AutowallTraceData` | `char __fastcall AutowallTraceData(_QWORD *a1, int *a2, int a3, int a4, _BYTE *a5, int a6)` | `raw` | `0x7FFA70BF12A0` | `0x9912A0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B 09` |
| `AutowallTracePos` | `char __fastcall AutowallTracePos(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70A6A1F0` | `0x80A1F0` | `40 55 56 41 54 41 55 41 57 48 8B EC` |
| `BuildBoneMergeWork` | `char __fastcall BuildBoneMergeWork(__int64 a1, _QWORD *a2, char a3)` | `raw` | `0x7FFA70BA22F0` | `0x9422F0` | `40 55 56 57 41 54 41 55 41 56 41 57 48 83 EC 50 48 8D 6C 24 50 80 A1 06 01 00 00 FB 4C 8B F9 80` |
| `BuildTemplateMaterialFromFile` | `CKeyValues_Data *__fastcall BuildTemplateMaterialFromFile(__int64 a1, const char *a2)` | `raw` | `0x7FFA7162CAF0` | `0x13CCAF0` | `48 89 54 24 10 55 53 41 55 41 57 48 8D AC 24 18 F9 FF FF 48 81 EC E8 07 00 00 4C 8B FA 48 85 D2` |
| `BulkRegenIterator` | `__int64 __fastcall BulkRegenIterator(char a1)` | `raw` | `0x7FFA709EFFA1` | `0x78FFA1` | `57 48 83 EC 40 0F B6 F9 E8 ? ? ? ? 48 85 C0 0F 84` |
| `BulletServices` | `void *__fastcall BulletServices(__int64 a1)` | `stringref` | `0x7FFA70A76580` | `0x816580` | `"CCSPlayer_BulletServices"` |
| `CAttributeStringFill` | `__int64 __fastcall CAttributeStringFill(__int64 a1, __int64 a2)` | `rel32` | `0x7FFA71113410` | `0xEB3410` | `E8 ? ? ? ? 41 83 CF 08` |
| `CAttributeStringInit` | `_QWORD *__fastcall CAttributeStringInit(_QWORD *a1, __int64 a2, char a3)` | `rel32` | `0x7FFA70858A20` | `0x5F8A20` | `E8 ? ? ? ? 48 8D 05 ? ? ? ? 48 89 7D ? 48 89 45 ? 49 8D 4F` |
| `CBodyComponent` | `__int64 CBodyComponent()` | `stringref` | `0x7FFA7041C520` | `0x1BC520` | `"CBodyComponent"` |
| `CBodyComponentSkeletonInstance` | `__int64 (__fastcall ***CBodyComponentSkeletonInstance())()` | `stringref` | `0x7FFA70423400` | `0x1C3400` | `"CBodyComponentSkeletonInstance"` |
| `CBufferStringInit` | `char __fastcall CBufferStringInit(__int64 a1, const char *a2)` | `raw` | `0x7FFA71A51EA0` | `0x17F1EA0` | `48 89 5C 24 ? 57 48 83 EC ? 8B 41 ? 48 8D 79` |
| `CCSGOHudVote_OnVoteResult` | `void __fastcall CCSGOHudVote_OnVoteResult(__int64 a1, int a2, const char *a3, int a4, __int64 a5)` | `raw` | `0x7FFA71077710` | `0xE17710` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 57 41 56 41 57 48 81 EC 90 01 00 00 65 48 8B 04 25 58 00 00 00 49 8B E8 44 8B 15 ? ? ? ? 8B FA` |
| `CCSGO_HudChat_OnSayText2` | `void __fastcall CCSGO_HudChat_OnSayText2(int a1, __int64 a2)` | `raw` | `0x7FFA71333800` | `0x10D3800` | `48 89 5C 24 08 55 56 57 41 54 41 55 41 56 41 57 48 8D AC 24 70 F3 FF FF 48 81 EC 90 0D 00 00 81 A5 DC 0C 00 00 FF FF 0F FF 33 F6 8B 5A 6C 48 8B` |
| `CCSGameRules` | `_QWORD *CCSGameRules()` | `stringref` | `0x7FFA702DE160` | `0x7E160` | `"CCSGameRules"` |
| `CCSGameRulesProxy` | `__int64 CCSGameRulesProxy()` | `stringref` | `0x7FFA7094A5B0` | `0x6EA5B0` | `"CCSGameRulesProxy"` |
| `CCSPlayerController` | `__int64 __fastcall CCSPlayerController(int a1, _QWORD *a2)` | `stringref` | `0x7FFA70A47F50` | `0x7E7F50` | `"CCSPlayerController"` |
| `CCSPlayerPawn` | `__int64 CCSPlayerPawn()` | `stringref` | `0x7FFA70E14370` | `0xBB4370` | `"CCSPlayerPawn"` |
| `CCSPlayer_MovementServices_ValidateVelocity` | `void __fastcall CCSPlayer_MovementServices_ValidateVelocity(__int64 movementServices)` | `stringref` | `0x7FFA70AAA1D0` | `0x84A1D0` | `"CCSPlayer_MovementServices(%s):  %d/%s Got a NaN velocity on %s\n"` |
| `CCSWeaponBase` | `__int64 CCSWeaponBase()` | `stringref` | `0x7FFA709E0DF0` | `0x780DF0` | `"CCSWeaponBase"` |
| `CCSWeaponBaseGun` | `__int64 CCSWeaponBaseGun()` | `stringref` | `0x7FFA709E0E90` | `0x780E90` | `"CCSWeaponBaseGun"` |
| `CCSWeaponBaseVData` | `const char *CCSWeaponBaseVData()` | `stringref` | `0x7FFA709BB800` | `0x75B800` | `"CCSWeaponBaseVData"` |
| `CCollisionProperty` | `__int64 __fastcall CCollisionProperty(int a1, __int64 a2, __int64 a3, __int64 a4)` | `stringref` | `0x7FFA70541350` | `0x2E1350` | `"CCollisionProperty"` |
| `CDecoyProjectile` | `__int64 CDecoyProjectile()` | `stringref` | `0x7FFA709AF770` | `0x74F770` | `"CDecoyProjectile"` |
| `CEconItemCreateInstance` | `uintptr_t __cdecl CEconItemCreateInstance()` | `raw` | `0x7FFA712663B0` | `0x10063B0` | `48 83 EC 28 B9 48 00 00 00 E8` |
| `CFlashbangProjectile` | `__int64 CFlashbangProjectile()` | `stringref` | `0x7FFA7124F030` | `0xFEF030` | `"CFlashbangProjectile"` |
| `CFogController` | `__int64 CFogController()` | `stringref` | `0x7FFA704DF390` | `0x27F390` | `"CFogController"` |
| `CGameSceneNode` | `__int64 __fastcall CGameSceneNode(int a1, __int64 a2)` | `stringref` | `0x7FFA70403EF0` | `0x1A3EF0` | `"CGameSceneNode"` |
| `CGlowProperty` | `__int64 __fastcall CGlowProperty(int a1, __int64 a2, __int64 a3, __int64 a4)` | `stringref` | `0x7FFA70541560` | `0x2E1560` | `"CGlowProperty"` |
| `CHEGrenadeProjectile` | `__int64 CHEGrenadeProjectile()` | `stringref` | `0x7FFA7124F0D0` | `0xFEF0D0` | `"CHEGrenadeProjectile"` |
| `CLegacyGameUI_Initialize` | `__int64 __fastcall CLegacyGameUI_Initialize(__int64 thisptr)` | `stringref` | `0x7FFA70F09D30` | `0xCA9D30` | `"CLegacyGameUI::Initialize() failed to get necessary interfaces\n"` |
| `CMolotovProjectile` | `__int64 CMolotovProjectile()` | `stringref` | `0x7FFA709AF950` | `0x74F950` | `"CMolotovProjectile"` |
| `CPostProcessingVolume` | `__int64 CPostProcessingVolume()` | `stringref` | `0x7FFA70504120` | `0x2A4120` | `"CPostProcessingVolume"` |
| `CPrediction_Update` | `__int64 __fastcall CPrediction_Update(__int64 thisptr, int reason)` | `raw` | `0x7FFA70DB1210` | `0xB51210` | `48 8B C4 89 50 ? 48 89 48 ? 55 53 57` |
| `CSBaseGunFireData` | `void __fastcall CSBaseGunFireData(__int64 a1)` | `raw` | `0x7FFA717581E0` | `0x14F81E0` | `48 8B C4 55 53 56 57 41 54 41 55 41 56 41 57 48 8D 68 A8 48 81 EC ? ? ? ? 4C 8B 69` |
| `CSkeletonInstance` | `__int64 __fastcall CSkeletonInstance(int a1, __int64 a2)` | `stringref` | `0x7FFA70403FF0` | `0x1A3FF0` | `"CSkeletonInstance"` |
| `CSmokeGrenadeProjectile` | `__int64 CSmokeGrenadeProjectile()` | `stringref` | `0x7FFA709AF9F0` | `0x74F9F0` | `"CSmokeGrenadeProjectile"` |
| `CTonemapController2` | `__int64 CTonemapController2()` | `stringref` | `0x7FFA704B8050` | `0x258050` | `"CTonemapController2"` |
| `C_AttributeContainer` | `__int64 __fastcall C_AttributeContainer(int a1, _QWORD *a2)` | `stringref` | `0x7FFA70E7D2D0` | `0xC1D2D0` | `"C_AttributeContainer"` |
| `C_BaseEntity` | `__int64 (__fastcall *C_BaseEntity())()` | `stringref` | `0x7FFA702AE260` | `0x4E260` | `"C_BaseEntity"` |
| `C_BaseEntity_CheckPredictionForceReLatch` | `__int64 __fastcall C_BaseEntity_CheckPredictionForceReLatch(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70DAAF10` | `0xB4AF10` | `48 8B C4 48 89 50 10 53 55 56 48 81 EC 00 01 00 00 0F 29 78 98 48 8B F2 8B 91 04 01 00 00` |
| `C_BaseEntity_ProcessInterpolatedList` | `__int64 __fastcall C_BaseEntity_ProcessInterpolatedList(__int64 a1, unsigned int a2, int a3, unsigned int a4)` | `raw` | `0x7FFA70CCEB80` | `0xA6EB80` | `4C 8B DC 49 89 5B 10 49 89 6B 18 49 89 73 20 57 41 54 41 57 48 83 EC 60 49 C7 43 B0 E1 07 00 00` |
| `C_BaseEntity_RestoreData` | `void __fastcall C_BaseEntity_RestoreData(__int64 a1, const char *a2, unsigned int a3, int a4)` | `raw` | `0x7FFA70CD43C0` | `0xA743C0` | `40 55 53 56 41 54 41 57 48 8D AC 24 20 FF FF FF 48 81 EC E0 01 00 00 48 8B D9 45 8B E1 48 8B 89` |
| `C_BaseEntity_SaveData` | `void __fastcall C_BaseEntity_SaveData(_QWORD *a1, const char *a2, __int64 a3, int a4, int a5, unsigned int a6, __int64 a7)` | `raw` | `0x7FFA70CD45D0` | `0xA745D0` | `48 8B C4 55 56 57 41 56 41 57 48 8D A8 E8 FD FF FF 48 81 EC F0 02 00 00 48 83 B9 A0 05 00 00 00` |
| `C_BaseEntity_StartParticleSystem` | `` | `raw` | `0x7FFA71006D50` | `0xDA6D50` | `48 89 5C 24 08 55 48 8B EC 48 83 EC 40 E8 ? ? ? ? 48 8D 05 ? ? ? ? 33 DB 48 8D 15` |
| `C_BaseModelEntity` | `__int64 __fastcall C_BaseModelEntity(int a1, _QWORD *a2)` | `stringref` | `0x7FFA703B8A50` | `0x158A50` | `"C_BaseModelEntity"` |
| `C_BasePlayerPawn` | `__int64 (__fastcall *C_BasePlayerPawn())()` | `stringref` | `0x7FFA702CDA20` | `0x6DA20` | `"C_BasePlayerPawn"` |
| `C_BasePlayerPawn_PrePhysicsSimulate` | `bool __fastcall C_BasePlayerPawn_PrePhysicsSimulate(__int64 pawn)` | `stringref` | `0x7FFA70B31FA0` | `0x8D1FA0` | `"C_BasePlayerPawn::PrePhysicsSimulate"` |
| `C_C4` | `__int64 (__fastcall *C_C4())()` | `stringref` | `0x7FFA702FA420` | `0x9A420` | `"C_C4"` |
| `C_CSPlayerPawn` | `__int64 __fastcall C_CSPlayerPawn(int a1, _QWORD *a2)` | `stringref` | `0x7FFA709235A0` | `0x6C35A0` | `"C_CSPlayerPawn"` |
| `C_CSPlayerPawnBase` | `__int64 *C_CSPlayerPawnBase()` | `stringref` | `0x7FFA70E3A760` | `0xBDA760` | `"C_CSPlayerPawnBase"` |
| `C_CSWeaponBase` | `_QWORD *__fastcall C_CSWeaponBase(int a1, _QWORD *a2)` | `stringref` | `0x7FFA709A3880` | `0x743880` | `"C_CSWeaponBase"` |
| `C_CSWeaponBase_GetEconWpnData` | `__int64 __fastcall C_CSWeaponBase_GetEconWpnData(__int64 a1)` | `raw` | `0x7FFA709F6C10` | `0x796C10` | `40 53 48 83 EC 40 48 8B D9 E8 ? ? ? ? 48 8B C8 E8 ? ? ? ? 48 85 C0 75 ? 48 8B 43 10` |
| `C_DispatchEffect` | `__int64 __fastcall C_DispatchEffect(const char *name, __int64 data)` | `stringref` | `0x7FFA70D30B80` | `0xAD0B80` | `"DispatchEffect: effect "%s" not found on client\n"` |
| `C_EconEntity_BuildLegacyGloveSkinMaterial` | `void __fastcall C_EconEntity_BuildLegacyGloveSkinMaterial(int *a1)` | `stringref` | `0x7FFA70E24A00` | `0xBC4A00` | `"MapPlayerPreview gloves"` |
| `C_EconEntity_BuildLegacyWeaponSkinMaterial` | `void __fastcall C_EconEntity_BuildLegacyWeaponSkinMaterial(__int64 a1, char a2)` | `stringref` | `0x7FFA709EDCD0` | `0x78DCD0` | `"workshop preview weapon"` |
| `C_EconEntity_BuildModernWeaponSkinMaterial` | `void __fastcall C_EconEntity_BuildModernWeaponSkinMaterial(__int64 a1, _QWORD *a2, __int64 a3, int a4, char a5, char a6, __int64 a7)` | `raw` | `0x7FFA70FE88C0` | `0xD888C0` | `48 85 C9 0F 84 ? ? 00 00 48 8B C4 48 89 50 10 48 89 48 08 55 41 54 41 56 41 57 48 8D A8 B8 FA` |
| `C_EconEntity_BuildNametagOverlayMaterial` | `char __fastcall C_EconEntity_BuildNametagOverlayMaterial(__int64 a1, __int64 a2)` | `stringref` | `0x7FFA709ECAA0` | `0x78CAA0` | `"low-res nametag"` |
| `C_EconItemView` | `_QWORD *__fastcall C_EconItemView(int a1, _QWORD *a2)` | `stringref` | `0x7FFA7096C760` | `0x70C760` | `"C_EconItemView"` |
| `C_EconWearable_OnNewCustomMaterials` | `__int64 __fastcall C_EconWearable_OnNewCustomMaterials(__int64 a1, char a2)` | `stringref` | `0x7FFA71329070` | `0x10C9070` | `"Invalid EconItemView -- Can't create custom materials for wearable, debug this.\n"` |
| `C_GameRules_ctor` | `__int64 __fastcall C_GameRules_ctor(__int64 thisptr)` | `stringref` | `0x7FFA70D67390` | `0xB07390` | `"%s:  CGameRules::CGameRules constructed\n"` |
| `C_Hostage` | `__int64 (__fastcall *C_Hostage())()` | `stringref` | `0x7FFA703475E0` | `0xE75E0` | `"C_Hostage"` |
| `C_Inferno` | `__int64 (__fastcall *C_Inferno())()` | `stringref` | `0x7FFA70357790` | `0xF7790` | `"C_Inferno"` |
| `C_PlantedC4_ClientThink` | `_DWORD *__fastcall C_PlantedC4_ClientThink(__int64 plantedC4)` | `stringref` | `0x7FFA70E6CFA0` | `0xC0CFA0` | `"C4.ExplodeTriggerTrip"` |
| `C_SmokeGrenadeProjectile` | `__int64 (__fastcall *C_SmokeGrenadeProjectile())()` | `stringref` | `0x7FFA702F5A10` | `0x95A10` | `"C_SmokeGrenadeProjectile"` |
| `CacheParticleEffect` | `` | `raw` | `0x7FFA70467F80` | `0x207F80` | `4C 8B DC 53 48 81 EC ? ? ? ? F2 0F 10 05` |
| `CalcViewmodelTransform_v2` | `__int64 __fastcall CalcViewmodelTransform_v2(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70A03FF0` | `0x7A3FF0` | `48 89 5C 24 20 55 56 57 41 54 41 55 41 56 41 57 48 8D 6C 24 80 48 81 EC 80 01 00 00 48 8B FA` |
| `CalcViewmodelView` | `__int64 __fastcall CalcViewmodelView(__int64 a1, __int64 a2, _DWORD *a3)` | `raw` | `0x7FFA70ECF6C0` | `0xC6F6C0` | `40 53 48 83 EC 60 48 8B 41 08 49 8B D8 8B 48 30 48 C1 E9 0C F6 C1 01 0F 85 48 01 00 00 41 B8 07` |
| `CalculateInterpolation` | `int *__fastcall CalculateInterpolation(__int64 a1, int *a2)` | `rel32` | `0x7FFA71737F10` | `0x14D7F10` | `E8 ? ? ? ? 8B 45 ? 3B 45 60 75 04 32 D2 EB 09 BA 01 00 00 00 41 0F 4C D5 C0 EA 07 84 D2 0F 85 87` |
| `CalculateWorldSpaceBones` | `void __fastcall CalculateWorldSpaceBones(__int64 a1, unsigned int a2)` | `raw` | `0x7FFA70C6DEC0` | `0xA0DEC0` | `48 89 4C 24 ? 55 53 56 57 41 54 41 55 41 56 41 57 B8 ? ? ? ? E8 ? ? ? ? 48 2B E0 48 8D 6C 24 ? 48 8B 81` |
| `Caller` | `__int64 __fastcall Caller(__int64 a1, const char *a2, __int64 a3, __int64 a4)` | `stringref` | `0x7FFA7162B974` | `0x13CB974` | `"CCompositeMaterialManager::AddNewPanoramaPanelRenderRequest"` |
| `CameraServices` | `__int64 CameraServices()` | `stringref` | `0x7FFA70A72690` | `0x812690` | `"CCSPlayer_CameraServices"` |
| `ChangeModel` | `__int64 __fastcall ChangeModel(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70B3DAA0` | `0x8DDAA0` | `40 53 48 83 EC ? 48 8B D9 4C 8B C2 48 8B 0D ? ? ? ? 48 8D 54 24` |
| `CheckJumpButton` | `void __fastcall CheckJumpButton(__int64 a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFA70D32260` | `0xAD2260` | `4C 89 44 24 18 55 56 41 56 48 8D AC 24 70 EC FF FF B8 90 14 00 00` |
| `ClearHUDWeaponIcon` | `__int64 __fastcall ClearHUDWeaponIcon(__int64 a1, int a2, __int64 a3)` | `rel32` | `0x7FFA710517C0` | `0xDF17C0` | `E8 ? ? ? ? 8B F8 C6 84 24 ? ? ? ? ?` |
| `Client` | `bool __fastcall Client(_QWORD *a1, __int64 a2, __int64 a3, __int64 a4, _BYTE *a5, __int64 a6)` | `raw` | `0x7FFA70BF1380` | `0x991380` | `48 89 5C 24 20 48 89 4C 24 08 55 57 41 54 41 55 41 56 48 8D AC 24 10 E0 FF FF B8 F0 20 00 00` |
| `ComputeRandomSeed` | `__int64 __fastcall ComputeRandomSeed(__int64 a1, __int64 a2, int a3)` | `raw` | `0x7FFA70EE1A70` | `0xC81A70` | `48 89 5C 24 ? 57 48 81 EC ? ? ? ? ? ? ? ? 48 8D 8C 24` |
| `ConCommand_firstperson` | `__int64 ConCommand_firstperson()` | `raw` | `0x7FFA70D2D100` | `0xACD100` | `48 83 EC 28 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 08 03 00 00 83 7C 24 ? 00 75 ? 48 8B 05 ? ? ? ? C6 80 29 02 00 00 00 C7 80 A8 06 00 00 00` |
| `ConCommand_thirdperson` | `__int64 ConCommand_thirdperson()` | `raw` | `0x7FFA70D2D1E0` | `0xACD1E0` | `48 83 EC 38 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 08 03 00 00 83 7C 24 ? 00 0F 85 ? ? ? ? 4C 8B 05 ? ? ? ? 41 8B 80 50 0B 00 00` |
| `Constructor` | `` | `raw` | `0x7FFA71082DA0` | `0xE22DA0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 30 48 8B F1 48 8B FA B9 ? ? ? ? E8 ? ? ? ? 48 8B D8 48 85 C0 74` |
| `Context` | `void __fastcall Context(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70C3E950` | `0x9DE950` | `48 8B C4 48 81 EC C8 00 00 00 48 89 58 10 48 89 68 18 48 8B EA 48 89 70 20 48 8B F1 48 89 78 F8` |
| `ConvarGet` | `void __fastcall ConvarGet(__int64 a1, unsigned int *a2)` | `raw` | `0x7FFA70B21FC2` | `0x8C1FC2` | `8B D0 48 8D 0D ? ? ? ? E8 ? ? ? ? 0F 10 45 ? 83 F0 74` |
| `CreateBaseTypeCache` | `` | `raw` | `0x7FFA71780F40` | `0x1520F40` | `40 53 48 83 EC ? 4C 8B 49 ? 44 8B D2` |
| `CreateEconItem` | `` | `raw` | `0x7FFA712663B0` | `0x10063B0` | `48 83 EC 28 B9 48 00 00 00 E8 ? ? ? ? 48 85` |
| `CreateEntityByClassName` | `__int64 __fastcall CreateEntityByClassName(__int64 a1, int a2, __int64 a3, __int64 a4)` | `raw` | `0x7FFA71874186` | `0x1614186` | `4C 8D 05 ? ? ? ? 4C 8B CF BA 03 00 00 00 FF 15 ? ? ? ? EB ? 0F B7 C8 48` |
| `CreateInterface` | `__int64 __fastcall CreateInterface(__int64 a1, _DWORD *a2)` | `raw` | `0x7FFA71AA4C60` | `0x1844C60` | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08` |
| `CreateNewSubtickMoveStep` | `__int64 __fastcall CreateNewSubtickMoveStep(__int64 a1)` | `rel32` | `0x7FFA70712140` | `0x4B2140` | `E8 ? ? ? ? 48 8B D0 48 8B CE E8 ? ? ? ? 48 8B C8` |
| `CreateParticleEffect` | `__int64 __fastcall CreateParticleEffect(int a1, int a2, int a3, __int64 a4, int a5)` | `raw` | `0x7FFA70BE9900` | `0x989900` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? F3 0F 10 1D ? ? ? ? 41 8B F8 8B DA 4C 8D 05` |
| `CreateSOSubclassEconItem` | `__int64 CreateSOSubclassEconItem()` | `raw` | `0x7FFA712663B0` | `0x10063B0` | `48 83 EC 28 B9 48 00 00 00 E8 ? ? ? ? 48 85` |
| `CreateTrace` | `` | `raw` | `0x7FFA70A67390` | `0x807390` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 56 41 57 48 83 EC ? ? ? ? ? 4D 8D 71` |
| `Ctrl` | `__int64 __fastcall Ctrl(__int64 a1)` | `raw` | `0x7FFA70B39BF0` | `0x8D9BF0` | `48 89 5C 24 ? 57 48 81 EC ? ? ? ? ? ? ? 48 8B F9 FF 90` |
| `DamageServices` | `__int64 __fastcall DamageServices(int a1, _QWORD *a2)` | `stringref` | `0x7FFA70A47F50` | `0x7E7F50` | `"CCSPlayerController_DamageServices"` |
| `DestroyParticle` | `void __fastcall DestroyParticle(__int64 a1, __int64 a2, unsigned __int8 a3, char a4)` | `raw` | `0x7FFA70BA8C90` | `0x948C90` | `83 FA ? 0F 84 ? ? ? ? 41 54` |
| `DispatchEffect` | `__int64 __fastcall DispatchEffect(__int64 a1, __int64 a2)` | `raw` | `0x7FFA705BA930` | `0x35A930` | `48 89 5C 24 ? 57 48 83 EC ? 48 8B F9 48 8B DA 48 8D 4C 24` |
| `DispatchSpawn` | `__int64 __fastcall DispatchSpawn(_QWORD *a1, __int64 a2, __int64 a3, __int64 a4)` | `raw` | `0x7FFA71745BB0` | `0x14E5BB0` | `4C 8B DC 55 56 48 83 EC 78 49 8B 68 08 48 8B F1 48 85 ED 0F 84 72 01 00 00 49 89 5B 08 49 8D 4B` |
| `DispatchSpawn_caller` | `__int64 __fastcall DispatchSpawn_caller(_QWORD *a1, __int64 a2, __int64 a3, __int64 a4)` | `raw` | `0x7FFA71745BB0` | `0x14E5BB0` | `4C 8B DC 55 56 48 83 EC 78 49 8B 68 08 48 8B F1 48 85 ED 0F 84 72 01 00 00` |
| `DispatchUpdateOnRemove` | `` | `raw` | `0x7FFA71743650` | `0x14E3650` | `48 89 5C 24 10 48 89 74 24 18 48 89 7C 24 20 55 41 56 41 57 48 8B EC 48 83 EC 60 48 8D B9 80 00 00 00 45 33 FF 4D 8B F0` |
| `DrawCrosshair` | `bool __fastcall DrawCrosshair(_QWORD *a1)` | `raw` | `0x7FFA70A12800` | `0x7B2800` | `48 89 5C 24 08 57 48 83 EC 20 48 8B D9 E8 ? ? ? ? 48 85` |
| `DrawLegs` | `void __fastcall DrawLegs(__int64 *a1, __int64 *a2, __int64 a3, __int64 a4, __int64 a5)` | `raw` | `0x7FFA713603F0` | `0x11003F0` | `40 55 53 56 41 56 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? F2 0F 10 42` |
| `DrawOverHead` | `unsigned __int8 __fastcall DrawOverHead(__int64 a1, unsigned int a2)` | `raw` | `0x7FFA70CC9AA0` | `0xA69AA0` | `40 53 48 83 EC ? 48 8B D9 83 FA ? 75 ? 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 ? ? ? ? 8B 10` |
| `DrawScopeOverlay` | `__int64 __fastcall DrawScopeOverlay(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70ABF930` | `0x85F930` | `48 8B C4 53 57 48 83 EC ? 48 8B FA` |
| `DrawSmokeVertex` | `__int64 __fastcall DrawSmokeVertex(__int64 a1, __int64 a2, int a3, int a4, __int64 a5, __int64 a6)` | `raw` | `0x7FFA70EDEA30` | `0xC7EA30` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 56 41 57 48 83 EC ? 48 8B 9C 24 ? ? ? ? 4D 8B F8` |
| `DrawTeamIntro` | `` | `raw` | `0x7FFA70964FB0` | `0x704FB0` | `48 83 EC ? ? ? ? ? 44 38 89` |
| `E8` | `__int64 __fastcall E8(__int64 a1, __int64 a2)` | `rel32` | `0x7FFA70AAFB50` | `0x84FB50` | `E8 ? ? ? ? 4C 8B C0 48 8D 55 ? 48 8B CB E8 ? ? ? ? 48 8D 0D` |
| `EmitPanoramaSound` | `` | `raw` | `0x7FFA70DC7380` | `0xB67380` | `40 53 48 81 EC ? ? ? ? ? ? ? 48 8B 05` |
| `EquipItemInLoadout` | `char __fastcall EquipItemInLoadout(_QWORD *a1, unsigned int a2, int a3, unsigned __int64 a4)` | `raw` | `0x7FFA70A23D60` | `0x7C3D60` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 89 54 24 ? 57 41 54 41 55 41 56 41 57 48 83 EC ? 0F B7 FA` |
| `Event` | `__int64 __fastcall Event(__int64 a1, unsigned int a2, int a3)` | `raw` | `0x7FFA7070AF00` | `0x4AAF00` | `48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC 20 48 63 FA 41` |
| `FindHudElement` | `_QWORD **__fastcall FindHudElement(__int64 a1, unsigned __int8 a2)` | `raw` | `0x7FFA71025888` | `0xDC5888` | `48 8D 15 ? ? ? ? 45 33 C0 B9 ? ? ? ? FF 15 ? ? ? ? EB ? 48 8B 15` |
| `FindHudElement_panorama` | `__int64 __fastcall FindHudElement_panorama(const char *a1)` | `raw` | `0x7FFA71027860` | `0xDC7860` | `4C 8B DC 53 48 83 EC 50 48 8B 05` |
| `FindOrCreateByName` | `char __fastcall FindOrCreateByName(__int64 a1, __int64 a2, char *a3, __int64 a4)` | `stringref` | `0x7FFA712CA060` | `0x106A060` | `"Kit "[%s]" specified, but doesn't exist!! You're probably missing an entry in items_paintkits.txt or items_stickerkits.txt or need to run with -use_local_item_data\n"` |
| `FindSOCache` | `__int64 __fastcall FindSOCache(__int64 a1, int *a2, __int64 a3, __int64 a4)` | `raw` | `0x7FFA71A8E550` | `0x182E550` | `48 89 5C 24 08 57 48 83 EC 30 4C 8B 52 08 48 8B D9 8B 0A` |
| `FireBullets` | `void FireBullets(unsigned int a1, __int64 a2, __int64 a3, __int64 *a4, __int64 a5, int a6, int a7, ...)` | `raw` | `0x7FFA70EE1B20` | `0xC81B20` | `48 8B C4 4C 89 48 20 48 89 50 10 55 53 57 41 54 41 55 48 8D A8 58 FB FF FF 48 81 EC A0 05` |
| `FirstPersonLegs` | `void __fastcall FirstPersonLegs(__int64 *a1, __int64 *a2, __int64 a3, __int64 a4, __int64 a5)` | `raw` | `0x7FFA713603F0` | `0x11003F0` | `40 55 53 56 41 56 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? F2 0F 10 42` |
| `FlashOverlay` | `void __fastcall FlashOverlay(__int64 a1, int a2)` | `raw` | `0x7FFA7100EC90` | `0xDAEC90` | `85 D2 0F 88 ? ? ? ? 48 89 4C 24` |
| `ForceButtonsDown` | `void __fastcall ForceButtonsDown(_QWORD *a1, __int64 a2)` | `raw` | `0x7FFA70C32F30` | `0x9D2F30` | `40 53 57 41 56 48 81 EC ? ? ? ? 48 83 79` |
| `FrameStageNotify` | `` | `raw` | `0x7FFA70D356F0` | `0xAD56F0` | `48 89 5C 24 ? 48 89 6C 24 ? 57 48 83 EC ? 48 8B F9 33 ED` |
| `GetAttributeDefByName` | `` | `raw` | `0x7FFA712BC6B0` | `0x105C6B0` | `48 89 5C 24 10 48 89 6C 24 18 57 41 56 41 57 48 83 EC 60 48 8D 05` |
| `GetAttributeDefinitionByName` | `__int64 __fastcall GetAttributeDefinitionByName(__int64 a1, unsigned __int8 *a2)` | `raw` | `0x7FFA712BC6B0` | `0x105C6B0` | `48 89 5C 24 10 48 89 6C 24 18 57 41 56 41 57 48 83 EC 60 48 8D 05` |
| `GetBaseEntity` | `__int64 __fastcall GetBaseEntity(__int64 a1, int a2)` | `raw` | `0x7FFA70BC9EE0` | `0x969EE0` | `4C 8D 49 ? 81 FA` |
| `GetBonePositionByName` | `__int64 __fastcall GetBonePositionByName(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70B2AA10` | `0x8CAA10` | `40 53 48 83 EC ? 48 8B 89 ? ? ? ? 48 8B DA 48 8B 01 FF 50 ? 48 8B C8` |
| `GetCSInvMgr_call` | `` | `rel32` | `0x7FFA70A28040` | `0x7C8040` | `E8 ? ? ? ? 48 8B D8 8B F7` |
| `GetChatObject` | `__int64 GetChatObject()` | `rel32` | `0x7FFA71333650` | `0x10D3650` | `E8 ? ? ? ? 48 8B E8 48 85 C0 0F 84 ? ? ? ? 4C 8D 05` |
| `GetClientSystem` | `__int64 *GetClientSystem()` | `rel32` | `0x7FFA712A5A90` | `0x1045A90` | `E8 ? ? ? ? 48 8B C8 E8 ? ? ? ? 8B D8 85 C0 74 33` |
| `GetControllerCmd` | `__int64 __fastcall GetControllerCmd(__int64 a1, int a2)` | `raw` | `0x7FFA70B205C0` | `0x8C05C0` | `40 53 48 83 EC 20 8B DA E8 ? ? ? ? 4C` |
| `GetCustomPaintKitIndex` | `__int64 __fastcall GetCustomPaintKitIndex(__int64 *a1)` | `raw` | `0x7FFA71318B20` | `0x10B8B20` | `48 89 5C 24 ? 57 48 83 EC ? 8B 15 ? ? ? ? 48 8B F9 65 48 8B 04 25 ? ? ? ? B9 ? ? ? ? 48 8B 04 D0 8B 04 01 39 05 ? ? ? ? 0F 8F ? ? ? ? E8 ? ? ? ? 8B 58 ? 39 1D ? ? ? ? 74 ? E8 ? ? ? ? 48 8B 15 ? ? ? ? 48 8B C8 E8 ? ? ? ? 48 89 05 ? ? ? ? 89 1D ? ? ? ? EB ? 48 8B 05 ? ? ? ? 48 85 C0 74` |
| `GetEconItemSystem` | `__int64 GetEconItemSystem()` | `raw` | `0x7FFA705D9BF0` | `0x379BF0` | `48 83 EC 28 48 8B 05 ? ? ? ? 48 85 C0 0F 85 ? ? ? ? 48 89 5C 24` |
| `GetEntityByIndex` | `__int64 __fastcall GetEntityByIndex(__int64 a1, int a2)` | `raw` | `0x7FFA70BC9EE0` | `0x969EE0` | `4C 8D 49 ? 81 FA` |
| `GetEntityHandle` | `__int64 __fastcall GetEntityHandle(__int64 a1)` | `raw` | `0x7FFA70BB1190` | `0x951190` | `48 85 C9 74 32 48 8B 49 10 48 85 C9 74 29 44 8B 41 10 BA` |
| `GetGameModeName` | `` | `raw` | `0x7FFA71138460` | `0xED8460` | `48 83 EC ? 48 8B 0D ? ? ? ? ? ? ? FF 90 ? ? ? ? 48 85 C0 74 ? 48 8B 0D ? ? ? ? ? ? ? 4C 8B 42` |
| `GetGlowColor` | `void __fastcall GetGlowColor(__int64 a1, float *a2)` | `raw` | `0x7FFA70D6E1C0` | `0xB0E1C0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B F2 48 8B F9 48 8B 54 24` |
| `GetHitGroup` | `__int64 __fastcall GetHitGroup(__int64 a1)` | `raw` | `0x7FFA70C7A9E0` | `0xA1A9E0` | `40 53 48 83 EC 20 48 83 79 10 00 48 8B D9 74 16 E8 ?? ?? ?? ?? 84 C0 75 0D 48 8B 43 10 8B 40 38` |
| `GetInaccuracy` | `` | `raw` | `0x7FFA709F7620` | `0x797620` | `48 89 5C 24 ? 55 56 57 48 81 EC ? ? ? ? 44` |
| `GetInstance` | `__int64 GetInstance()` | `raw` | `0x7FFA70D6E2D0` | `0xB0E2D0` | `48 8B 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 8B 41 38 C3` |
| `GetInventoryManager` | `__int64 *GetInventoryManager()` | `rel32` | `0x7FFA70A28040` | `0x7C8040` | `E8 ? ? ? ? 48 8B D3 48 8B C8 4C 8B 00 41 FF 90 00 02` |
| `GetItemInLoadout` | `__int64 *__fastcall GetItemInLoadout(__int64 a1, unsigned int a2, unsigned int a3)` | `raw` | `0x7FFA70A25980` | `0x7C5980` | `40 55 48 83 EC ? 49 63 E8` |
| `GetItemViewByID` | `uintptr_t __fastcall GetItemViewByID(uintptr_t, uint64_t)` | `raw` | `0x7FFA712BF090` | `0x105F090` | `48 89 54 24 ? 53 48 83 EC ? 48 8B D9 48 85 D2 75 ? 33 C0 48 83 C4 ? 5B C3 48 83 C1 38 48 8D` |
| `GetLocalControllerById` | `__int64 __fastcall GetLocalControllerById(int a1)` | `raw` | `0x7FFA70B43950` | `0x8E3950` | `48 83 EC 28 83 F9 FF 75 ? 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 ? ? ? ? 8B 08 48 63 C1 4C 8D 05` |
| `GetLocalPawn` | `__int64 __fastcall GetLocalPawn(int a1)` | `raw` | `0x7FFA70B43950` | `0x8E3950` | `48 83 EC ? 83 F9 ? 75 ? 48 8B 0D ? ? ? ? 48 8D 54 24 ? ? ? ? FF 90 ? ? ? ? ? ? 48 63 C1 4C 8D 05` |
| `GetLocalPlayer_dispatcher` | `__int64 GetLocalPlayer_dispatcher()` | `raw` | `0x7FFA705D95C0` | `0x3795C0` | `48 83 EC 38 48 8B 05 ? ? ? ? 48 85 C0 0F 85 14 06 00 00 48 89 5C 24 40 B9 50 00 00 00 48 89` |
| `GetMatrixForView` | `double __fastcall GetMatrixForView(__int64 a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFA703CA010` | `0x16A010` | `40 53 48 83 EC 60 0F 29 74 24 50 0F 57 DB F3 0F 10 ? ? ? ? ? 49 8B D8` |
| `GetPlayerByIndex_export` | `__int64 GetPlayerByIndex_export()` | `raw` | `0x7FFA71166AE0` | `0xF06AE0` | `48 83 EC 28 4C 8D 05 ? ? ? ? 48 8D 15 ? ? ? ? 48 8D 0D ? ? ? ? E8 ? ? ? ? 4C 8D` |
| `GetPlayerInterp` | `float __fastcall GetPlayerInterp(__int64 a1)` | `raw` | `0x7FFA70B1BE20` | `0x8BBE20` | `40 53 48 83 EC ? 48 8B D9 48 8B 0D ? ? ? ? 48 83 C1` |
| `GetPlayerTeamName` | `` | `raw` | `0x7FFA7114A010` | `0xEEA010` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B CA 48 8B EA` |
| `GetRemovedAimpunch` | `__int64 GetRemovedAimpunch()` | `raw` | `0x7FFA70372D07` | `0x112D07` | `F2 0F 10 44 24 ? F2 0F 11 84 24 ? ? ? ? FF 15` |
| `GetServerName` | `` | `raw` | `0x7FFA7114EB60` | `0xEEEB60` | `40 53 48 83 EC ? 48 8B D9 48 8B 0D ? ? ? ? 48 85 C9 74 ? E8 ? ? ? ? 48 85 C0` |
| `GetSurfaceData` | `__int64 __fastcall GetSurfaceData(__int64 a1)` | `rel32` | `0x7FFA70BB5E00` | `0x955E00` | `E8 ? ? ? ? 80 78 18 00` |
| `GetTickBase` | `__int64 __fastcall GetTickBase(__int64 a1)` | `rel32` | `0x7FFA70B203C0` | `0x8C03C0` | `E8 ? ? ? ? EB ? 48 8B 05 ? ? ? ? 8B 40` |
| `GetTraceInfo` | `__int64 __fastcall GetTraceInfo(__int64 a1, __int64 a2, float a3, unsigned __int64 *a4)` | `raw` | `0x7FFA70A699C0` | `0x8099C0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B E9 0F 29 74 24 ? 48 8B CA` |
| `GetTransformsForHitboxList` | `char __fastcall GetTransformsForHitboxList(__int64 a1, __int64 a2, int *a3)` | `raw` | `0x7FFA70C7D460` | `0xA1D460` | `48 89 5C 24 18 55 56 57 41 55 41 57 48 81 EC A0 00 00 00 49 63 28 4D 8B F8 48 8B FA 48 8B D9 85` |
| `GetUserCmdManager` | `__int64 __fastcall GetUserCmdManager(__int64 a1)` | `raw` | `0x7FFA70B20650` | `0x8C0650` | `41 56 41 57 48 83 EC ? 48 8D 54 24` |
| `GetViewAngles` | `__int64 *__fastcall GetViewAngles(__int64 a1, int a2)` | `raw` | `0x7FFA70D38AF0` | `0xAD8AF0` | `4C 8B C1 85 D2 74 08 48 8D 05 ? ? ? ? C3` |
| `GetWeaponInAccuracyRecoveryTime` | `__m128 __fastcall GetWeaponInAccuracyRecoveryTime(__int64 a1)` | `rel32` | `0x7FFA709F8090` | `0x798090` | `E8 ? ? ? ? F3 0F 10 B7 ? ? ? ? F3 0F 5E F8` |
| `HandleBulletPenetration` | `char __fastcall HandleBulletPenetration(__int64 a1, float *a2, __int64 a3, int a4, __int64 a5)` | `raw` | `0x7FFA70A83BD0` | `0x823BD0` | `48 8B C4 44 89 48 ? 48 89 50 ? 48 89 48 ? 55` |
| `HandleEntityList` | `__int64 __fastcall HandleEntityList(__int64 a1, __int64 a2, __int64 a3, __int64 a4, __int64 a5, int a6, int a7)` | `rel32` | `0x7FFA70423AC0` | `0x1C3AC0` | `E8 ? ? ? ? 84 C0 74 ? 48 63 03` |
| `HandleTeamIntro` | `void __fastcall HandleTeamIntro(__int64 a1, __int64 a2, char *a3)` | `raw` | `0x7FFA70964FB0` | `0x704FB0` | `48 83 EC ? ? ? ? ? 44 38 89` |
| `HostageServices` | `void *__fastcall HostageServices(__int64 a1)` | `stringref` | `0x7FFA70A76580` | `0x816580` | `"CCSPlayer_HostageServices"` |
| `HudChatPrintf` | `__int64 HudChatPrintf(__int64 a1, unsigned int a2, __int64 a3, ...)` | `rel32` | `0x7FFA713310D0` | `0x10D10D0` | `E8 ? ? ? ? 49 8B 4E 20 BA ? ? ? ?` |
| `InGameMoneyServices` | `__int64 __fastcall InGameMoneyServices(int a1, _QWORD *a2)` | `stringref` | `0x7FFA70A47F50` | `0x7E7F50` | `"CCSPlayerController_InGameMoneyServices"` |
| `Init` | `__int64 __fastcall Init(__int64 a1, __int64 a2, __int64 a3)` | `stringref` | `0x7FFA70DF4860` | `0xB94860` | `"CompositeMaterialPanoramaPanel_t::Init"` |
| `InitFilter` | `__int64 __fastcall InitFilter(__int64 a1, _DWORD *a2, __int64 a3, char a4, char a5)` | `raw` | `0x7FFA7058BFB0` | `0x32BFB0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 0F B6 41 ? 33 FF 24 C9 C7 41 ?` |
| `InitPlayerMovementTraceFilter` | `__int64 __fastcall InitPlayerMovementTraceFilter(__int64 a1, _DWORD *a2, __int64 a3, char a4)` | `raw` | `0x7FFA70AA2AF0` | `0x842AF0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 0F B6 41 ? 33 FF C7 41 ?` |
| `InitTraceData` | `` | `raw` | `0x7FFA70A630E0` | `0x8030E0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8D 79 ? 33 F6 C7 47` |
| `InitTraceInfo` | `__int64 __fastcall InitTraceInfo(__int64 a1)` | `raw` | `0x7FFA7186B7E0` | `0x160B7E0` | `40 55 41 55 41 57 48 83 EC` |
| `InventoryServices` | `__int64 __fastcall InventoryServices(int a1, _QWORD *a2)` | `stringref` | `0x7FFA70A47F50` | `0x7E7F50` | `"CCSPlayerController_InventoryServices"` |
| `IsLatched` | `` | `raw` | `0x7FFA71165420` | `0xF05420` | `0F B6 81 ? ? ? ? C3 ? ? ? ? ? ? ? ? 48 83 EC ? 33 C9` |
| `IsLocalPlayerWatchingOwnDemo` | `` | `raw` | `0x7FFA71165590` | `0xF05590` | `48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 56 48 83 EC ? 48 8B 0D` |
| `IsOverwatch` | `` | `raw` | `0x7FFA711657D0` | `0xF057D0` | `48 83 EC ? E8 ? ? ? ? 0F B6 40 ? 48 83 C4 ? C3` |
| `ItemServices` | `void *__fastcall ItemServices(__int64 a1)` | `stringref` | `0x7FFA70AB2FF0` | `0x852FF0` | `"CCSPlayer_ItemServices"` |
| `LevelInit` | `__int64 __fastcall LevelInit(__int64 a1)` | `raw` | `0x7FFA70B32930` | `0x8D2930` | `40 55 56 41 56 48 8D 6C 24 ? 48 81 EC ? ? ? ? 48` |
| `LevelShutdown` | `` | `raw` | `0x7FFA70D204B0` | `0xAC04B0` | `48 83 EC ? 48 8B 0D ? ? ? ? 48 8D 15` |
| `LoadDefaultKit` | `char __fastcall LoadDefaultKit(__int64 a1, KeyValues *a2, _DWORD *a3)` | `stringref` | `0x7FFA7129BA80` | `0x103BA80` | `"Unable to find "default" paint kit in "paint_kits_rarity""` |
| `LoadFileForMe` | `void __fastcall LoadFileForMe(__int64 a1)` | `raw` | `0x7FFA70B7F6B0` | `0x91F6B0` | `40 55 57 41 56 48 83 EC 20 4C` |
| `LoadPath` | `void __fastcall LoadPath(signed int *a1, signed int a2, unsigned int a3)` | `rel32` | `0x7FFA7091C2B0` | `0x6BC2B0` | `E8 ? ? ? ? 8B 44 24 2C` |
| `LookupBone` | `__int64 __fastcall LookupBone(__int64 a1, __int64 a2)` | `rel32` | `0x7FFA70B2AA10` | `0x8CAA10` | `E8 ? ? ? ? 48 8B 8D ? ? ? ? B3` |
| `MatchFoundHandler` | `` | `raw` | `0x7FFA70EC0580` | `0xC60580` | `48 83 EC ? 48 8B 0D ? ? ? ? 48 85 C9 74 ? ? ? ? 48 89 7C 24` |
| `ModulationUpdate` | `__int64 __fastcall ModulationUpdate(__int64 a1, char a2)` | `raw` | `0x7FFA70C3D2B0` | `0x9DD2B0` | `48 89 5C 24 08 57 48 83 EC 20 8B FA 48 8B D9 E8 ? ? ? ? 84 C0 0F 84` |
| `MovementServices` | `__int64 *MovementServices()` | `stringref` | `0x7FFA70AA0370` | `0x840370` | `"CCSPlayer_MovementServices"` |
| `NoClipOnChange` | `__int64 __fastcall NoClipOnChange(__int64 a1)` | `raw` | `0x7FFA703C6FC0` | `0x166FC0` | `48 89 5C 24 10 48 89 74 24 18 48 89 7C 24 20 55 48 8B EC 48 83 EC 30 48 8D 05` |
| `NoSpread1` | `__int64 __fastcall NoSpread1(__int64 a1, __int64 a2, int a3)` | `raw` | `0x7FFA70EE1A70` | `0xC81A70` | `48 89 5C 24 08 57 48 81 EC F0 00` |
| `OnAddEntity` | `__int64 __fastcall OnAddEntity(__int64 a1, __int64 a2, int a3)` | `raw` | `0x7FFA70BCAF20` | `0x96AF20` | `48 89 74 24 ? 57 48 83 EC ? 41 B9 ? ? ? ? 41 8B C0 41 23 C1 48 8B F2 41 83 F8 ? 48 8B F9 44 0F 45 C8 41 81 F9 ? ? ? ? 73 ? FF 81` |
| `OnBodyGroupChoiceChanged` | `__int64 __fastcall OnBodyGroupChoiceChanged(__int64 a1, __int64 a2, int a3, _DWORD *a4)` | `raw` | `0x7FFA70C880B0` | `0xA280B0` | `48 89 5C 24 08 57 48 83 EC 20 49 63 D8 49 8B F9 45 85 C0 78 20 3B 99 18 02 00 00 7D 18` |
| `OnGlowTypeChanged` | `__int64 __fastcall OnGlowTypeChanged(__int64 a1)` | `raw` | `0x7FFA70D70390` | `0xB10390` | `48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 20 48 8B 05 ? ? ? ? 48 8B D9 F3 0F 10 41 4C` |
| `OnPostDataUpdate` | `` | `raw` | `0x7FFA70C0E930` | `0x9AE930` | `48 89 5C 24 08 48 89 74 24 18 55 57 41 56 48 8B EC 48 83 EC 50 45 8B F1 48 8B FA 48 8B F1 45 85` |
| `OnRemoveEntity` | `__int64 __fastcall OnRemoveEntity(__int64 a1, _QWORD *a2, int a3)` | `raw` | `0x7FFA70BCB780` | `0x96B780` | `48 89 74 24 ? 57 48 83 EC ? 41 B9 ? ? ? ? 41 8B C0 41 23 C1 48 8B F2 41 83 F8 ? 48 8B F9 44 0F 45 C8 41 81 F9 ? ? ? ? 73 ? FF 89` |
| `OnSkeletonModelChanged` | `__int64 __fastcall OnSkeletonModelChanged(__int64 a1, __int64 a2, __int64 *a3)` | `raw` | `0x7FFA70C882C0` | `0xA282C0` | `49 8B 00 48 89 81 B8 00 00 00 C6 81 B0 00 00 00 01 C3` |
| `PanelConstructorPointer` | `` | `raw` | `0x7FFA7189C260` | `0x163C260` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 30 48 8B F1 48 8B FA B9 ? ? ? ? E8 ? ? ? ? 48 8B D8 48 85 C0 74 ? 48` |
| `PanoramaEvent` | `` | `raw` | `0x7FFA70F0C400` | `0xCAC400` | `40 56 57 41 57 48 83 EC ? 48 8B 3D ? ? ? ? 4D 85 C0` |
| `ParseSubtickDuration` | `` | `raw` | `0x7FFA7030D4D0` | `0xAD4D0` | `40 55 48 8D AC 24 70 FD FF FF 48 81 EC 90 03 00 00 F2 0F 10 05 ? ? ? ? 48 8D 05` |
| `ParseSubtickFraction` | `` | `raw` | `0x7FFA7030D810` | `0xAD810` | `40 55 48 8D AC 24 40 FE FF FF 48 81 EC C0 02 00 00 F2 0F 10 05 ? ? ? ? 48 8D 05` |
| `ParticleCollection` | `__int64 __fastcall ParticleCollection(__int64 a1)` | `raw` | `0x7FFA70455150` | `0x1F5150` | `48 89 5C 24 ? 57 48 83 EC ? 0F 28 05` |
| `Pawn` | `__int64 __fastcall Pawn(__int64 a1, __int64 a2)` | `raw` | `0x7FFA7047F310` | `0x21F310` | `48 89 5C 24 ? 57 48 83 EC ? ? ? ? 48 8B FA 48 8B D9 FF 90 ? ? ? ? 84 C0 0F 85` |
| `PerTick` | `void __fastcall PerTick(int *a1)` | `raw` | `0x7FFA70E24A00` | `0xBC4A00` | `40 55 56 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 48 8B B9 A0 00 00 00` |
| `PerTickOrchestrator` | `char __fastcall PerTickOrchestrator(_QWORD *a1)` | `raw` | `0x7FFA70E275E0` | `0xBC75E0` | `48 8B C4 55 53 48 8D A8 ? ? ? ? 48 81 EC ? ? ? ? 80 B9 B1 13 00 00 00` |
| `PerformBatchedInvalidatePhysicsRecursive` | `void __fastcall PerformBatchedInvalidatePhysicsRecursive(char a1)` | `raw` | `0x7FFA70BA0F10` | `0x940F10` | `40 57 48 81 EC 90 00 00 00 84 C9 74 4D BF 01 00 00 00 F0 0F C1 3D ? ? ? ? FF C7 83 FF 01 0F 85 63 05 00 00 48 8D 0D ? ? ? ? 48 8D 15` |
| `PingServices` | `void *__fastcall PingServices(__int64 a1)` | `stringref` | `0x7FFA70AB4290` | `0x854290` | `"CCSPlayer_PingServices"` |
| `PlayVSound_client` | `__int64 __fastcall PlayVSound_client(__int64 a1)` | `raw` | `0x7FFA7177EDA0` | `0x151EDA0` | `48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 55 48 8D 6C 24 ? 48 81 EC ? ? ? ? 33 FF` |
| `PointerToGetInaccuracyFunction` | `` | `raw` | `0x7FFA709F7620` | `0x797620` | `48 89 5C 24 ? 55 56 57 48 81 EC ? ? ? ? 44` |
| `PointerToGetSpreadFunction` | `` | `raw` | `0x7FFA709F8640` | `0x798640` | `48 83 EC ? 48 63 91` |
| `PostDataUpdate` | `char __fastcall PostDataUpdate(__int64 a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFA70C89250` | `0xA29250` | `48 8B C4 4C 89 40 18 89 50 10 55 57 48 8D A8 68 FE FF FF 48 81 EC 88 02 00 00 48 89 70 E0 48 8B` |
| `ProcessForceSubtickMoves` | `` | `raw` | `0x7FFA70C38D50` | `0x9D8D50` | `40 55 53 48 8D AC 24 68 FF FF FF 48 81 EC 98 01 00 00 8B 15 ? ? ? ? 48 8B D9 65 48 8B 04 25 58 00 00 00 B9 98 00 00 00 48 8B 04 D0 8B 04 01 39 05 ? ? ? ? 0F 8F B6 07 00 00` |
| `ProcessImpacts` | `__int64 __fastcall ProcessImpacts(_QWORD *a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFA70C31850` | `0x9D1850` | `48 8B C4 53 56 41 55` |
| `ProcessMovement` | `__int64 __fastcall ProcessMovement(__int64 a1, __int64 a2)` | `rel32` | `0x7FFA70C3C890` | `0x9DC890` | `E8 ? ? ? ? 48 8B 06 48 8B CE FF 90 ? ? ? ? 48 85 DB` |
| `QueueForceSubtickMove` | `` | `raw` | `0x7FFA70C2A6F0` | `0x9CA6F0` | `48 83 EC 28 8B 0D ? ? ? ? 65 48 8B 04 25 58 00 00 00 BA 98 00 00 00 48 8B 04 C8 8B 04 02 39 05 ? ? ? ? 0F 8F F4 11 00 00` |
| `QueuePostDataUpdates` | `` | `raw` | `0x7FFA7171DF00` | `0x14BDF00` | `48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 40 80 B9 DA 0B 00 00 00 49 8B D8 8B FA 48 8B F1 74 61` |
| `RegenerateWeaponSkin` | `void __fastcall RegenerateWeaponSkin(__int64 a1, char a2)` | `raw` | `0x7FFA709EDCD0` | `0x78DCD0` | `40 55 53 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 44 0F B6 FA 48 8B D9 BA ? ? ? ? 48 8D 0D ? ? ? ? E8 ? ? ? ?` |
| `RegenerateWeaponSkin_v2` | `void __fastcall RegenerateWeaponSkin_v2(__int64 a1, char a2)` | `raw` | `0x7FFA709EDCD0` | `0x78DCD0` | `40 55 53 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 44 0F B6 FA 48 8B D9 BA ? ? ? ? 48 8D 0D ? ? ? ? E8` |
| `RegenerateWeaponSkins` | `__int64 RegenerateWeaponSkins()` | `raw` | `0x7FFA70A12950` | `0x7B2950` | `48 83 EC ? E8 ? ? ? ? 48 85 C0 0F 84 ? ? ? ? 48 8B 10` |
| `ReportHit` | `char __fastcall ReportHit(_QWORD *a1)` | `rel32` | `0x7FFA70862670` | `0x602670` | `E8 ? ? ? ? 48 8B AC 24 D8 00 00 00 48 81 C4` |
| `RunCommand` | `void __fastcall RunCommand(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70C3E950` | `0x9DE950` | `48 8B C4 48 81 EC ? ? ? ? 48 89 58 10` |
| `RunCommand_processor` | `void __fastcall RunCommand_processor(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70C3E950` | `0x9DE950` | `48 8B C4 48 81 EC C8 00 00 00 48 89 58 10 48 89 68 18 48 8B EA 48 89 70 20 48 8B F1 48 89 78 F8` |
| `SOCreated` | `void __fastcall SOCreated(uintptr_t, uint64_t, uintptr_t, int)` | `raw` | `0x7FFA705E75F0` | `0x3875F0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B FA 48 8B F1` |
| `Scope_callsite` | `__int64 __fastcall Scope_callsite(__int64 a1, __int64 a2)` | `rel32` | `0x7FFA70ABF930` | `0x85F930` | `E8 ? ? ? ? 80 7C 24 34 ? 74 ?` |
| `SendChatMessage` | `__int64 SendChatMessage(__int64 a1, unsigned int a2, __int64 a3, ...)` | `rel32` | `0x7FFA713310D0` | `0x10D10D0` | `E8 ? ? ? ? 49 8B 4E 20 BA ? ? ? ?` |
| `SetBodyGroup` | `` | `raw` | `0x7FFA70B3C750` | `0x8DC750` | `85 D2 0F 88 ? ? ? ? 55 53 56 41 56 48 8B EC 48 83 EC 78` |
| `SetBodyGroup_inv` | `void __fastcall SetBodyGroup_inv(__int64 a1, int a2, const char *a3)` | `raw` | `0x7FFA70FFABE0` | `0xD9ABE0` | `85 D2 0F 88 ? ? ? ? 53 55` |
| `SetBodygroup` | `void __fastcall SetBodygroup(__int64 a1, int a2, int a3)` | `raw` | `0x7FFA70B3C750` | `0x8DC750` | `85 D2 0F 88 CB 01 00 00 55 53 56 41 56 48 8B EC 48 83 EC 78 45 8B F0 8B DA 48 8B F1 E8 ? ? ?` |
| `SetCollisionBounds` | `__int64 __fastcall SetCollisionBounds(__int64 a1, __int64 *a2)` | `raw` | `0x7FFA70A664B0` | `0x8064B0` | `48 83 EC ? F2 0F 10 02 8B 42 08` |
| `SetDynamicAttributeValue` | `__int64 __fastcall SetDynamicAttributeValue(__int64 a1, __int64 a2, _DWORD *a3)` | `raw` | `0x7FFA71273F20` | `0x1013F20` | `48 89 6C 24 ? 57 41 56 41 57 48 81 EC ? ? ? ? 48 8B FA C7 44 24 ? ? ? ? ? 4D 8B F8` |
| `SetDynamicAttributeValue_raw` | `__int64 __fastcall SetDynamicAttributeValue_raw(__int64 a1, __int64 a2, _DWORD *a3)` | `raw` | `0x7FFA71273F20` | `0x1013F20` | `48 89 6C 24 ? 57 41 56 41 57 48 81 EC ? ? ? ? 48 8B FA C7 44 24` |
| `SetMaterialGroup` | `void __fastcall SetMaterialGroup(__int64 a1, unsigned int a2)` | `raw` | `0x7FFA70C8F5D0` | `0xA2F5D0` | `3B 91 C4 03 00 00 74 24 89 91 C4 03 00 00 48 8B 81 28 02 00 00 48 85 C0 74 12` |
| `SetMeshGroupMask` | `__int64 __fastcall SetMeshGroupMask(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70C908F0` | `0xA308F0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8D 99` |
| `SetModel` | `__int64 __fastcall SetModel(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70B3DAA0` | `0x8DDAA0` | `40 53 48 83 EC ? 48 8B D9 4C 8B C2 48 8B 0D ? ? ? ? 48 8D 54 24` |
| `SetPlayerReady` | `char __fastcall SetPlayerReady(__int64 a1, __int64 a2)` | `raw` | `0x7FFA71184B30` | `0xF24B30` | `40 53 48 83 EC 20 48 8B DA 48 8D 15 ? ? ? ? 48 8B CB FF 15 ? ? ? ? 85 C0 75 14 BA` |
| `SetSelectedIndexFunctionPointer` | `` | `raw` | `0x7FFA718F6150` | `0x1696150` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B F1 8B DA 48 83` |
| `SetTraceData` | `__int64 __fastcall SetTraceData(int *a1, _OWORD *a2)` | `rel32` | `0x7FFA70A367F0` | `0x7D67F0` | `E8 ? ? ? ? 8B 85 ? ? ? ? 48 8D 54 24 ? F2 0F 10 45` |
| `SetTraceInit` | `` | `rel32` | `0x7FFA70D5B720` | `0xAFB720` | `E8 ? ? ? ? F2 0F 10 ? 4C 8D ?` |
| `SetTypeKV3` | `unsigned __int64 *__fastcall SetTypeKV3(unsigned __int64 *a1, unsigned __int8 a2, unsigned __int8 a3)` | `raw` | `0x7FFA71A8A380` | `0x182A380` | `40 53 48 83 EC 30 4C 8B 11 41 B9 ? ? ? ? 49 83 CA 01 0F B6 C2 80 FA 06 48 8B D9 44 0F 45 C8` |
| `SetViewAngle` | `void __fastcall SetViewAngle(__int64 a1, int a2, __int64 *a3)` | `raw` | `0x7FFA70D47D80` | `0xAE7D80` | `85 D2 75 3D 48 63 81 ? ? ? ?` |
| `SetViewAngles` | `` | `raw` | `0x7FFA70D47D80` | `0xAE7D80` | `85 D2 75 ? 48 63 81` |
| `SetupCmd` | `__int64 __fastcall SetupCmd(__int64 a1)` | `raw` | `0x7FFA70B1D8E0` | `0x8BD8E0` | `48 83 EC 28 E8 ? ? ? ? 8B 80` |
| `SetupMapInfo` | `` | `raw` | `0x7FFA70EEC020` | `0xC8C020` | `48 8B C4 48 89 58 ? 48 89 68 ? 48 89 70 ? 57 48 81 EC ? ? ? ? 0F 29 70 ? 48 8B EA 0F 29 78 ? 45 33 C0` |
| `SetupMove` | `__int64 __fastcall SetupMove(__int64 a1, int *a2)` | `raw` | `0x7FFA70F809E0` | `0xD209E0` | `48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 56 48 83 EC ? 48 8B EA 4C 8B F1 E8 ? ? ? ? 48 8D 15` |
| `SetupMovementMoves` | `__int64 __fastcall SetupMovementMoves(__int64 a1, __int64 a2, __int64 a3, __int64 a4)` | `raw` | `0x7FFA713F6D2F` | `0x1196D2F` | `48 8B ? E8 ? ? ? ? 48 8B 5C 24 ? 48 8B 6C 24 ? 48 83 C4 30` |
| `SharedRandomFloat` | `` | `raw` | `0x7FFA70C91BF0` | `0xA31BF0` | `4C 8B DC 49 89 5B 08 49 89 73 10 57 48 81 EC 00 01 00 00 8B 05 ? ? ? ? 48 8D 54 24 40` |
| `ShouldShowHudElements` | `` | `raw` | `0x7FFA71185B20` | `0xF25B20` | `48 83 EC ? BA ? ? ? ? 48 8D 0D ? ? ? ? E8 ? ? ? ? 48 85 C0 75 ? 48 8B 05 ? ? ? ? 48 8B 40 ? ? ? 00 74 ? BA` |
| `ShowMessageBox` | `` | `raw` | `0x7FFA70F08BA0` | `0xCA8BA0` | `44 88 4C 24 ? 53 41 56` |
| `Shutdown` | `__int64 Shutdown()` | `raw` | `0x7FFA70D48C30` | `0xAE8C30` | `48 89 5C 24 08 55 56 57 41 54 41 55 41 56 41 57 48 81 EC 40 02 00 00 8B 0D ? ? ? ? BA 02 00 00` |
| `SomeTimingFromPawn` | `float __fastcall SomeTimingFromPawn(__int64 a1, int a2, unsigned int a3)` | `raw` | `0x7FFA70CBA060` | `0xA5A060` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 49 63 D8 48 8B F1` |
| `SpectatorInput` | `__int64 __fastcall SpectatorInput(_DWORD *a1, __int64 a2, unsigned int a3)` | `raw` | `0x7FFA70A3B950` | `0x7DB950` | `48 89 5C 24 10 55 56 57 41 56 41 57 48 8B EC 48 83 EC 60 48 8B 01 41 8B F8 48 8B DA 48 8B F1 FF` |
| `SpreadSeedGen` | `__int64 __fastcall SpreadSeedGen(__int64 a1, __int64 a2, int a3)` | `raw` | `0x7FFA70EE1A70` | `0xC81A70` | `48 89 5C 24 08 57 48 81 EC F0 00 00 00 F3 0F 10 0A 48 8D 8C 24 10 01 00 00 41 8B D8 48 8B FA E8` |
| `StartHierarchicalAttachment` | `char __fastcall StartHierarchicalAttachment(__int64 a1)` | `raw` | `0x7FFA70BEEEC0` | `0x98EEC0` | `48 89 5C 24 10 48 89 6C 24 18 48 89 74 24 20 57 41 54 41 55 41 56 41 57 48 83 EC 30 48 8B F9 8B` |
| `SubmitCommendation` | `` | `raw` | `0x7FFA7118B180` | `0xF2B180` | `48 89 74 24 ? 55 57 41 56 48 8D 6C 24 ? 48 81 EC ? ? ? ? 48 8B CA` |
| `SubmitPlayerReport` | `` | `raw` | `0x7FFA7118B460` | `0xF2B460` | `48 89 5C 24 ? 56 48 83 EC ? 48 8B CA` |
| `TakeDamageOld` | `unsigned __int64 __fastcall TakeDamageOld(__int64 a1, __int64 a2, __int64 **a3)` | `raw` | `0x7FFA704840E0` | `0x2240E0` | `40 55 53 56 57 41 54 48 8D 6C 24 E0 48 81 EC 20 01 00 00 4D 8B E0 48 8B FA 48 8B F1 E8` |
| `TestSurfaces` | `void __fastcall TestSurfaces(__int64 a1, float a2, float a3, float a4, int a5, int a6, __int64 a7)` | `raw` | `0x7FFA70A698A0` | `0x8098A0` | `40 53 57 41 56 48 83 EC 50 8B` |
| `ThinkReturn` | `char __fastcall ThinkReturn(__int64 a1, _DWORD *a2)` | `raw` | `0x7FFA7057A8BF` | `0x31A8BF` | `BA 04 00 00 00 FF 15 ? ? ? ? 84 C0 0F 84` |
| `ThirdPersonOffHandler` | `__int64 ThirdPersonOffHandler()` | `raw` | `0x7FFA70D2D100` | `0xACD100` | `48 83 EC 28 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 08 03 00 00 83 7C 24 ? 00 75 ? 48 8B 05 ? ? ? ? C6 80 29 02 00 00 00 C7 80 A8 06 00 00 00` |
| `ThirdPersonOnHandler` | `__int64 ThirdPersonOnHandler()` | `raw` | `0x7FFA70D2D1E0` | `0xACD1E0` | `48 83 EC 38 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 08 03 00 00 83 7C 24 ? 00 0F 85 ? ? ? ? 4C 8B 05 ? ? ? ? 41 8B 80 50 0B 00 00` |
| `ThirdPersonReset` | `` | `raw` | `0x7FFA70D2B5B0` | `0xACB5B0` | `48 8B 40 08 44 38 ? 75 10 44 88 ? 01` |
| `TraceCreate` | `char __fastcall TraceCreate(__int64 a1, __int64 a2, int a3, __int64 a4, int a5, char a6)` | `raw` | `0x7FFA70A67390` | `0x807390` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 56 41 57 48 83 EC 50 F2 0F 10 02` |
| `TraceGetInfo` | `__int64 __fastcall TraceGetInfo(__int64 a1, __int64 a2, float a3, unsigned __int64 *a4)` | `raw` | `0x7FFA70A699C0` | `0x8099C0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC 60 48 8B E9 0F 29 74 24` |
| `TraceHandleBulletPen` | `char __fastcall TraceHandleBulletPen(__int64 a1, float *a2, __int64 a3, int a4, __int64 a5)` | `raw` | `0x7FFA70A83BD0` | `0x823BD0` | `48 8B C4 44 89 48 20 48 89 50 10 48 89 48 08 55 57 41 57` |
| `TraceInitData` | `__int64 __fastcall TraceInitData(__int64 a1)` | `raw` | `0x7FFA70A630E0` | `0x8030E0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 20 48 8D 79 ? 33 F6 C7 47` |
| `TraceInitFilter` | `__int64 __fastcall TraceInitFilter(__int64 a1, _DWORD *a2, __int64 a3, char a4, char a5)` | `raw` | `0x7FFA7058BFB0` | `0x32BFB0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 0F B6 41 ? 33 FF 24` |
| `TraceInitInfo` | `__int64 __fastcall TraceInitInfo(__int64 a1)` | `raw` | `0x7FFA7186B7E0` | `0x160B7E0` | `40 55 41 55 41 57 48 83 EC 30` |
| `TracePlayerBBox` | `__int64 __fastcall TracePlayerBBox(__int64 a1, __int64 *a2, __int64 *a3)` | `raw` | `0x7FFA70DD4430` | `0xB74430` | `48 89 5C 24 ? 55 57 41 54 41 55 41 56` |
| `TraceShape` | `bool __fastcall TraceShape(_QWORD *a1, __int64 a2, __int64 a3, __int64 a4, _BYTE *a5, __int64 a6)` | `raw` | `0x7FFA70BF1380` | `0x991380` | `48 89 5C 24 ? 48 89 4C 24 ? 55 57` |
| `TraceToExit` | `char __fastcall TraceToExit(__int64 a1, __int64 a2, int a3, __int64 a4, int a5, char a6)` | `raw` | `0x7FFA70A67390` | `0x807390` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 56 41 57 48 83 EC ? F2 0F 10 02` |
| `TypeManager` | `__int64 __fastcall TypeManager(int a1, __int64 a2)` | `stringref` | `0x7FFA71649150` | `0x13E9150` | `"InfoForResourceTypeCCompositeMaterialKit"` |
| `UnknownParticleFunction` | `` | `raw` | `0x7FFA70BE9DC0` | `0x989DC0` | `40 56 48 83 EC ? 41 8B F0` |
| `UnserializeEvent` | `__int64 __fastcall UnserializeEvent(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70BF5730` | `0x995730` | `48 8B C4 48 89 50 10 55 41 54 41 55 41 56 48 8D 68 D8 48 81 EC 08 01 00 00 48 89 58 D8 4C 8D B1` |
| `UntrustedFlagSetter` | `` | `raw` | `0x7FFA703B6F05` | `0x156F05` | `74 26 C6 05 ? ? ? ? 01 33 C0 83 F8 01` |
| `UpdateGlobalVars` | `void *__fastcall UpdateGlobalVars(__int64 a1, void *a2)` | `raw` | `0x7FFA70D477D0` | `0xAE77D0` | `48 8B 0D ? ? ? ? 4C 8D 05 ? ? ? ? 48 85 D2` |
| `UpdateOnRemove` | `` | `raw` | `0x7FFA71739BF0` | `0x14D9BF0` | `48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 40 48 8B D9 C6 05 ? ? ? ? 01 48 8B 49` |
| `UpdatePostProcessing` | `void __fastcall UpdatePostProcessing(__int64 a1, _BYTE *a2)` | `raw` | `0x7FFA71188CC0` | `0xF28CC0` | `48 85 D2 0F 84 ? ? ? ? 48 89 5C 24 08 57 48 83 EC 60 80` |
| `UpdateSkybox` | `__int64 __fastcall UpdateSkybox(__int64 a1)` | `raw` | `0x7FFA704BAC10` | `0x25AC10` | `48 89 5C 24 ? 57 48 83 EC ? 48 8B F9 E8 ? ? ? ? 48 8B 47` |
| `UpdateSubClass` | `void __fastcall UpdateSubClass(_QWORD *a1)` | `raw` | `0x7FFA7045ACF0` | `0x1FACF0` | `4C 8B DC 53 48 81 EC ? ? ? ? 48 8B 41 10 48 8B D9 8B 50 30 C1 EA 04` |
| `UpdateTurningInAccuracy` | `float *__fastcall UpdateTurningInAccuracy(float *a1)` | `rel32` | `0x7FFA70A119B0` | `0x7B19B0` | `E8 ? ? ? ? F3 0F 10 87 ? ? ? ? 44 0F 2F C8` |
| `Use` | `` | `raw` | `0x7FFA70A12190` | `0x7B2190` | `40 55 53 56 48 8D AC 24 C0 FE FF FF 48 81 EC 40 02 00 00 48 8B DA 48 8B F1 BA FF FF FF FF` |
| `UseServices` | `__int64 UseServices()` | `stringref` | `0x7FFA70AE45F0` | `0x8845F0` | `"CCSPlayer_UseServices"` |
| `ViewModelHideZoomed` | `__int64 __fastcall ViewModelHideZoomed(__int64 a1, __int64 a2, __int64 **a3)` | `raw` | `0x7FFA70A01F60` | `0x7A1F60` | `48 89 5C 24 20 55 56 57 41 54 41 56 48 8B EC 48 83 EC 50 48 8D 05` |
| `WaterServices` | `__int64 *WaterServices()` | `stringref` | `0x7FFA70AD9880` | `0x879880` | `"CCSPlayer_WaterServices"` |
| `WeaponServices` | `__int64 *WeaponServices()` | `stringref` | `0x7FFA70AD9C30` | `0x879C30` | `"CCSPlayer_WeaponServices"` |
| `create_move_v2` | `void __fastcall create_move_v2(__int64 *a1, int a2, char a3)` | `raw` | `0x7FFA70D2EF70` | `0xACEF70` | `85 D2 0F 85 ? ? ? ? 48 8B C4 44 88 40` |
| `draw_smoke_array` | `__int64 __fastcall draw_smoke_array(__int64 a1, __int64 a2, __int64 a3, __int64 a4, __int64 a5, unsigned int *a6)` | `raw` | `0x7FFA70EDEB20` | `0xC7EB20` | `40 55 41 54 41 55 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 4C 8B E2` |
| `draw_view_punch_v2` | `float *__fastcall draw_view_punch_v2(int a1, __int64 a2, __int64 a3, float *a4, float *a5, float *a6)` | `raw` | `0x7FFA70A66C50` | `0x806C50` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC ? 49 8B E9 49 8B F8` |
| `frame_stage_notify` | `__int64 __fastcall frame_stage_notify(__int64 a1, int a2)` | `raw` | `0x7FFA70D35B81` | `0xAD5B81` | `4C 8B 0D ? ? ? ? 48 8D 15 ? ? ? ? 48 8B 8F ? ? ? ? F3 41 0F 10 51 ? 45 8B 49 ? 0F 5A D2 66 49 0F 7E D0 FF 15 ? ? ? ? 48 8B 97 ? ? ? ? 48 8B 0D ? ? ? ? E8 ? ? ? ? E9` |
| `get_fov` | `float *__fastcall get_fov(int a1, __int64 a2, __int64 a3, float *a4, float *a5, float *a6)` | `raw` | `0x7FFA70A66C50` | `0x806C50` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC ? 49 8B E9 49 8B F8` |
| `get_map_name` | `__int64 get_map_name()` | `raw` | `0x7FFA71143190` | `0xEE3190` | `48 83 EC ? 48 8B 0D ? ? ? ? ? ? ? FF 90 ? ? ? ? 48 8B C8 48 83 C4` |
| `get_view_angles_v2` | `void __fastcall get_view_angles_v2(__int64 a1, int a2, __int64 a3)` | `raw` | `0x7FFA70D37450` | `0xAD7450` | `4D 85 C0 74 ? 85 D2 74` |
| `is_demo_or_hltv` | `char is_demo_or_hltv()` | `raw` | `0x7FFA71164B80` | `0xF04B80` | `48 83 EC ? 48 8B 0D ? ? ? ? ? ? ? FF 90 ? ? ? ? 84 C0 75 ? 38 05` |
| `level_init_v2` | `__int64 __fastcall level_init_v2(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70D5DF90` | `0xAFDF90` | `40 55 56 41 56 48 8D 6C 24 ? 48 81 EC ? ? ? ? 48 8B 0D` |
| `level_shutdown` | `__int64 level_shutdown()` | `raw` | `0x7FFA70D5E210` | `0xAFE210` | `48 83 EC ? 48 8B 0D ? ? ? ? 48 8D 15 ? ? ? ? 45 33 C9 45 33 C0 ? ? ? FF 50 ? 48 85 C0 74 ? 48 8B 0D ? ? ? ? 48 8B D0 ? ? ? 41 FF 50 ? 48 83 C4` |
| `mark_interp_latch_flags_dirty` | `void __fastcall mark_interp_latch_flags_dirty(__int64 a1, unsigned int a2)` | `raw` | `0x7FFA70478430` | `0x218430` | `40 53 56 57 48 83 EC ? 80 3D ? ? ? ? 00` |
| `on_add_entity_v2` | `__int64 __fastcall on_add_entity_v2(__int64 a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFA70BCB490` | `0x96B490` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 8B 81 ? ? ? ? 49 8B F0` |
| `override_view_short` | `void __fastcall override_view_short(__int64 a1, __int64 a2)` | `raw` | `0x7FFA70EC2F10` | `0xC62F10` | `40 57 48 83 EC ? 48 8B FA E8 ? ? ? ? BA` |
| `paintkit_prefab` | `__int64 __fastcall paintkit_prefab(__int64 *a1)` | `stringref` | `0x7FFA712CCD80` | `0x106CD80` | `"set item texture prefab"` |
| `paintkit_seed` | `__int64 __fastcall paintkit_seed(__int64 a1)` | `stringref` | `0x7FFA711570C0` | `0xEF70C0` | `"set item texture seed"` |
| `paintkit_wear` | `__int64 __fastcall paintkit_wear(__int64 a1)` | `stringref` | `0x7FFA711570C0` | `0xEF70C0` | `"set item texture wear"` |
| `remove_legs` | `void __fastcall remove_legs(__int64 *a1, __int64 *a2, __int64 a3, __int64 a4, __int64 a5)` | `raw` | `0x7FFA713603F0` | `0x11003F0` | `40 55 53 56 41 56 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? F2 0F 10 42` |
| `statTrak_killEater` | `__int64 __fastcall statTrak_killEater(__int64 a1)` | `stringref` | `0x7FFA711570C0` | `0xEF70C0` | `"kill eater"` |
| `statTrak_scoreType` | `__int64 statTrak_scoreType()` | `stringref` | `0x7FFA7037BBB0` | `0x11BBB0` | `"kill eater score type"` |
| `unlock_inventory` | `char __fastcall unlock_inventory(__int64 a1)` | `raw` | `0x7FFA709622C0` | `0x7022C0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B E9 48 8B 0D ? ? ? ? ? ? ? FF 50` |
| `update_global_vars` | `void *__fastcall update_global_vars(__int64 a1, void *a2)` | `raw` | `0x7FFA70D477D0` | `0xAE77D0` | `48 8B 0D ? ? ? ? 4C 8D 05 ? ? ? ? 48 85 D2` |
| `update_post_processing_v2` | `void __fastcall update_post_processing_v2(__int64 a1)` | `raw` | `0x7FFA7118D276` | `0xF2D276` | `48 89 AC 24 ? ? ? ? 45 33 ED` |

## `engine2.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `CClient_SendMovePacket` | `char __fastcall CClient_SendMovePacket(__int64 a1)` | `raw` | `0x7FFA82574FA0` | `0x64FA0` | `40 55 57 41 55 48 8D AC 24 90 E0 FF FF B8 70 20 00 00 E8 ? ? ? ? 48 2B E0 4C 8B E9 C7 44 24 20 FF FF FF FF` |
| `CGameEventSystem_PostEventAbstract` | `__int64 __fastcall CGameEventSystem_PostEventAbstract(_BYTE *a1, unsigned int a2, char a3, int a4, __int64 *a5, __int64 a6, __int64 a7, __int64 a8, char a9)` | `raw` | `0x7FFA82726530` | `0x216530` | `48 89 5C 24 10 48 89 74 24 18 48 89 7C 24 20 55 41 54 41 55 41 56 41 57 48 8D 6C 24 F1 48 81 EC A0 00 00 00 4C 8B 65 67 4C 8B F1` |
| `CHLTVClient_SendSnapshot` | `char __fastcall CHLTVClient_SendSnapshot(__int64 a1, __int64 a2)` | `raw` | `0x7FFA82632120` | `0x122120` | `48 89 54 24 10 48 89 4C 24 08 55 53 56 57 41 56 41 57 48 8D 6C 24 88 48 81 EC 78 01 00 00 48 8D 05 ? ? ? ? 48 C7 45 18 7A 02 00 00` |
| `CHLTVClient_SetSignonState` | `char __fastcall CHLTVClient_SetSignonState(__int64 a1, int a2, __int64 a3, int a4)` | `raw` | `0x7FFA82633790` | `0x123790` | `40 55 53 41 55 41 56 41 57 48 8D 6C 24 C9 48 81 EC E0 00 00 00 45 8B E8 8B DA 4C 8B F9 45 33 F6` |
| `CHostStateMgr_HostStateRequest_Start` | `void __fastcall CHostStateMgr_HostStateRequest_Start(__int64 a1, __int64 a2)` | `raw` | `0x7FFA82729AF0` | `0x219AF0` | `40 53 48 83 EC 40 8B 01 48 8B D9 C6 41 18 01 83 F8 02 74 07 83 F8 04 75 21 EB 0D 8B 49 20 83 E9 06 74 17 83 F9 01 74 12` |
| `CInputService_ProcessConVar` | `void __fastcall CInputService_ProcessConVar(__int64 a1, __int64 a2)` | `raw` | `0x7FFA826D3DB0` | `0x1C3DB0` | `48 89 5C 24 08 48 89 74 24 10 48 89 7C 24 18 55 41 54 41 55 41 56 41 57 48 8D AC 24 40 F3 FF FF 48 81 EC C0 0D 00 00` |
| `CNetworkGameClient_InternalProcessPacketEntities` | `void __fastcall CNetworkGameClient_InternalProcessPacketEntities(__int64 a1, __int64 a2)` | `raw` | `0x7FFA825583B0` | `0x483B0` | `40 55 56 57 41 56 41 57 48 8D AC 24 40 FF FF FF 48 81 EC C0 01 00 00 65 48 8B 04 25 58 00 00 00` |
| `CNetworkGameClient_ProcessServerInfo` | `char __fastcall CNetworkGameClient_ProcessServerInfo(__int64 a1, __int64 a2)` | `raw` | `0x7FFA8257B140` | `0x6B140` | `48 89 5C 24 08 57 48 83 EC 30 48 8B FA 48 8B D9 8B 0D ? ? ? ? BA 02 00 00 00 FF 15` |
| `CNetworkStringTableContainer_CreateStringTable` | `__int64 __fastcall CNetworkStringTableContainer_CreateStringTable(__int64 a1, const char *a2, __int64 a3)` | `raw` | `0x7FFA8261C7F0` | `0x10C7F0` | `40 53 41 56 48 83 EC 48 4C 8B F2 48 8B D9 48 8B 12 48 85 D2 0F 84 ? ? ? ? 80 79 34 00` |
| `CNetworkStringTableContainer_WriteUpdateMessageAtTick` | `__int64 __fastcall CNetworkStringTableContainer_WriteUpdateMessageAtTick(__int64 a1, __int64 a2, int a3, int a4, int a5)` | `raw` | `0x7FFA8261D470` | `0x10D470` | `44 89 4C 24 20 44 89 44 24 18 48 89 4C 24 08 55 53 56 57 41 54 41 55 41 57 48 8D 6C 24 F0` |
| `CServerSideClient_ProcessServerInfo` | `char __fastcall CServerSideClient_ProcessServerInfo(__int64 a1, __int64 a2)` | `raw` | `0x7FFA82594B20` | `0x84B20` | `48 89 5C 24 20 55 56 57 41 54 41 56 48 8D AC 24 10 FE FF FF 48 81 EC F0 02 00 00` |
| `CSplitScreenSlot` | `char __fastcall CSplitScreenSlot(__int64 a1, __int64 a2, int a3, __int64 a4)` | `stringref` | `0x7FFA8275AF50` | `0x24AF50` | `"CSplitScreenSlot"` |
| `ClientCommand` | `char ClientCommand(__int64 a1, int a2, __int64 a3, ...)` | `raw` | `0x7FFA825B1270` | `0xA1270` | `48 8B C4 4C 89 40 18 4C 89 48 20 55 53 57 48 8D 68 A1 48 81 EC C0 00 00 00 33 FF 48 63 DA 48 39` |
| `Connect` | `void __fastcall Connect(__int64 a1, int a2, unsigned int a3, __int64 a4, unsigned int a5, char a6)` | `raw` | `0x7FFA8258F420` | `0x7F420` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 57 48 83 EC 40 44 89 81 3C 02 00 00 49 8B E9 44 8B` |
| `DisablePvsAccessor` | `__int64 __fastcall DisablePvsAccessor(_DWORD *a1, __int64 a2, int a3, char a4)` | `raw` | `0x7FFA8274E0D2` | `0x23E0D2` | `48 8D 0D ? ? ? ? 33 D2 FF 50` |
| `Engine_Disconnect_main` | `__int64 *Engine_Disconnect_main()` | `raw` | `0x7FFA826E2210` | `0x1D2210` | `48 89 5C 24 20 55 57 41 54 48 8B EC 48 83 EC 70 45 33 E4 48 C7 05` |
| `ExecuteStringCommand` | `char __fastcall ExecuteStringCommand(__int64 a1, __int64 a2)` | `raw` | `0x7FFA82630ED0` | `0x120ED0` | `40 53 56 48 81 EC 48 07 00 00 48 8B F1 48 8B DA 48 8B 4A 48 48 83 E1 FC 48 83 79 18 0F 76 03 48` |
| `ForceDemoRecordingFullUpdateAfterNextDeltaPacket` | `char __fastcall ForceDemoRecordingFullUpdateAfterNextDeltaPacket(__int64 a1, const char *a2)` | `raw` | `0x7FFA825392C0` | `0x292C0` | `48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 20 48 8B 1D ? ? ? ? 48 8B FA 48 8B F1 48 85 DB` |
| `GetFreeClient` | `` | `raw` | `0x7FFA825BF4B0` | `0xAF4B0` | `48 89 54 24 ? 53 56 57 41 56 48 83 EC` |
| `GetLevelName` | `` | `raw` | `0x7FFA82586540` | `0x76540` | `48 83 EC ? E8 ? ? ? ? 84 C0 74 ? 48 8D 05 ? ? ? ? 48 83 C4 ? C3 48 8B 0D ? ? ? ? 48 85 C9 74 ? 83 B9 ? ? ? ? ? 7C ? 48 8B 89 ? ? ? ? 48 8D 05 ? ? ? ? 48 85 C9 48 0F 45 C1 48 83 C4 ? C3 48 8D 05 ? ? ? ? 48 83 C4 ? C3 ? ? ? ? ? ? ? ? ? ? ? ? 48 83 EC` |
| `GetLevelNameShort` | `` | `raw` | `0x7FFA825865A0` | `0x765A0` | `48 83 EC ? E8 ? ? ? ? 84 C0 74 ? 48 8D 05 ? ? ? ? 48 83 C4 ? C3 48 8B 0D ? ? ? ? 48 85 C9 74 ? 83 B9 ? ? ? ? ? 7C ? 48 8B 89 ? ? ? ? 48 8D 05 ? ? ? ? 48 85 C9 48 0F 45 C1 48 83 C4 ? C3 48 8D 05 ? ? ? ? 48 83 C4 ? C3 ? ? ? ? ? ? ? ? ? ? ? ? B8` |
| `GetScreenAspectRatio` | `float __fastcall GetScreenAspectRatio(__int64 a1, int a2, int a3)` | `raw` | `0x7FFA825869F0` | `0x769F0` | `48 89 5C 24 08 57 48 83 EC 20 8B FA 48 8D 0D` |
| `HostStateRequest` | `` | `raw` | `0x7FFA8272BB20` | `0x21BB20` | `48 89 74 24 ? 57 48 83 EC ? 33 F6 48 8B FA 48 39 35` |
| `Host_FilterTime` | `bool __fastcall Host_FilterTime(__int64 a1, float *a2)` | `raw` | `0x7FFA827218F0` | `0x2118F0` | `48 89 5C 24 10 48 89 74 24 18 48 89 4C 24 08 57 48 81 EC A0 00 00 00 48 8B BC 24 D0 00 00 00` |
| `IsConnected` | `` | `raw` | `0x7FFA825864A0` | `0x764A0` | `48 8B 05 ? ? ? ? 48 85 C0 74 ? 83 B8 ? ? ? ? ? 0F 9D C0` |
| `IsHearingClient` | `` | `raw` | `0x7FFA825CCD30` | `0xBCD30` | `40 53 48 83 EC ? 48 8B D9 3B 51` |
| `IsInGame` | `bool IsInGame()` | `raw` | `0x7FFA82586470` | `0x76470` | `48 8B 05 ? ? ? ? 48 85 C0 74 ? 80 B8 ? ? ? ? 00 75 ? 83 B8 ? ? ? ? ? 7C` |
| `LoadGameInfo` | `char __fastcall LoadGameInfo(__int64 a1, const char *a2)` | `raw` | `0x7FFA8269E460` | `0x18E460` | `40 55 56 41 56 48 8D 6C 24 F0 48 81 EC 10 01 00 00 4C 8B F1 C7 44 24 40 00 00 00 00 48 8B CA 48` |
| `MountAddon` | `void __fastcall MountAddon(__int64 a1, const char *a2, char a3)` | `raw` | `0x7FFA826A4140` | `0x194140` | `48 85 D2 0F 84 DA 0A 00 00 48 8B C4 44 88 40 18 55 57 41 54 41 57 48 8D A8 C8 FC FF FF 48 81 EC` |
| `NetTimeoutDisconnect` | `__int64 __fastcall NetTimeoutDisconnect(__int64 a1, unsigned int a2, __int64 a3)` | `raw` | `0x7FFA825797A0` | `0x697A0` | `40 53 55 56 57 41 56 48 81 EC 80 00 00 00 0F 29 74 24 70 49 8B F8` |
| `OnSvCheatsChange` | `void __fastcall OnSvCheatsChange(__int64 a1, __int64 a2, _BYTE *a3, char *a4)` | `raw` | `0x7FFA825AC220` | `0x9C220` | `40 53 48 83 EC 20 48 8B 41 08 48 8B D9 8B 50 30 48 C1 EA 0C F6 C2 01 0F 85` |
| `ProcessTick` | `char __fastcall ProcessTick(__int64 a1, __int64 a2)` | `raw` | `0x7FFA8257AB10` | `0x6AB10` | `48 89 5C 24 20 55 57 41 57 48 81 EC F0 00 00 00 8B 7A 50 45 33 FF 44 38 3D ? ? ? ? 48 8B EA` |
| `QueueNewRequest` | `__int64 __fastcall QueueNewRequest(__int64 a1, __int64 a2)` | `raw` | `0x7FFA8272BCC0` | `0x21BCC0` | `48 89 6C 24 18 48 89 7C 24 20 41 56 48 83 EC 30 48 8B EA 48 8B F9 8B 0D ? ? ? ? BA 02 00 00` |
| `RegisterConCommand` | `_QWORD *__fastcall RegisterConCommand(_QWORD *a1, __int64 a2, __int128 *a3, __int64 a4, __int64 a5, __int128 *a6)` | `raw` | `0x7FFA8290DAC0` | `0x3FDAC0` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 57 48 83 EC 60 44 8B 15` |
| `RegisterConVar` | `__int128 *__fastcall RegisterConVar(_QWORD *a1, __int64 a2, __int64 a3, __int64 a4, __int128 *a5)` | `raw` | `0x7FFA8290C8D0` | `0x3FC8D0` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 41 57 48 81 EC D0 00 00` |
| `ReplyConnection` | `` | `raw` | `0x7FFA825B4AE0` | `0xA4AE0` | `48 8B C4 55 41 55 41 56` |
| `RunPrediction` | `void __fastcall RunPrediction(__int64 a1, unsigned int a2)` | `raw` | `0x7FFA825764B0` | `0x664B0` | `40 55 41 56 48 83 EC ? 80 B9` |
| `SetSignonState` | `char __fastcall SetSignonState(__int64 a1, unsigned int a2, unsigned int a3, __int64 a4)` | `raw` | `0x7FFA82570FA0` | `0x60FA0` | `44 89 44 24 18 89 54 24 10 55 53 56 57 41 55 41 56 41 57 48 8D 6C 24 D9 48 81 EC D0 00 00 00 8B` |
| `Tokenize` | `` | `raw` | `0x7FFA8290DF60` | `0x3FDF60` | `48 89 6C 24 20 4C 89 44 24 18 56 57 41 54 41 56 41 57 48 83 EC 70 48 8B F2 49 8B E8 8B 51 08 4C` |

## `gameoverlayrenderer64.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `GameOverlay` | `` | `raw` | `0x7FFB099E5230` | `0x95230` | `48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 54 41 56 41 57 48 83 EC ? 41 8B E8` |

## `inputsystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `AttachToWindow` | `int __fastcall AttachToWindow(__int64 a1, HWND a2)` | `raw` | `0x7FFB099039F0` | `0x39F0` | `48 89 5C 24 20 55 48 83 EC 20 48 63 41 30 48 8B EA 33 D2 48 8B D9 85 C0 7E 20 4C 8B C0 8B CA` |
| `EventHandler` | `void __fastcall SDL_EventHandler(__int64 a1, SDL_Event* event)` | `raw` | `0x7FFB09904F01` | `0x4F01` | `53 48 81 EC ? ? ? ? 8B 02 48 8B DA 2D 00 04 00 00` |

## `matchmaking.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `InitializeGameSettings` | `char __fastcall InitializeGameSettings(__int64 a1)` | `raw` | `0x7FFA6C91E6A0` | `0xEE6A0` | `40 53 48 81 EC 40 01 00 00 48 89 BC 24 58 01 00 00 48 8D 15 ? ? ? ? 48 8B F9 41 B0 01 48 8B 49 10 FF 15 ? ? ? ? 48 8B D8 48 85 C0 74 59` |

## `materialsystem2.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `ApplyMaterialVarsForBatch` | `` | `raw` | `0x7FFACC4F8B80` | `0x18B80` | `4C 89 4C 24 20 4C 89 44 24 18 48 89 54 24 10 53 55 56 57 41 54 41 55 41 56 41 57 48 83 EC 78` |
| `BindIdentityInstanceIDBufferAndSetRenderState` | `char __fastcall BindIdentityInstanceIDBufferAndSetRenderState(__int64 *a1, __int64 a2, __int64 a3, __int64 a4)` | `stringref` | `0x7FFACC550000` | `0x70000` | `"BindIdentityInstanceIDBufferAndSetRenderState: GetMode == NULL? Can't Render\n"` |
| `BuildPassCommandData` | `int __fastcall BuildPassCommandData(__int64 a1, int a2, __int64 a3)` | `raw` | `0x7FFACC4F8F80` | `0x18F80` | `89 54 24 10 55 53 56 57 41 54 41 55 41 56 41 57 48 8D AC 24 58 FE FF FF 48 81 EC A8 02 00 00` |
| `CacheGate` | `__int64 __fastcall CacheGate(__int64 a1, unsigned __int64 a2, __int64 a3, int a4, __int64 a5, int a6, char a7)` | `raw` | `0x7FFACC58E950` | `0xAE950` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 57 48 83 EC 60 80 39 00 45 8B D9` |
| `ComputeWorkItemsToSetupStaticCombosForMode` | `char __fastcall ComputeWorkItemsToSetupStaticCombosForMode(unsigned __int16 *a1, unsigned int a2, int *a3)` | `stringref` | `0x7FFACC4F5F3C` | `0x15F3C` | `"CMaterialLayer::ComputeWorkItemsToSetupStaticCombosForMode(3154): Failed call to FindOrLoadStaticComboData()!\n"` |
| `CreateCommandBuffer` | `__int64 __fastcall CreateCommandBuffer(__int64 a1, __int64 a2, int a3, int a4, _DWORD *a5)` | `stringref` | `0x7FFACC4F9820` | `0x19820` | `"\nCMaterialLayer::CreateCommandBuffer(4446): Find a graphics programmer! Trying to bind a "%s" shader that doesn't exist! for %s\n"` |
| `CreateMaterial` | `void* __fastcall CreateMaterial(void* a1, void** a2, const char* a3, void* a4, void* a5, char a6)` | `raw` | `0x7FFACC51C090` | `0x3C090` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 56 48 81 EC 10 01 00 00 48 8B 05 ? ? ? ? 4C 8B F2` |
| `DynamicShaderCompile` | `char __fastcall DynamicShaderCompile(__int64 a1, __int64 a2)` | `stringref` | `0x7FFACC4F3FA0` | `0x13FA0` | `"CompileComboAndGetVariables_DynamicShaderCompile(), C:\buildworker\csgo_rel_win64\build\src\materialsystem2\material2.cpp:2786"` |
| `FindOrCreateStaticComboDataInCache` | `__int64 __fastcall FindOrCreateStaticComboDataInCache(__int64 a1, __int64 a2)` | `stringref` | `0x7FFACC58E0E0` | `0xAE0E0` | `"CVfxProgramData::FindOrCreateStaticComboDataInCache(4448): Error! Ref count !=0 for static combo data cache entry!\n"` |
| `FindOrLoadStaticComboData` | `__int64 __fastcall FindOrLoadStaticComboData(__int64 a1, __int64 a2, __int64 a3, __int64 a4, char a5)` | `stringref` | `0x7FFACC59DAE0` | `0xBDAE0` | `"Shader %s attribute "%s" has inconsistent value or type across multiple shaders of a feature combo! ["` |
| `FindParameter` | `__int64 __fastcall FindParameter(__int64 a1, __int64 a2)` | `raw` | `0x7FFACC4F1E30` | `0x11E30` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 20 48 8B 59 20 48` |
| `FrameUpdate` | `__int64 __fastcall FrameUpdate(__int64 *a1)` | `raw` | `0x7FFACC51BAC0` | `0x3BAC0` | `48 89 4C 24 08 55 53 56 57 41 54 41 56 48 8B EC 48 83 EC 68 48 8D 05 ? ? ? ? 48 C7 45 C0` |
| `GetErrorMaterial` | `__int64 __fastcall GetErrorMaterial(__int64 a1, __int64 a2, __int64 a3, _QWORD *a4, char a5)` | `stringref` | `0x7FFACC4F74D7` | `0x174D7` | `"CMaterialSystem2::GetErrorMaterial(529): GetErrorMaterial() called when m_pMaterialTypeManager == NULL!\n"` |
| `GetMode` | `__int64 __fastcall GetMode(__int64 a1, unsigned int *a2)` | `raw` | `0x7FFACC4EBD40` | `0xBD40` | `48 89 5C 24 18 57 48 83 EC 30 8B 02 48 8B D9 39 05 ? ? ? ? 48 8B 0D ? ? ? ? 48 89 74 24` |
| `GetVertexShaderInputSignature` | `__int64 __fastcall GetVertexShaderInputSignature(__int64 a1)` | `raw` | `0x7FFACC4EC8C0` | `0xC8C0` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 56 48 83 EC 30 F6 41 0B 01 4C 8B` |
| `Init` | `__int64 __fastcall Init(__int64 a1)` | `stringref` | `0x7FFACC516E40` | `0x36E40` | `"MaterialSystem2"` |
| `LoadShadersAndSetupModes` | `__int64 __fastcall LoadShadersAndSetupModes(__int64 a1, __int64 a2, unsigned int a3)` | `raw` | `0x7FFACC4F0040` | `0x10040` | `44 89 44 24 18 48 89 54 24 10 53 56 41 54 41 55 48 81 EC 88 00 00 00 4C 8B E9 48 C7 44 24 60` |
| `PrepareSceneMaterial` | `float __fastcall PrepareSceneMaterial(__int64 a1, __int64 a2, float a3)` | `raw` | `0x7FFACC4F1BE0` | `0x11BE0` | `48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 30 48 8B 59 ? 48 8B F2 48 63 79 ? 48 C1 E7 06` |
| `ProcessQueue` | `void __fastcall ProcessQueue(__int64 a1)` | `stringref` | `0x7FFACC51A5E0` | `0x3A5E0` | `"Compiling %i shaders:"` |
| `ReloadAndSync` | `void ReloadAndSync()` | `raw` | `0x7FFACC5155C1` | `0x355C2` | `48 83 EC 20 48 8B 35 ? ? ? ? 48 8B CE E8 ? ? ? ? 48 8B CE E8 ? ? ? ? 80 BE A0 03 00 00 00 74 ?` |
| `SetVariableAndRenderState` | `` | `stringref` | `0x7FFACC50F9B0` | `0x2F9B0` | `"SetRenderStateValueFromVariable(1172): Unsupported render state type in material "%s"!\n"` |
| `UnloadAllMaterials` | `__int64 __fastcall UnloadAllMaterials(__int64 a1)` | `stringref` | `0x7FFACC519AA0` | `0x39AA0` | `"CMaterialSystem2::DynamicShaderCompile_UnloadAllMaterials(1084): ERROR!!! Shaders not freed before shader reload! (See spew above)\n\n"` |
| `UpdateParameter` | `_QWORD *__fastcall UpdateParameter(__int64 a1)` | `raw` | `0x7FFACC4F2370` | `0x12370` | `48 89 7C 24 ? 41 56 48 83 EC ? 8B 81` |

## `networksystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `Init` | `` | `raw` | `0x7FFA8035CED0` | `0xECED0` | `40 55 53 57 41 54 41 55 41 57 48 8D AC 24 98 FC FF FF 48 81 EC 68 04 00 00 4C 8B E9` |
| `ProcessMessages` | `` | `raw` | `0x7FFA8032C090` | `0xBC090` | `48 8B C4 53 57 41 54 41 56 48 81 EC A8 00 00 00 48 89 70 D0 45 33 E4 4C 89 68 C8 48 8B D9 48 89` |
| `RegisterNetMessageHandlerAbstract` | `` | `raw` | `0x7FFA8032CA10` | `0xBCA10` | `48 89 5C 24 10 48 89 6C 24 18 57 41 56 41 57 48 83 EC 50 4C 8B B4 24 90 00 00 00 41 8B D9` |
| `SendNetMessage` | `` | `raw` | `0x7FFA8032E480` | `0xBE480` | `48 89 5C 24 10 48 89 6C 24 18 56 57 41 56 48 83 EC 40 41 0F B6 F0 48 8D 99 F8 73 00 00 4C 8B F2` |

## `panorama.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `DispatchEvent` | `void __fastcall DispatchEvent(int *a1, unsigned __int8 a2, __int64 a3)` | `raw` | `0x7FFA75157100` | `0x97100` | `48 8B C4 48 89 58 18 88 50 10 55 56 57 41 54 41 55 41 56 41 57 48 8D A8 78 F7 FF FF 48 81 EC 50` |
| `GetPanelPointerFunctionPointer` | `` | `raw` | `0x7FFA7516B5F0` | `0xAB5F0` | `4C 63 0A 4C 8B DA` |
| `MakeSymbolFunctionPointer` | `` | `raw` | `0x7FFA75153FF0` | `0x93FF0` | `40 55 56 48 83 EC ? 48 63` |
| `OnDeletePanelFunctionPointer` | `` | `raw` | `0x7FFA7516B240` | `0xAB240` | `48 85 D2 0F 84 ? ? ? ? 48 89 ? 24 ? 57 48 83 EC ? 48` |
| `RegisterEventHandlerFunctionPointer` | `` | `raw` | `0x7FFA7516B950` | `0xAB950` | `48 89 5C 24 ? 66 89 54 24 ? 55 56 57 41 56 41 57 48 83 EC ? 48 8D 05 ? ? ? ? 48 C7 44 24 28 ? ? ? ? 48 89 44 24 ? 4D` |
| `RunFrame` | `__int64 __fastcall RunFrame(_QWORD *a1)` | `raw` | `0x7FFA751683D0` | `0xA83D0` | `48 89 5C 24 10 48 89 6C 24 18 56 57 41 54 41 56 41 57 48 81 EC 80 00 00 00 45 33 F6 48 8B F1` |
| `RunScript` | `` | `raw` | `0x7FFA75165E00` | `0xA5E00` | `48 89 5C 24 ? 4C 89 4C 24 ? 48 89 54 24 ? 55 56 57 41 54 41 55 41 56 41 57 48 8D` |
| `RunScriptFunctionPointer` | `` | `raw` | `0x7FFA75165E00` | `0xA5E00` | `48 89 5C 24 ? 4C 89 4C 24 ? 48 89 54 24 ? 55 56 57 41 54 41 55 41 56 41 57 48 8D` |

## `particles.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `CParticleSystemMgr_CreateParticleCollection` | `__int64 __fastcall CParticleSystemMgr_CreateParticleCollection(__int64 a1, const char *a2, __int64 a3, __int64 a4, char a5, int a6, int a7)` | `raw` | `0x7FFA7A5E0DD0` | `0xA0DD0` | `4C 8B DC 49 89 5B 10 49 89 6B 18 49 89 73 20 57 41 56 41 57 48 81 EC 80 00 00 00 49 C7 43 B0 ? ? 00 00 48 8D 05 ? ? ? ? 49 89 43 A8` |
| `CParticleSystemMgr_FindParticleSystem` | `__int64 *__fastcall CParticleSystemMgr_FindParticleSystem(__int64 a1, __int64 *a2, const char *a3, char a4)` | `raw` | `0x7FFA7A5E0BC0` | `0xA0BC0` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 57 41 56 41 57 48 81 EC 40 01 00 00 48 8D 05 ? ? ? ? 48 C7 44 24 28 ? ? 00 00 48 89 44 24 20` |
| `DrawArray` | `_BYTE *__fastcall DrawArray(__int64 a1, __int64 a2, __int64 a3, int a4, __int64 a5, __int64 a6, __int64 a7)` | `raw` | `0x7FFA7A5620B0` | `0x220B0` | `40 55 53 56 57 48 8D 6C 24` |
| `FindKeyVar` | `__int64 __fastcall FindKeyVar(const char *a1, unsigned int a2, int a3)` | `raw` | `0x7FFA7A57A650` | `0x3A650` | `48 89 5C 24 ? 57 48 81 EC ? ? ? ? 33 C0 8B DA` |
| `SetMaterialShaderType` | `void __fastcall SetMaterialShaderType(__int64 a1, int *a2)` | `raw` | `0x7FFA7A5DD8D0` | `0x9D8D0` | `48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 54 41 56 41 57 48 81 EC ? ? ? ? 4C 63 32` |

## `rendersystemdx11.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `BeginSubmittingDisplayLists` | `` | `stringref` | `0x7FFA8207C430` | `0x3C430` | `"CRenderDeviceDx11::BeginSubmittingDisplayLists(1162): "` |
| `CompileShaderSourceMain` | `` | `stringref` | `0x7FFA8207FA40` | `0x3FA40` | `"Shader compilation failed! Reported no errors.\n"` |
| `CreateConstantBuffer` | `` | `stringref` | `0x7FFA8206F500` | `0x2F500` | `"CRenderDeviceBase::CreateConstantBuffer(1571): "` |
| `QueuePresentAndWait` | `` | `raw` | `0x7FFA82074650` | `0x34650` | `40 55 53 57 41 54 41 55 48 8D 6C 24 C9 48 81 EC C0 00 00 00 48 8D 05 ? ? ? ? 4C 89 B4 24` |
| `ResizeBuffers` | `` | `raw` | `0x7FFA8207DC70` | `0x3DC70` | `48 8B C4 55 53 56 57 41 54 48 8B EC 48 83 EC 70 4C 89 68 10 4D 8B E0 4C 89 70 18 4C 8B EA 4C 89` |
| `SetHardwareGammaRamp` | `` | `raw` | `0x7FFA8207F6E0` | `0x3F6E0` | `48 89 5C 24 18 57 B8 B0 40 00 00 E8 ? ? ? ? 48 2B E0 0F 29 BC 24 90 40 00 00 0F 57 C9 0F 28` |
| `SetMode` | `` | `raw` | `0x7FFA82079930` | `0x39930` | `44 89 4C 24 20 44 89 44 24 18 89 54 24 10 55 53 56 57 41 54 41 55 41 56 41 57 48 81 EC D8 02 00` |

## `resourcesystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `BlockingLoadResourceByName` | `` | `raw` | `0x7FFB08F37360` | `0x17360` | `40 53 55 57 48 81 EC 80 00 00 00 48 8B 01 49 8B E8 48 8B FA 48 8B D9 FF 90 98 01 00 00 83 F8 03` |
| `FindOrRegisterResourceByName` | `` | `raw` | `0x7FFB08F36D80` | `0x16D80` | `48 89 5C 24 18 48 89 74 24 20 57 48 81 EC A0 00 00 00 F7 02 FF FF FF 3F 41 0F B6 F8 48 8B DA 48` |
| `FrameUpdate` | `` | `raw` | `0x7FFB08F3C010` | `0x1C010` | `44 88 4C 24 20 44 89 44 24 18 89 54 24 10 55 56 41 54 41 55 41 56 48 8D 6C 24 A0 48 81 EC 60 01` |

## `scenesystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `AllocateAttributeListFunctionPointer` | `` | `raw` | `0x7FFA7ADE7D00` | `0xC7D00` | `40 55 48 83 EC ? 48 83 BA` |
| `CAnimatableSceneObjectDescRender` | `` | `raw` | `0x7FFA7AD75B70` | `0x55B70` | `48 8B C4 53 57 41 54` |
| `CreateStaticShape` | `` | `raw` | `0x7FFA7ADD1A70` | `0xB1A70` | `48 8B C4 48 89 48 08 55 41 54 41 56 48 8D 68 D8 48 81 EC 10 01 00 00 4C 8B 65 50 48 8D 4D 80` |
| `CullView` | `` | `stringref` | `0x7FFA7AE09270` | `0xE9270` | `"CSceneSystem::Thread_CullView(), C:\buildworker\csgo_rel_win64\build\src\scenesystem\scenesystem.cpp:3312"` |
| `DeleteObjectForReal` | `` | `raw` | `0x7FFA7ADEA530` | `0xCA530` | `40 53 56 41 54 48 83 EC 50 0F B6 82 9B 00 00 00 45 33 E4 48 8B DA 48 8B F1 F0 FF 8C 81 CC 30 00 00` |
| `DeleteSceneObjectFunctionPointer` | `` | `raw` | `0x7FFA7ADEB430` | `0xCB430` | `48 85 D2 0F 84 ? ? ? ? 48 8B C4 48 89 50` |
| `Dispatch` | `` | `raw` | `0x7FFA7AE0DD00` | `0xEDD00` | `48 8B C4 48 89 48 08 55 53 56 57 41 54 41 55 41 56 41 57 48 8D A8 B8 FE FF FF 48 81 EC 08 02 00` |
| `DrawAggeregateObject` | `` | `raw` | `0x7FFA7AE4CE30` | `0x12CE30` | `48 8B C4 4C 89 48 20 4C 89 40 ? 48 89 50 ? 55 53 41 57 48 8D A8` |
| `DrawAggregateSceneObjectArray` | `` | `raw` | `0x7FFA7AD5BCB0` | `0x3BCB0` | `48 8B C4 48 89 50 ? 48 89 48 ? 55 53 56 57 41 54 41 55 41 56 41 57 48 8D A8 ? ? ? ? 48 81 EC ? ? ? ? 0F 29 70` |
| `DrawArrayLight` | `` | `raw` | `0x7FFA7AD9AA40` | `0x7AA40` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 54 24` |
| `DrawObject_legacy` | `` | `raw` | `0x7FFA7AD75B70` | `0x55B70` | `48 8B C4 53 57 41 54 48 81 EC D0 00 00 00 49 63 F9 49` |
| `FrameUpdate` | `` | `raw` | `0x7FFA7AE01C30` | `0xE1C30` | `48 8B C4 88 50 10 48 89 48 08 55 53 41 54 41 55 48 8D 68 A1 48 81 EC 98 00 00 00` |
| `InitGfxObjects` | `` | `raw` | `0x7FFA7ADD3DB0` | `0xB3DB0` | `40 55 53 56 57 41 54 41 55 41 56 41 57 48 8D AC 24 08 FE FF FF 48 81 EC F8 02 00 00` |
| `RenderSceneDrawList` | `` | `raw` | `0x7FFA7AE0D9B0` | `0xED9B0` | `40 55 53 56 57 41 54 41 55 41 56 41 57 48 8D 6C 24 E1 48 81 EC D8 00 00 00 4C 8B 71 28 48 8B D9` |
| `ToneMapUpdate` | `` | `raw` | `0x7FFA7AEA6DC0` | `0x186DC0` | `40 53 48 83 EC ? 48 8B D9 0F 29 74 24` |
| `UpdateLightObject` | `` | `raw` | `0x7FFA7AEB8FF0` | `0x198FF0` | `48 89 54 24 ? 55 57 41 56 48 83 EC` |

## `schemasystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `InstallSchemaBindings` | `` | `raw` | `0x7FFAFBAE75D0` | `0x375D0` | `40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? 85 C0 74 08 32 C0` |
| `RegisterModuleAndBuiltins` | `` | `raw` | `0x7FFAFBAC06F0` | `0x106F0` | `48 89 54 24 10 53 56 57 41 55 41 56 41 57 48 83 EC 48 45 33 ED 49 63 C0 33 FF 44 89 AC 24 90 00` |
| `VerifySchemaBindingConsistency` | `` | `raw` | `0x7FFAFBAB58F0` | `0x58F0` | `88 54 24 10 55 53 57 41 54 41 55 48 8B EC 48 81 EC 80 00 00 00 65 48 8B 04 25 58 00 00 00` |

## `server.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `AcceptInput` | `` | `raw` | `0x7FFA73BB2B10` | `0x1212B10` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 49 8B F0 48 8B D9 48 8B 0D` |
| `AddEntityIOEvent` | `` | `raw` | `0x7FFA73B8CB30` | `0x11ECB30` | `48 89 5C 24 18 4C 89 4C 24 20 48 89 4C 24 08 55 56 57 41 54 41 55 41 56 41 57 48 83 EC 20` |
| `BotNavIgnore` | `` | `raw` | `0x7FFA72C659BA` | `0x2C59BA` | `0F 84 ? ? ? ? 80 B8 ? ? ? ? 00 0F 84 ? ? ? ? 80 3D ? ? ? ? 00 74 15` |
| `CCSGameRules__sm_mapGcBanInformation` | `` | `raw` | `0x7FFA7323AF87` | `0x89AF87` | `48 8D 0D ? ? ? ? 48 89 45 ? 0F 11 45` |
| `CTakeDamageInfo` | `` | `raw` | `0x7FFA737EB090` | `0xE4B090` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC ? 45 33 F6 48 C7 41` |
| `CanAcquire` | `` | `raw` | `0x7FFA733D12C0` | `0xA312C0` | `44 89 44 24 ? 48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 55 41 56 41 57 48 8B EC` |
| `CanUse` | `` | `raw` | `0x7FFA73411C00` | `0xA71C00` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 54 41 55 41 56 41 57 48 83 EC ? 48 8B 01 48 8B FA` |
| `CheckSteamBan` | `` | `raw` | `0x7FFA7326CA90` | `0x8CCA90` | `41 54 48 81 EC ? ? ? ? BA ? ? ? ? 48 8D 0D ? ? ? ? E8 ? ? ? ? 48 85 C0` |
| `CheckTransmit` | `` | `raw` | `0x7FFA736893D0` | `0xCE93D0` | `48 8B C4 4C 89 48 ? 48 89 50 ? 48 89 48 ? 55 48 8D A8 ? ? ? ? 48 81 EC ? ? ? ? 48 89 58 ? 48 89 70` |
| `ClientPrint` | `` | `raw` | `0x7FFA732C3A20` | `0x923A20` | `48 85 C9 0F 84 ? ? ? ? 48 89 5C 24 ? 55` |
| `ClientPrintAll` | `` | `raw` | `0x7FFA73814800` | `0xE74800` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 8B E9 49 8B D9` |
| `CreateEntityByName` | `` | `raw` | `0x7FFA734EC000` | `0xB4C000` | `48 83 EC 48 C6 44 24 30 00` |
| `DispatchParticleEffect` | `` | `raw` | `0x7FFA73548DD0` | `0xBA8DD0` | `48 89 5C 24 08 48 89 74 24 10 48 89 7C 24 18 4C 89 74 24 20 55 48 8D 6C 24 D1` |
| `DispatchSpawn` | `` | `raw` | `0x7FFA735A0E40` | `0xC00E40` | `48 89 5C 24 ? 57 48 83 EC ? 48 8B DA 48 8B F9 48 85 C9 0F 84 ? ? ? ? 48 85 D2` |
| `EmitSoundFilter` | `` | `raw` | `0x7FFA737AD1D0` | `0xE0D1D0` | `40 53 48 83 EC ? 4C 89 4C 24 ? 48 8B D9 45 8B C8` |
| `EmitSoundParams` | `` | `raw` | `0x7FFA735F2E10` | `0xC52E10` | `48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 55 48 8B EC 48 81 EC ? ? ? ? 33 C0` |
| `EndTouch` | `` | `raw` | `0x7FFA73CB7910` | `0x1317910` | `40 53 41 55 48 83 EC ? 83 BA` |
| `EquipWeapon` | `` | `raw` | `0x7FFA73415F40` | `0xA75F40` | `48 89 5C 24 ? 57 48 83 EC ? 48 83 79 ? ? 48 8B FA 48 8B D9 75 ? E8 ? ? ? ? 48 8B 53` |
| `FindEntityByClassName` | `` | `raw` | `0x7FFA734F9CE0` | `0xB59CE0` | `48 83 EC 68 45 33 C9` |
| `FindEntityByName` | `` | `raw` | `0x7FFA734FA220` | `0xB5A220` | `48 81 EC 88 ? ? ? 4D 85 C0` |
| `FindUseEntity` | `` | `raw` | `0x7FFA73417280` | `0xA77280` | `4C 89 44 24 ? F3 0F 11 4C 24 ? 55 53 56` |
| `FireOutputInternal` | `` | `raw` | `0x7FFA73BA4C60` | `0x1204C60` | `4C 89 4C 24 ? 48 89 4C 24 ? 53 56` |
| `GameSystems` | `` | `raw` | `0x7FFA72EC8FF6` | `0x528FF6` | `8B 05 ? ? ? ? 83 E8 ? 48 63 F8 0F 88` |
| `GetCSWeaponDataFromKey` | `` | `raw` | `0x7FFA72EC8530` | `0x528530` | `48 89 5C 24 ? 57 48 83 EC ? 33 FF 4C 8B CA 8B D9` |
| `GetEyeAngles` | `` | `raw` | `0x7FFA7345A450` | `0xABA450` | `48 89 5C 24 ? 57 48 81 EC ? ? ? ? 48 8B F9 48 8B DA 48 8B 89 ? ? ? ? 48 85 C9` |
| `GetSpawnGroups` | `` | `raw` | `0x7FFA72F0D430` | `0x56D430` | `40 56 48 83 EC ? 48 89 5C 24 ? 48 8D B1` |
| `GiveNamedItem` | `__int64 __fastcall GiveNamedItem(__int64 a1, const char *a2, int a3, __int64 a4, char a5, unsigned __int64 *a6)` | `raw` | `0x7FFA733D04C0` | `0xA304C0` | `48 89 5C 24 08 48 89 74 24 10 48 89 7C 24 20 44 89 44 24 18 55 41 54 41 55 41 56 41 57 48 8D AC 24 40 FF FF FF 48 81 EC C0 01 00 00 4D 8B E1 45 8B E8` |
| `GravityTouch` | `` | `raw` | `0x7FFA737FBC60` | `0xE5BC60` | `48 89 5C 24 ? 57 48 83 EC ? 48 8B 02 48 8B F9 48 8B CA 48 8B DA FF 90 ? ? ? ? 84 C0 74 ? F3 0F 10 8F` |
| `IGameSystem_InitAllSystems_pFirst` | `` | `raw` | `0x7FFA72EC38B3` | `0x5238B3` | `48 8B 1D ? ? ? ? 48 85 DB 0F 84 ? ? ? ? BD` |
| `IGameSystem_LoopPostInitAllSystems_pEventDispatcher` | `` | `raw` | `0x7FFA72EC9719` | `0x529719` | `48 39 1D ? ? ? ? 74 ? 39 05` |
| `Init` | `` | `raw` | `0x7FFA735020C0` | `0xB620C0` | `40 53 48 83 EC ? 48 8B 01 48 8B D9 FF 50 ? 48 8B 03` |
| `InputTestActivator` | `` | `raw` | `0x7FFA734B08B0` | `0xB108B0` | `48 89 5C 24 ? 57 48 83 EC ? 4C 8B 02` |
| `InputTriggerForActivatedPlayer` | `` | `raw` | `0x7FFA736BB360` | `0xD1B360` | `48 89 5C 24 18 56 48 83 EC 20 48 8B 1A` |
| `InputTriggerForAllPlayers` | `` | `raw` | `0x7FFA736BB440` | `0xD1B440` | `40 55 53 41 54 41 56 48 8B EC 48 83 EC ? 4C 8B F1` |
| `LegacyGameEventListener` | `` | `raw` | `0x7FFA734FEEF0` | `0xB5EEF0` | `48 8B 15 ? ? ? ? 48 85 D2 74 ? 83 F9 ? 77` |
| `NetworkStateChanged` | `` | `raw` | `0x7FFA73BB6BA0` | `0x1216BA0` | `4C 8B C2 48 8B D1 48 8B 09` |
| `PostThink` | `` | `raw` | `0x7FFA72BA18A0` | `0x2018A0` | `40 55 53 56 57 41 54 48 8D 6C 24 ? 48 81 EC ? ? ? ? 4C 89 AC 24` |
| `ProcessUsercmds` | `` | `raw` | `0x7FFA7346FB80` | `0xACFB80` | `48 8B C4 44 88 48 20 44 89 40 18 48 89 50 10 53` |
| `Remove` | `` | `raw` | `0x7FFA73530CB0` | `0xB90CB0` | `48 85 C9 74 ? 48 8B D1 48 8B 0D ? ? ? ?` |
| `RemovePlayerItem` | `` | `raw` | `0x7FFA734146D0` | `0xA746D0` | `48 85 D2 0F 84 ? ? ? ? 48 89 5C 24 ? 57 48 83 EC ? 48 8B DA 48 8B F9 E8` |
| `Say` | `` | `raw` | `0x7FFA735DE940` | `0xC3E940` | `44 89 4C 24 ? 44 88 44 24` |
| `SayText2Filter` | `` | `raw` | `0x7FFA73815AE0` | `0xE75AE0` | `48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 55 41 56 41 57 48 8D 6C 24 ? 48 81 EC ? ? ? ? 45 0F B6 F0` |
| `SayTextFilter` | `` | `raw` | `0x7FFA73815F70` | `0xE75F70` | `48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 55 41 56 41 57 48 8D 6C 24 ? 48 81 EC ? ? ? ? 49 8B F8` |
| `SetEntityName` | `` | `raw` | `0x7FFA73BB8100` | `0x1218100` | `48 89 5C 24 10 57 48 83 EC 20 48 8B D9 4C 8B C2` |
| `SetGravityScale` | `` | `raw` | `0x7FFA72D8E580` | `0x3EE580` | `48 89 5C 24 ? 57 48 83 EC ? F3 0F 10 81 ? ? ? ? 48 8B F9 0F 29 74 24 ? 0F 28 F1 0F 2E C6 7A ? 74` |
| `SetGroundEntity` | `` | `raw` | `0x7FFA736FDFC0` | `0xD5DFC0` | `48 89 5C 24 ? 55 56 57 41 55 41 57 48 83 EC ? 44 8B 89` |
| `SetModel` | `` | `raw` | `0x7FFA7347A580` | `0xADA580` | `40 53 48 83 EC ? 48 8B D9 4C 8B C2 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 50 ? 48 8B 54 24 ? 48 8B CB E8 ? ? ? ? 48 83 C4 ? 5B C3` |
| `SetMoveType` | `` | `raw` | `0x7FFA72D8F080` | `0x3EF080` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC ? 41 0F B6 F0` |
| `SetOrAddAttributeValueByName` | `` | `raw` | `0x7FFA738995D0` | `0xEF95D0` | `40 53 55 41 56 48 81 EC 90 00 00 00` |
| `SetPawn` | `` | `raw` | `0x7FFA7347CE60` | `0xADCE60` | `44 88 4C 24 ? 53 57 41 54 41 56 41 57 48 83 EC` |
| `StartTouch` | `` | `raw` | `0x7FFA72D92120` | `0x3F2120` | `40 57 41 56 48 83 EC ? 48 8B 01` |
| `SwitchTeam` | `__int64 __fastcall SwitchTeam(__int64 a1, unsigned int a2)` | `raw` | `0x7FFA733B3150` | `0xA13150` | `40 53 57 48 81 EC 88 00 00 00 48 8B D9 8B FA 8B CA E8 ? ? ? ? 48 85 C0 0F 84 3A 02 00 00` |
| `TerminateRound` | `_BYTE *__fastcall TerminateRound(__int64 a1, __int64 a2, unsigned int a3, __int64 a4)` | `raw` | `0x7FFA73294FC0` | `0x8F4FC0` | `48 8B C4 4C 89 48 20 48 89 48 08 55 56 41 56 41 57 48 8D 68 A1 48 81 EC E8 00 00 00 4C 8D B1` |
| `Think` | `double __fastcall Think(__int64 a1)` | `raw` | `0x7FFA7327D5F0` | `0x8DD5F0` | `40 55 53 41 55 41 57 48 8D 6C 24 C1 48 81 EC A8 00 00 00 80 79 48 00 4C 8B F9 4C 8B 2D` |
| `Touch` | `` | `raw` | `0x7FFA73813D40` | `0xE73D40` | `40 55 53 57 48 8D 6C 24 ? 48 81 EC ? ? ? ? 48 8B 02 48 8B F9` |
| `TraceFunc` | `` | `raw` | `0x7FFA734D8B40` | `0xB38B40` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 54 41 56 41 57 48 81 EC ? ? ? ? 45 33 E4` |

## `soundsystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `PlayVSND` | `` | `raw` | `0x7FFA7EB1A1D0` | `0x2A1D0` | `48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 55 48 8D 6C 24 ? 48 81 EC ? ? ? ? 33 F6 C7 45 ? ? ? ? ? 4C 8B C1` |
| `PlayVSound` | `_UNKNOWN **__fastcall PlayVSound(__int64 a1, __int64 a2, int a3, int a4)` | `raw` | `0x7FFA7EE39830` | `0x349830` | `48 8B C4 48 89 58 08 57 48 81 EC ? ? ? ? 33 FF 48 8B D9` |
| `SomeUtlSymbolFunc` | `__int64 __fastcall SomeUtlSymbolFunc(__int64 a1, unsigned int a2)` | `raw` | `0x7FFA7EBA0730` | `0xB0730` | `48 89 74 24 18 57 48 83 EC 20 48 63 F2 48 8B F9 3B 71 30` |

## `tier0.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `CreateInterface` | `void *__fastcall CreateInterface(const char *pName, int *pReturnCode)` | `raw` | `0x7FFA9DE60C40` | `0x210C40` | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 2E 49 8B 41 08 4D 8B C3 4C 2B C0` |
| `LoadKV3` | `` | `raw` | `0x7FFA9DD78F30` | `0x128F30` | `48 89 5C 24 08 57 48 83 EC 70 4C 8B D1 48 C7 C0 FF FF FF FF 48 FF C0 41 80 3C 00 00 75 F6` |
| `LoadKeyValues` | `` | `rel32` | `0x7FFA9DD79000` | `0x129000` | `E8 ? ? ? ? 8B 4C 24 34 0F B6 D8` |
| `Plat_FloatTime` | `double __fastcall Plat_FloatTime()` | `raw` | `0x7FFA9DD96BA0` | `0x146BA0` | `48 83 EC 28 48 83 3D ? ? ? ? 00 75 05 E8 ? ? ? ? 48 8D 4C 24 30 FF 15 ? ? ? ? 48 8B 4C 24 30 48 8B 05 ? ? ? ? 48 3B C8 73 05 48 8B C8 EB 07 48 89 0D ? ? ? ? 48 2B 0D ? ? ? ? 0F 57 C0 78 12` |
| `Plat_GetTime` | `unsigned __int64 __fastcall Plat_GetTime()` | `raw` | `0x7FFA9DD969E0` | `0x1469E0` | `48 83 EC 28 48 8D 4C 24 30 E8 ? ? ? ? 48 8B 44 24 30 48 83 C4 28 C3` |
| `Plat_MSTime` | `unsigned __int64 __fastcall Plat_MSTime()` | `raw` | `0x7FFA9DD96C20` | `0x146C20` | `40 53 48 83 EC 20 48 8B 1D ? ? ? ? 48 85 DB 75 0C E8 ? ? ? ? 48 8B 1D ? ? ? ? 48 8D 4C 24 30 FF 15 ? ? ? ? 48 8B 44 24 30 48 8B 0D ? ? ? ? 48 3B C1 73 05 48 8B C1 EB 07 48 89 05 ? ? ? ? 48 2B 05 ? ? ? ? 33 D2 48 F7 F3 48 8B C8 48 69 C2 E8 03 00 00 69 C9 E8 03 00 00` |
| `UtlBuffer` | `` | `raw` | `0x7FFA9DCA3F10` | `0x53F10` | `48 89 5C 24 ? 57 48 83 EC ? 8B 41 ? 8D 7A` |

