# CS2 Signatures

_This file is regenerated on every successful run of `cs2-sdk`._

**587/633 signatures resolved across 16 module(s).**

## `animationsystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `FrameUpdate` | `void __fastcall FrameUpdate(__int64 a1)` | `raw` | `0x7FFC2432B480` | `0x8B480` | `48 89 4C 24 08 55 53 56 57 41 54 41 55 41 56 41 57 48 8D AC 24 C8 EB FF FF B8 38 15 00 00` |
| `ShouldUpdateSequences` | `__int64 __fastcall ShouldUpdateSequences(__int64 a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFC243EEFF0` | `0x14EFF0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 20 49 8B 40 48` |

## `client.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `ActionTrackingServices` | `__int64 __fastcall ActionTrackingServices(int a1, _QWORD *a2)` | `stringref` | `0x7FFC154CDAC0` | `0x80DAC0` | `"CCSPlayerController_ActionTrackingServices"` |
| `AddListener` | `__int64 __fastcall AddListener(__int64 a1, __int64 a2, const char *a3, unsigned __int8 a4)` | `raw` | `0x7FFC15622630` | `0x962630` | `48 89 5C 24 10 48 89 6C 24 18 56 57 41 56 48 83 EC 50 41 0F B6 E9 48 8D 99 E0 00 00 00 49 8B F0` |
| `AddNametagEntity` | `char __fastcall AddNametagEntity(__int64 a1, __int64 a2)` | `raw` | `0x7FFC154734A0` | `0x7B34A0` | `40 55 53 56 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 48 8B DA` |
| `AddStattrakEntity` | `void __fastcall AddStattrakEntity(__int64 a1, unsigned int a2)` | `raw` | `0x7FFC15734D70` | `0xA74D70` | `48 8B C4 48 89 58 08 48 89 70 10 57 48 83 EC 50 33 F6 8B FA 48 8B D1` |
| `AddToTail` | `__int64 __fastcall AddToTail(int *a1, __int64 a2)` | `raw` | `0x7FFC15472080` | `0x7B20D2` | `41 B9 88 02 00 00 8B 57 14 81 E2 FF FF FF 3F 8D 71 01 44 8B C6 FF 15` |
| `AnimGraphRebuild` | `__int64 __fastcall AnimGraphRebuild(__int64 a1, char a2)` | `raw` | `0x7FFC15597350` | `0x8D7350` | `40 55 56 48 83 EC 28 4C 89 74 24 58 48 8B F1 80 FA FF 75 04 0F B6 51 18` |
| `ApplyEconCustomization` | `__int64 __fastcall ApplyEconCustomization(__int64 a1, char a2)` | `raw` | `0x7FFC15490ED0` | `0x7D0ED0` | `48 89 5C 24 ? 57 48 83 EC ? 8B FA 48 8B D9 E8 ? ? ? ? 48 8B CB E8 ? ? ? ? 48 85 C0 74` |
| `AutowallInit` | `__int64 __fastcall AutowallInit(__int64 a1)` | `raw` | `0x7FFC155CA510` | `0x90A510` | `40 53 48 83 EC ? 48 8B D9 48 81 C1 ? ? ? ? E8 ? ? ? ?` |
| `AutowallTraceData` | `char __fastcall AutowallTraceData(_QWORD *a1, int *a2, int a3, int a4, _BYTE *a5, int a6)` | `raw` | `0x7FFC15677020` | `0x9B7020` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B 09` |
| `AutowallTracePos` | `char __fastcall AutowallTracePos(__int64 a1, __int64 a2)` | `raw` | `0x7FFC154EFF30` | `0x82FF30` | `40 55 56 41 54 41 55 41 57 48 8B EC` |
| `BuildBoneMergeWork` | `char __fastcall BuildBoneMergeWork(__int64 a1, _QWORD *a2, char a3)` | `raw` | `0x7FFC15628080` | `0x968080` | `40 55 56 57 41 54 41 55 41 56 41 57 48 83 EC 50 48 8D 6C 24 50 80 A1 06 01 00 00 FB 4C 8B F9 80` |
| `BuildTemplateMaterialFromFile` | `CKeyValues_Data *__fastcall BuildTemplateMaterialFromFile(__int64 a1, const char *a2)` | `raw` | `0x7FFC160A6040` | `0x13E6040` | `48 89 54 24 10 55 53 41 55 41 57 48 8D AC 24 18 F9 FF FF 48 81 EC E8 07 00 00 4C 8B FA 48 85 D2` |
| `BulkRegenIterator` | `__int64 __fastcall BulkRegenIterator(char a1)` | `raw` | `0x7FFC154769A1` | `0x7B69A1` | `57 48 83 EC 40 0F B6 F9 E8 ? ? ? ? 48 85 C0 0F 84` |
| `BulletServices` | `void *__fastcall BulletServices(__int64 a1)` | `stringref` | `0x7FFC154FC2C0` | `0x83C2C0` | `"CCSPlayer_BulletServices"` |
| `CAttributeStringFill` | `__int64 __fastcall CAttributeStringFill(__int64 a1, __int64 a2)` | `rel32` | `0x7FFC15B98150` | `0xED8150` | `E8 ? ? ? ? 41 83 CF 08` |
| `CAttributeStringInit` | `_QWORD *__fastcall CAttributeStringInit(_QWORD *a1, __int64 a2, char a3)` | `rel32` | `0x7FFC152B8140` | `0x5F8140` | `E8 ? ? ? ? 48 8D 05 ? ? ? ? 48 89 7D ? 48 89 45 ? 49 8D 4F` |
| `CBodyComponent` | `__int64 CBodyComponent()` | `stringref` | `0x7FFC14E7C7F0` | `0x1BC7F0` | `"CBodyComponent"` |
| `CBodyComponentSkeletonInstance` | `__int64 (__fastcall ***CBodyComponentSkeletonInstance())()` | `stringref` | `0x7FFC14E836D0` | `0x1C36D0` | `"CBodyComponentSkeletonInstance"` |
| `CBufferStringInit` | `char __fastcall CBufferStringInit(__int64 a1, const char *a2)` | `raw` | `0x7FFC164CBAA0` | `0x180BAA0` | `48 89 5C 24 ? 57 48 83 EC ? 8B 41 ? 48 8D 79` |
| `CCSGOHudVote_OnVoteResult` | `void __fastcall CCSGOHudVote_OnVoteResult(__int64 a1, int a2, const char *a3, int a4, __int64 a5)` | `raw` | `0x7FFC15AFD250` | `0xE3D250` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 57 41 56 41 57 48 81 EC 90 01 00 00 65 48 8B 04 25 58 00 00 00 49 8B E8 44 8B 15 ? ? ? ? 8B FA` |
| `CCSGO_HudChat_OnSayText2` | `void __fastcall CCSGO_HudChat_OnSayText2(int a1, __int64 a2)` | `raw` | `0x7FFC15DACD60` | `0x10ECD60` | `48 89 5C 24 08 55 56 57 41 54 41 55 41 56 41 57 48 8D AC 24 70 F3 FF FF 48 81 EC 90 0D 00 00 81 A5 DC 0C 00 00 FF FF 0F FF 33 F6 8B 5A 6C 48 8B` |
| `CCSGameRules` | `_QWORD *CCSGameRules()` | `stringref` | `0x7FFC14D3E690` | `0x7E690` | `"CCSGameRules"` |
| `CCSGameRulesProxy` | `__int64 CCSGameRulesProxy()` | `stringref` | `0x7FFC153D19B0` | `0x7119B0` | `"CCSGameRulesProxy"` |
| `CCSPlayerController` | `__int64 __fastcall CCSPlayerController(int a1, _QWORD *a2)` | `stringref` | `0x7FFC154CDAC0` | `0x80DAC0` | `"CCSPlayerController"` |
| `CCSPlayerPawn` | `__int64 CCSPlayerPawn()` | `stringref` | `0x7FFC15899BA0` | `0xBD9BA0` | `"CCSPlayerPawn"` |
| `CCSPlayer_MovementServices_ValidateVelocity` | `void __fastcall CCSPlayer_MovementServices_ValidateVelocity(__int64 movementServices)` | `stringref` | `0x7FFC1552FF10` | `0x86FF10` | `"CCSPlayer_MovementServices(%s):  %d/%s Got a NaN velocity on %s\n"` |
| `CCSWeaponBase` | `__int64 CCSWeaponBase()` | `stringref` | `0x7FFC15467800` | `0x7A7800` | `"CCSWeaponBase"` |
| `CCSWeaponBaseGun` | `__int64 CCSWeaponBaseGun()` | `stringref` | `0x7FFC154678A0` | `0x7A78A0` | `"CCSWeaponBaseGun"` |
| `CCSWeaponBaseVData` | `const char *CCSWeaponBaseVData()` | `stringref` | `0x7FFC15442760` | `0x782760` | `"CCSWeaponBaseVData"` |
| `CCollisionProperty` | `__int64 __fastcall CCollisionProperty(int a1, __int64 a2, __int64 a3, __int64 a4)` | `stringref` | `0x7FFC14FA1620` | `0x2E1620` | `"CCollisionProperty"` |
| `CDecoyProjectile` | `__int64 CDecoyProjectile()` | `stringref` | `0x7FFC154366D0` | `0x7766D0` | `"CDecoyProjectile"` |
| `CEconItemCreateInstance` | `uintptr_t __cdecl CEconItemCreateInstance()` | `raw` | `0x7FFC15CE0CB0` | `0x1020CB0` | `48 83 EC 28 B9 48 00 00 00 E8` |
| `CFlashbangProjectile` | `__int64 CFlashbangProjectile()` | `stringref` | `0x7FFC15CC9930` | `0x1009930` | `"CFlashbangProjectile"` |
| `CFogController` | `__int64 CFogController()` | `stringref` | `0x7FFC14F3F660` | `0x27F660` | `"CFogController"` |
| `CGameSceneNode` | `__int64 __fastcall CGameSceneNode(int a1, __int64 a2)` | `stringref` | `0x7FFC14E63F80` | `0x1A3F80` | `"CGameSceneNode"` |
| `CGlowProperty` | `__int64 __fastcall CGlowProperty(int a1, __int64 a2, __int64 a3, __int64 a4)` | `stringref` | `0x7FFC14FA1830` | `0x2E1830` | `"CGlowProperty"` |
| `CHEGrenadeProjectile` | `__int64 CHEGrenadeProjectile()` | `stringref` | `0x7FFC15CC99D0` | `0x10099D0` | `"CHEGrenadeProjectile"` |
| `CLegacyGameUI_Initialize` | `__int64 __fastcall CLegacyGameUI_Initialize(__int64 thisptr)` | `stringref` | `0x7FFC1598FA10` | `0xCCFA10` | `"CLegacyGameUI::Initialize() failed to get necessary interfaces\n"` |
| `CMolotovProjectile` | `__int64 CMolotovProjectile()` | `stringref` | `0x7FFC154368B0` | `0x7768B0` | `"CMolotovProjectile"` |
| `CPostProcessingVolume` | `__int64 CPostProcessingVolume()` | `stringref` | `0x7FFC14F643F0` | `0x2A43F0` | `"CPostProcessingVolume"` |
| `CPrediction_Update` | `__int64 __fastcall CPrediction_Update(__int64 thisptr, int reason)` | `raw` | `0x7FFC15836A40` | `0xB76A40` | `48 8B C4 89 50 ? 48 89 48 ? 55 53 57` |
| `CSBaseGunFireData` | `void __fastcall CSBaseGunFireData(__int64 a1)` | `raw` | `0x7FFC161D1730` | `0x1511730` | `48 8B C4 55 53 56 57 41 54 41 55 41 56 41 57 48 8D 68 A8 48 81 EC ? ? ? ? 4C 8B 69` |
| `CSkeletonInstance` | `__int64 __fastcall CSkeletonInstance(int a1, __int64 a2)` | `stringref` | `0x7FFC14E64080` | `0x1A4080` | `"CSkeletonInstance"` |
| `CSmokeGrenadeProjectile` | `__int64 CSmokeGrenadeProjectile()` | `stringref` | `0x7FFC15436950` | `0x776950` | `"CSmokeGrenadeProjectile"` |
| `CTonemapController2` | `__int64 CTonemapController2()` | `stringref` | `0x7FFC14F18320` | `0x258320` | `"CTonemapController2"` |
| `C_AttributeContainer` | `__int64 __fastcall C_AttributeContainer(int a1, _QWORD *a2)` | `stringref` | `0x7FFC15901F20` | `0xC41F20` | `"C_AttributeContainer"` |
| `C_BaseEntity` | `__int64 (__fastcall *C_BaseEntity())()` | `stringref` | `0x7FFC14D0E260` | `0x4E260` | `"C_BaseEntity"` |
| `C_BaseEntity_CheckPredictionForceReLatch` | `__int64 __fastcall C_BaseEntity_CheckPredictionForceReLatch(__int64 a1, __int64 a2)` | `raw` | `0x7FFC15830740` | `0xB70740` | `48 8B C4 48 89 50 10 53 55 56 48 81 EC 00 01 00 00 0F 29 78 98 48 8B F2 8B 91 04 01 00 00` |
| `C_BaseEntity_ProcessInterpolatedList` | `__int64 __fastcall C_BaseEntity_ProcessInterpolatedList(__int64 a1, unsigned int a2, int a3, unsigned int a4)` | `raw` | `0x7FFC157543B0` | `0xA943B0` | `4C 8B DC 49 89 5B 10 49 89 6B 18 49 89 73 20 57 41 54 41 57 48 83 EC 60 49 C7 43 B0 E1 07 00 00` |
| `C_BaseEntity_RestoreData` | `void __fastcall C_BaseEntity_RestoreData(__int64 a1, const char *a2, unsigned int a3, int a4)` | `raw` | `0x7FFC15759BF0` | `0xA99BF0` | `40 55 53 56 41 54 41 57 48 8D AC 24 20 FF FF FF 48 81 EC E0 01 00 00 48 8B D9 45 8B E1 48 8B 89` |
| `C_BaseEntity_SaveData` | `void __fastcall C_BaseEntity_SaveData(_QWORD *a1, const char *a2, __int64 a3, int a4, int a5, unsigned int a6, __int64 a7)` | `raw` | `0x7FFC15759E00` | `0xA99E00` | `48 8B C4 55 56 57 41 56 41 57 48 8D A8 E8 FD FF FF 48 81 EC F0 02 00 00 48 83 B9 A0 05 00 00 00` |
| `C_BaseEntity_StartParticleSystem` | `` | `raw` | `0x7FFC15A8C890` | `0xDCC890` | `48 89 5C 24 08 55 48 8B EC 48 83 EC 40 E8 ? ? ? ? 48 8D 05 ? ? ? ? 33 DB 48 8D 15` |
| `C_BaseModelEntity` | `__int64 __fastcall C_BaseModelEntity(int a1, _QWORD *a2)` | `stringref` | `0x7FFC14E18650` | `0x158650` | `"C_BaseModelEntity"` |
| `C_BasePlayerPawn` | `__int64 (__fastcall *C_BasePlayerPawn())()` | `stringref` | `0x7FFC14D2DA20` | `0x6DA20` | `"C_BasePlayerPawn"` |
| `C_BasePlayerPawn_PrePhysicsSimulate` | `bool __fastcall C_BasePlayerPawn_PrePhysicsSimulate(__int64 pawn)` | `stringref` | `0x7FFC155B7CF0` | `0x8F7CF0` | `"C_BasePlayerPawn::PrePhysicsSimulate"` |
| `C_C4` | `__int64 (__fastcall *C_C4())()` | `stringref` | `0x7FFC14D5A950` | `0x9A950` | `"C_C4"` |
| `C_CSPlayerPawn` | `__int64 __fastcall C_CSPlayerPawn(int a1, _QWORD *a2)` | `stringref` | `0x7FFC153AA8E0` | `0x6EA8E0` | `"C_CSPlayerPawn"` |
| `C_CSPlayerPawnBase` | `__int64 *C_CSPlayerPawnBase()` | `stringref` | `0x7FFC158BFF90` | `0xBFFF90` | `"C_CSPlayerPawnBase"` |
| `C_CSWeaponBase` | `_QWORD *__fastcall C_CSWeaponBase(int a1, _QWORD *a2)` | `stringref` | `0x7FFC1542A660` | `0x76A660` | `"C_CSWeaponBase"` |
| `C_CSWeaponBase_GetEconWpnData` | `__int64 __fastcall C_CSWeaponBase_GetEconWpnData(__int64 a1)` | `raw` | `0x7FFC1547D5C0` | `0x7BD5C0` | `40 53 48 83 EC 40 48 8B D9 E8 ? ? ? ? 48 8B C8 E8 ? ? ? ? 48 85 C0 75 ? 48 8B 43 10` |
| `C_DispatchEffect` | `__int64 __fastcall C_DispatchEffect(const char *name, __int64 data)` | `stringref` | `0x7FFC157B63B0` | `0xAF63B0` | `"DispatchEffect: effect "%s" not found on client\n"` |
| `C_EconEntity_BuildLegacyGloveSkinMaterial` | `void __fastcall C_EconEntity_BuildLegacyGloveSkinMaterial(int *a1)` | `stringref` | `0x7FFC158AA230` | `0xBEA230` | `"MapPlayerPreview gloves"` |
| `C_EconEntity_BuildLegacyWeaponSkinMaterial` | `void __fastcall C_EconEntity_BuildLegacyWeaponSkinMaterial(__int64 a1, char a2)` | `stringref` | `0x7FFC154746D0` | `0x7B46D0` | `"workshop preview weapon"` |
| `C_EconEntity_BuildModernWeaponSkinMaterial` | `void __fastcall C_EconEntity_BuildModernWeaponSkinMaterial(__int64 a1, _QWORD *a2, __int64 a3, int a4, char a5, char a6, __int64 a7)` | `raw` | `0x7FFC15A6E490` | `0xDAE490` | `48 85 C9 0F 84 ? ? 00 00 48 8B C4 48 89 50 10 48 89 48 08 55 41 54 41 56 41 57 48 8D A8 B8 FA` |
| `C_EconEntity_BuildNametagOverlayMaterial` | `char __fastcall C_EconEntity_BuildNametagOverlayMaterial(__int64 a1, __int64 a2)` | `stringref` | `0x7FFC154734A0` | `0x7B34A0` | `"low-res nametag"` |
| `C_EconItemView` | `_QWORD *__fastcall C_EconItemView(int a1, _QWORD *a2)` | `stringref` | `0x7FFC153F3A70` | `0x733A70` | `"C_EconItemView"` |
| `C_EconWearable_OnNewCustomMaterials` | `__int64 __fastcall C_EconWearable_OnNewCustomMaterials(__int64 a1, char a2)` | `stringref` | `0x7FFC15DA25D0` | `0x10E25D0` | `"Invalid EconItemView -- Can't create custom materials for wearable, debug this.\n"` |
| `C_GameRules_ctor` | `__int64 __fastcall C_GameRules_ctor(__int64 thisptr)` | `stringref` | `0x7FFC157ECBC0` | `0xB2CBC0` | `"%s:  CGameRules::CGameRules constructed\n"` |
| `C_Hostage` | `__int64 (__fastcall *C_Hostage())()` | `stringref` | `0x7FFC14DA7B10` | `0xE7B10` | `"C_Hostage"` |
| `C_Inferno` | `__int64 (__fastcall *C_Inferno())()` | `stringref` | `0x7FFC14DB7AD0` | `0xF7AD0` | `"C_Inferno"` |
| `C_PlantedC4` | `__int64 (__fastcall *C_PlantedC4())()` | `stringref` | `0x7FFC14DB0E30` | `0xF0E30` | `"C_PlantedC4"` |
| `C_PlantedC4_ClientThink` | `_DWORD *__fastcall C_PlantedC4_ClientThink(__int64 plantedC4)` | `stringref` | `0x7FFC158F27D0` | `0xC327D0` | `"C4.ExplodeTriggerTrip"` |
| `C_SmokeGrenadeProjectile` | `__int64 (__fastcall *C_SmokeGrenadeProjectile())()` | `stringref` | `0x7FFC14D55F40` | `0x95F40` | `"C_SmokeGrenadeProjectile"` |
| `CacheParticleEffect` | `` | `raw` | `0x7FFC14EC8250` | `0x208250` | `4C 8B DC 53 48 81 EC ? ? ? ? F2 0F 10 05` |
| `CalcSpread` | `` | `raw` | `0x7FFC15968070` | `0xCA8070` | `48 8B C4 48 89 58 ? 48 89 68 ? 48 89 70 ? 57 41 54 41 55 41 56 41 57 48 81 EC ? ? ? ? 4C 63 EA` |
| `CalcViewmodel` | `void __fastcall CalcViewmodel(__int64 a1, float *a2, float *a3)` | `raw` | `0x7FFC15537660` | `0x877660` | `40 55 53 56 41 56 41 57 48 8B EC` |
| `CalcViewmodelTransform_v2` | `__int64 __fastcall CalcViewmodelTransform_v2(__int64 a1, __int64 a2)` | `raw` | `0x7FFC1548A930` | `0x7CA930` | `48 89 5C 24 20 55 56 57 41 54 41 55 41 56 41 57 48 8D 6C 24 80 48 81 EC 80 01 00 00 48 8B FA` |
| `CalcViewmodelView` | `__int64 __fastcall CalcViewmodelView(__int64 a1, __int64 a2, _DWORD *a3)` | `raw` | `0x7FFC159553A0` | `0xC953A0` | `40 53 48 83 EC 60 48 8B 41 08 49 8B D8 8B 48 30 48 C1 E9 0C F6 C1 01 0F 85 48 01 00 00 41 B8 07` |
| `CalculateInterpolation` | `int *__fastcall CalculateInterpolation(__int64 a1, int *a2)` | `rel32` | `0x7FFC161B1460` | `0x14F1460` | `E8 ? ? ? ? 8B 45 ? 3B 45 60 75 04 32 D2 EB 09 BA 01 00 00 00 41 0F 4C D5 C0 EA 07 84 D2 0F 85 87` |
| `CalculateWorldSpaceBones` | `void __fastcall CalculateWorldSpaceBones(__int64 a1, unsigned int a2)` | `raw` | `0x7FFC156F36F0` | `0xA336F0` | `48 89 4C 24 ? 55 53 56 57 41 54 41 55 41 56 41 57 B8 ? ? ? ? E8 ? ? ? ? 48 2B E0 48 8D 6C 24 ? 48 8B 81` |
| `Caller` | `__int64 __fastcall Caller(__int64 a1, const char *a2, __int64 a3, __int64 a4)` | `stringref` | `0x7FFC160A4EC4` | `0x13E4EC4` | `"CCompositeMaterialManager::AddNewPanoramaPanelRenderRequest"` |
| `CameraServices` | `__int64 CameraServices()` | `stringref` | `0x7FFC154F83D0` | `0x8383D0` | `"CCSPlayer_CameraServices"` |
| `ChangeModel` | `__int64 __fastcall ChangeModel(__int64 a1, __int64 a2)` | `raw` | `0x7FFC155C37F0` | `0x9037F0` | `40 53 48 83 EC ? 48 8B D9 4C 8B C2 48 8B 0D ? ? ? ? 48 8D 54 24` |
| `CheckJumpButton` | `void __fastcall CheckJumpButton(__int64 a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFC157B7A90` | `0xAF7A90` | `4C 89 44 24 18 55 56 41 56 48 8D AC 24 70 EC FF FF B8 90 14 00 00` |
| `ClearHUDWeaponIcon` | `__int64 __fastcall ClearHUDWeaponIcon(__int64 a1, int a2, __int64 a3)` | `rel32` | `0x7FFC15AD7300` | `0xE17300` | `E8 ? ? ? ? 8B F8 C6 84 24 ? ? ? ? ?` |
| `Client` | `bool __fastcall Client(_QWORD *a1, __int64 a2, __int64 a3, __int64 a4, _BYTE *a5, __int64 a6)` | `raw` | `0x7FFC15677100` | `0x9B7100` | `48 89 5C 24 20 48 89 4C 24 08 55 57 41 54 41 55 41 56 48 8D AC 24 10 E0 FF FF B8 F0 20 00 00` |
| `ComputeRandomSeed` | `__int64 __fastcall ComputeRandomSeed(__int64 a1, __int64 a2, int a3)` | `raw` | `0x7FFC15967750` | `0xCA7750` | `48 89 5C 24 ? 57 48 81 EC ? ? ? ? ? ? ? ? 48 8D 8C 24` |
| `ConCommand_firstperson` | `__int64 ConCommand_firstperson()` | `raw` | `0x7FFC157B2930` | `0xAF2930` | `48 83 EC 28 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 08 03 00 00 83 7C 24 ? 00 75 ? 48 8B 05 ? ? ? ? C6 80 29 02 00 00 00 C7 80 A8 06 00 00 00` |
| `ConCommand_thirdperson` | `__int64 ConCommand_thirdperson()` | `raw` | `0x7FFC157B2A10` | `0xAF2A10` | `48 83 EC 38 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 08 03 00 00 83 7C 24 ? 00 0F 85 ? ? ? ? 4C 8B 05 ? ? ? ? 41 8B 80 50 0B 00 00` |
| `Constructor` | `` | `raw` | `0x7FFC15B088E0` | `0xE488E0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 30 48 8B F1 48 8B FA B9 ? ? ? ? E8 ? ? ? ? 48 8B D8 48 85 C0 74` |
| `Context` | `void __fastcall Context(__int64 a1, __int64 a2)` | `raw` | `0x7FFC156C4180` | `0xA04180` | `48 8B C4 48 81 EC C8 00 00 00 48 89 58 10 48 89 68 18 48 8B EA 48 89 70 20 48 8B F1 48 89 78 F8` |
| `ConvarGet` | `void __fastcall ConvarGet(__int64 a1, unsigned int *a2)` | `raw` | `0x7FFC155A7D12` | `0x8E7D12` | `8B D0 48 8D 0D ? ? ? ? E8 ? ? ? ? 0F 10 45 ? 83 F0 74` |
| `CreateBaseTypeCache` | `` | `raw` | `0x7FFC161FA490` | `0x153A490` | `40 53 48 83 EC ? 4C 8B 49 ? 44 8B D2` |
| `CreateEconItem` | `` | `raw` | `0x7FFC15CE0CB0` | `0x1020CB0` | `48 83 EC 28 B9 48 00 00 00 E8 ? ? ? ? 48 85` |
| `CreateEntityByClassName` | `__int64 __fastcall CreateEntityByClassName(__int64 a1, int a2, __int64 a3, __int64 a4)` | `raw` | `0x7FFC162EDD86` | `0x162DD86` | `4C 8D 05 ? ? ? ? 4C 8B CF BA 03 00 00 00 FF 15 ? ? ? ? EB ? 0F B7 C8 48` |
| `CreateInterface` | `__int64 __fastcall CreateInterface(__int64 a1, _DWORD *a2)` | `raw` | `0x7FFC1651E860` | `0x185E860` | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 ? 49 8B 41 08` |
| `CreateMove` | `bool __fastcall CreateMove(void* pthis, int nSlot, float flInputSampleTime, bool bActive)` | `raw` | `0x7FFC15947C10` | `0xC87C10` | `48 8B C4 4C 89 40 18 48 89 48 08 55 53 41 54 41 55` |
| `CreateNewSubtickMoveStep` | `__int64 __fastcall CreateNewSubtickMoveStep(__int64 a1)` | `rel32` | `0x7FFC15172400` | `0x4B2400` | `E8 ? ? ? ? 48 8B D0 48 8B CE E8 ? ? ? ? 48 8B C8` |
| `CreateParticleEffect` | `__int64 __fastcall CreateParticleEffect(int a1, int a2, int a3, __int64 a4, int a5)` | `raw` | `0x7FFC1566F680` | `0x9AF680` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? F3 0F 10 1D ? ? ? ? 41 8B F8 8B DA 4C 8D 05` |
| `CreateSOSubclassEconItem` | `__int64 CreateSOSubclassEconItem()` | `raw` | `0x7FFC15CE0CB0` | `0x1020CB0` | `48 83 EC 28 B9 48 00 00 00 E8 ? ? ? ? 48 85` |
| `CreateTrace` | `` | `raw` | `0x7FFC154ED0D0` | `0x82D0D0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 56 41 57 48 83 EC ? ? ? ? ? 4D 8D 71` |
| `Ctrl` | `__int64 __fastcall Ctrl(__int64 a1)` | `raw` | `0x7FFC155BF940` | `0x8FF940` | `48 89 5C 24 ? 57 48 81 EC ? ? ? ? ? ? ? 48 8B F9 FF 90` |
| `DamageFeedbackEmitter` | `void __fastcall DamageFeedbackEmitter(__int64 a1, _QWORD *a2, __int64 a3)` | `raw` | `0x7FFC15508260` | `0x848260` | `48 89 4C 24 08 55 53 41 54 41 55 41 57 48 8D AC 24 E0 FE FF FF 48 81 EC 20 02 00 00 48 83 79 38` |
| `DamageServices` | `__int64 __fastcall DamageServices(int a1, _QWORD *a2)` | `stringref` | `0x7FFC154CDAC0` | `0x80DAC0` | `"CCSPlayerController_DamageServices"` |
| `DestroyParticle` | `void __fastcall DestroyParticle(__int64 a1, __int64 a2, unsigned __int8 a3, char a4)` | `raw` | `0x7FFC1562EA20` | `0x96EA20` | `83 FA ? 0F 84 ? ? ? ? 41 54` |
| `DispatchEffect` | `__int64 __fastcall DispatchEffect(__int64 a1, __int64 a2)` | `raw` | `0x7FFC1501ABF0` | `0x35ABF0` | `48 89 5C 24 ? 57 48 83 EC ? 48 8B F9 48 8B DA 48 8D 4C 24` |
| `DispatchSpawn` | `__int64 __fastcall DispatchSpawn(_QWORD *a1, __int64 a2, __int64 a3, __int64 a4)` | `raw` | `0x7FFC161BF100` | `0x14FF100` | `4C 8B DC 55 56 48 83 EC 78 49 8B 68 08 48 8B F1 48 85 ED 0F 84 72 01 00 00 49 89 5B 08 49 8D 4B` |
| `DispatchSpawn_caller` | `__int64 __fastcall DispatchSpawn_caller(_QWORD *a1, __int64 a2, __int64 a3, __int64 a4)` | `raw` | `0x7FFC161BF100` | `0x14FF100` | `4C 8B DC 55 56 48 83 EC 78 49 8B 68 08 48 8B F1 48 85 ED 0F 84 72 01 00 00` |
| `DispatchUpdateOnRemove` | `` | `raw` | `0x7FFC161BCBA0` | `0x14FCBA0` | `48 89 5C 24 10 48 89 74 24 18 48 89 7C 24 20 55 41 56 41 57 48 8B EC 48 83 EC 60 48 8D B9 80 00 00 00 45 33 FF 4D 8B F0` |
| `DrawCrosshair` | `bool __fastcall DrawCrosshair(_QWORD *a1)` | `raw` | `0x7FFC15499140` | `0x7D9140` | `48 89 5C 24 08 57 48 83 EC 20 48 8B D9 E8 ? ? ? ? 48 85` |
| `DrawLegs` | `void __fastcall DrawLegs(__int64 *a1, __int64 *a2, __int64 a3, __int64 a4, __int64 a5)` | `raw` | `0x7FFC15DD9950` | `0x1119950` | `40 55 53 56 41 56 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? F2 0F 10 42` |
| `DrawOverHead` | `unsigned __int8 __fastcall DrawOverHead(__int64 a1, unsigned int a2)` | `raw` | `0x7FFC1574F2D0` | `0xA8F2D0` | `40 53 48 83 EC ? 48 8B D9 83 FA ? 75 ? 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 ? ? ? ? 8B 10` |
| `DrawScopeOverlay` | `__int64 __fastcall DrawScopeOverlay(__int64 a1, __int64 a2)` | `raw` | `0x7FFC15545670` | `0x885670` | `48 8B C4 53 57 48 83 EC ? 48 8B FA` |
| `DrawSmokeVertex` | `__int64 __fastcall DrawSmokeVertex(__int64 a1, __int64 a2, int a3, int a4, __int64 a5, __int64 a6)` | `raw` | `0x7FFC15964710` | `0xCA4710` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 56 41 57 48 83 EC ? 48 8B 9C 24 ? ? ? ? 4D 8B F8` |
| `E8` | `__int64 __fastcall E8(__int64 a1, __int64 a2)` | `rel32` | `0x7FFC15535890` | `0x875890` | `E8 ? ? ? ? 4C 8B C0 48 8D 55 ? 48 8B CB E8 ? ? ? ? 48 8D 0D` |
| `EmitPanoramaSound` | `` | `raw` | `0x7FFC1584CBB0` | `0xB8CBB0` | `40 53 48 81 EC ? ? ? ? ? ? ? 48 8B 05` |
| `EmitSoundByHandle` | `__int64 __fastcall EmitSoundByHandle(__int64 a1, int a2, int a3, __int64 a4)` | `raw` | `0x7FFC1584C940` | `0xB8C940` | `40 53 48 83 EC 30 4C 89 4C 24 20 48 8B D9 45 8B C8 4C 8B C2 48 8B D1 48 8D 0D ?? ?? ?? ?? E8` |
| `EquipItemInLoadout` | `char __fastcall EquipItemInLoadout(_QWORD *a1, unsigned int a2, int a3, unsigned __int64 a4)` | `raw` | `0x7FFC154AA6A0` | `0x7EA6A0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 89 54 24 ? 57 41 54 41 55 41 56 41 57 48 83 EC ? 0F B7 FA` |
| `Event` | `__int64 __fastcall Event(__int64 a1, unsigned int a2, int a3)` | `raw` | `0x7FFC1516B1C0` | `0x4AB1C0` | `48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC 20 48 63 FA 41` |
| `FindHudElement` | `_QWORD **__fastcall FindHudElement(__int64 a1, unsigned __int8 a2)` | `raw` | `0x7FFC15AAB3C8` | `0xDEB3C8` | `48 8D 15 ? ? ? ? 45 33 C0 B9 ? ? ? ? FF 15 ? ? ? ? EB ? 48 8B 15` |
| `FindHudElement_panorama` | `__int64 __fastcall FindHudElement_panorama(const char *a1)` | `raw` | `0x7FFC15AAD3A0` | `0xDED3A0` | `4C 8B DC 53 48 83 EC 50 48 8B 05` |
| `FindOrCreateByName` | `char __fastcall FindOrCreateByName(__int64 a1, __int64 a2, char *a3, __int64 a4)` | `stringref` | `0x7FFC15D43BD0` | `0x1083BD0` | `"Kit "[%s]" specified, but doesn't exist!! You're probably missing an entry in items_paintkits.txt or items_stickerkits.txt or need to run with -use_local_item_data\n"` |
| `FindSOCache` | `__int64 __fastcall FindSOCache(__int64 a1, int *a2, __int64 a3, __int64 a4)` | `raw` | `0x7FFC16508150` | `0x1848150` | `48 89 5C 24 08 57 48 83 EC 30 4C 8B 52 08 48 8B D9 8B 0A` |
| `FireBullets` | `void FireBullets(unsigned int a1, __int64 a2, __int64 a3, __int64 *a4, __int64 a5, int a6, int a7, ...)` | `raw` | `0x7FFC15967800` | `0xCA7800` | `48 8B C4 4C 89 48 20 48 89 50 10 55 53 57 41 54 41 55 48 8D A8 58 FB FF FF 48 81 EC A0 05` |
| `FirstPersonLegs` | `void __fastcall FirstPersonLegs(__int64 *a1, __int64 *a2, __int64 a3, __int64 a4, __int64 a5)` | `raw` | `0x7FFC15DD9950` | `0x1119950` | `40 55 53 56 41 56 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? F2 0F 10 42` |
| `FlashOverlay` | `void __fastcall FlashOverlay(__int64 a1, int a2)` | `raw` | `0x7FFC15A947D0` | `0xDD47D0` | `85 D2 0F 88 ? ? ? ? 48 89 4C 24` |
| `ForceButtonsDown` | `void __fastcall ForceButtonsDown(_QWORD *a1, __int64 a2)` | `raw` | `0x7FFC156B8760` | `0x9F8760` | `40 53 57 41 56 48 81 EC ? ? ? ? 48 83 79` |
| `FrameStageNotify` | `` | `raw` | `0x7FFC157BAF20` | `0xAFAF20` | `48 89 5C 24 ? 48 89 6C 24 ? 57 48 83 EC ? 48 8B F9 33 ED` |
| `GetAttributeDefByName` | `` | `raw` | `0x7FFC15D363E0` | `0x10763E0` | `48 89 5C 24 10 48 89 6C 24 18 57 41 56 41 57 48 83 EC 60 48 8D 05` |
| `GetAttributeDefinitionByName` | `__int64 __fastcall GetAttributeDefinitionByName(__int64 a1, unsigned __int8 *a2)` | `raw` | `0x7FFC15D363E0` | `0x10763E0` | `48 89 5C 24 10 48 89 6C 24 18 57 41 56 41 57 48 83 EC 60 48 8D 05` |
| `GetBaseEntity` | `__int64 __fastcall GetBaseEntity(__int64 a1, int a2)` | `raw` | `0x7FFC1564FC60` | `0x98FC60` | `4C 8D 49 ? 81 FA` |
| `GetBonePositionByName` | `__int64 __fastcall GetBonePositionByName(__int64 a1, __int64 a2)` | `raw` | `0x7FFC155B0760` | `0x8F0760` | `40 53 48 83 EC ? 48 8B 89 ? ? ? ? 48 8B DA 48 8B 01 FF 50 ? 48 8B C8` |
| `GetCSInvMgr_call` | `` | `rel32` | `0x7FFC154AE980` | `0x7EE980` | `E8 ? ? ? ? 48 8B D8 8B F7` |
| `GetChatObject` | `__int64 GetChatObject()` | `rel32` | `0x7FFC15DACBB0` | `0x10ECBB0` | `E8 ? ? ? ? 48 8B E8 48 85 C0 0F 84 ? ? ? ? 4C 8D 05` |
| `GetClientSystem` | `__int64 *GetClientSystem()` | `rel32` | `0x7FFC15D1FAB0` | `0x105FAB0` | `E8 ? ? ? ? 48 8B C8 E8 ? ? ? ? 8B D8 85 C0 74 33` |
| `GetControllerCmd` | `__int64 __fastcall GetControllerCmd(__int64 a1, int a2)` | `raw` | `0x7FFC155A6310` | `0x8E6310` | `40 53 48 83 EC 20 8B DA E8 ? ? ? ? 4C` |
| `GetCustomPaintKitIndex` | `__int64 __fastcall GetCustomPaintKitIndex(__int64 *a1)` | `raw` | `0x7FFC15D91FA0` | `0x10D1FA0` | `48 89 5C 24 ? 57 48 83 EC ? 8B 15 ? ? ? ? 48 8B F9 65 48 8B 04 25 ? ? ? ? B9 ? ? ? ? 48 8B 04 D0 8B 04 01 39 05 ? ? ? ? 0F 8F ? ? ? ? E8 ? ? ? ? 8B 58 ? 39 1D ? ? ? ? 74 ? E8 ? ? ? ? 48 8B 15 ? ? ? ? 48 8B C8 E8 ? ? ? ? 48 89 05 ? ? ? ? 89 1D ? ? ? ? EB ? 48 8B 05 ? ? ? ? 48 85 C0 74` |
| `GetEconItemSystem` | `__int64 GetEconItemSystem()` | `raw` | `0x7FFC15039EB0` | `0x379EB0` | `48 83 EC 28 48 8B 05 ? ? ? ? 48 85 C0 0F 85 ? ? ? ? 48 89 5C 24` |
| `GetEntityByIndex` | `__int64 __fastcall GetEntityByIndex(__int64 a1, int a2)` | `raw` | `0x7FFC1564FC60` | `0x98FC60` | `4C 8D 49 ? 81 FA` |
| `GetEntityHandle` | `__int64 __fastcall GetEntityHandle(__int64 a1)` | `raw` | `0x7FFC15636F10` | `0x976F10` | `48 85 C9 74 32 48 8B 49 10 48 85 C9 74 29 44 8B 41 10 BA` |
| `GetGlowColor` | `void __fastcall GetGlowColor(__int64 a1, float *a2)` | `raw` | `0x7FFC157F39F0` | `0xB339F0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B F2 48 8B F9 48 8B 54 24` |
| `GetHitGroup` | `__int64 __fastcall GetHitGroup(__int64 a1)` | `raw` | `0x7FFC15700210` | `0xA40210` | `40 53 48 83 EC 20 48 83 79 10 00 48 8B D9 74 16 E8 ?? ?? ?? ?? 84 C0 75 0D 48 8B 43 10 8B 40 38` |
| `GetInaccuracy` | `` | `raw` | `0x7FFC1547DFD0` | `0x7BDFD0` | `48 89 5C 24 ? 55 56 57 48 81 EC ? ? ? ? 44` |
| `GetInstance` | `__int64 GetInstance()` | `raw` | `0x7FFC157F3B00` | `0xB33B00` | `48 8B 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 8B 41 38 C3` |
| `GetInventoryManager` | `__int64 *GetInventoryManager()` | `rel32` | `0x7FFC154AE980` | `0x7EE980` | `E8 ? ? ? ? 48 8B D3 48 8B C8 4C 8B 00 41 FF 90 00 02` |
| `GetItemInLoadout` | `__int64 *__fastcall GetItemInLoadout(__int64 a1, unsigned int a2, unsigned int a3)` | `raw` | `0x7FFC154AC2C0` | `0x7EC2C0` | `40 55 48 83 EC ? 49 63 E8` |
| `GetItemViewByID` | `uintptr_t __fastcall GetItemViewByID(uintptr_t, uint64_t)` | `raw` | `0x7FFC15D38CE0` | `0x1078CE0` | `48 89 54 24 ? 53 48 83 EC ? 48 8B D9 48 85 D2 75 ? 33 C0 48 83 C4 ? 5B C3 48 83 C1 38 48 8D` |
| `GetLocalControllerById` | `__int64 __fastcall GetLocalControllerById(int a1)` | `raw` | `0x7FFC155C96A0` | `0x9096A0` | `48 83 EC 28 83 F9 FF 75 ? 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 ? ? ? ? 8B 08 48 63 C1 4C 8D 05` |
| `GetLocalPawn` | `__int64 __fastcall GetLocalPawn(int a1)` | `raw` | `0x7FFC155C96A0` | `0x9096A0` | `48 83 EC ? 83 F9 ? 75 ? 48 8B 0D ? ? ? ? 48 8D 54 24 ? ? ? ? FF 90 ? ? ? ? ? ? 48 63 C1 4C 8D 05` |
| `GetLocalPlayer_dispatcher` | `__int64 GetLocalPlayer_dispatcher()` | `raw` | `0x7FFC15039880` | `0x379880` | `48 83 EC 38 48 8B 05 ? ? ? ? 48 85 C0 0F 85 14 06 00 00 48 89 5C 24 40 B9 50 00 00 00 48 89` |
| `GetMatrixForView` | `double __fastcall GetMatrixForView(__int64 a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFC14E2A2E0` | `0x16A2E0` | `40 53 48 83 EC 60 0F 29 74 24 50 0F 57 DB F3 0F 10 ? ? ? ? ? 49 8B D8` |
| `GetPlayerByIndex_export` | `__int64 GetPlayerByIndex_export()` | `raw` | `0x7FFC15BE9E40` | `0xF29E40` | `48 83 EC 28 4C 8D 05 ? ? ? ? 48 8D 15 ? ? ? ? 48 8D 0D ? ? ? ? E8 ? ? ? ? 4C 8D` |
| `GetPlayerInterp` | `float __fastcall GetPlayerInterp(__int64 a1)` | `raw` | `0x7FFC155A1B70` | `0x8E1B70` | `40 53 48 83 EC ? 48 8B D9 48 8B 0D ? ? ? ? 48 83 C1` |
| `GetRemovedAimpunch` | `__int64 GetRemovedAimpunch()` | `raw` | `0x7FFC14DD2FD7` | `0x112FD7` | `F2 0F 10 44 24 ? F2 0F 11 84 24 ? ? ? ? FF 15` |
| `GetSurfaceData` | `__int64 __fastcall GetSurfaceData(__int64 a1)` | `rel32` | `0x7FFC1563BB80` | `0x97BB80` | `E8 ? ? ? ? 80 78 18 00` |
| `GetTickBase` | `__int64 __fastcall GetTickBase(__int64 a1)` | `rel32` | `0x7FFC155A6110` | `0x8E6110` | `E8 ? ? ? ? EB ? 48 8B 05 ? ? ? ? 8B 40` |
| `GetTraceInfo` | `__int64 __fastcall GetTraceInfo(__int64 a1, __int64 a2, float a3, unsigned __int64 *a4)` | `raw` | `0x7FFC154EF700` | `0x82F700` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B E9 0F 29 74 24 ? 48 8B CA` |
| `GetTransformsForHitboxList` | `char __fastcall GetTransformsForHitboxList(__int64 a1, __int64 a2, int *a3)` | `raw` | `0x7FFC15702C90` | `0xA42C90` | `48 89 5C 24 18 55 56 57 41 55 41 57 48 81 EC A0 00 00 00 49 63 28 4D 8B F8 48 8B FA 48 8B D9 85` |
| `GetUserCmdManager` | `__int64 __fastcall GetUserCmdManager(__int64 a1)` | `raw` | `0x7FFC155A63A0` | `0x8E63A0` | `41 56 41 57 48 83 EC ? 48 8D 54 24` |
| `GetViewAngles` | `__int64 *__fastcall GetViewAngles(__int64 a1, int a2)` | `raw` | `0x7FFC157BE320` | `0xAFE320` | `4C 8B C1 85 D2 74 08 48 8D 05 ? ? ? ? C3` |
| `GetViewModelOffsets` | `void __fastcall GetViewModelOffsets(__int64 viewmodel, float *outOffsets, float *outFov)` | `raw` | `0x7FFC15537660` | `0x877660` | `40 55 53 56 41 56 41 57 48 8B EC 48 83 EC 20 4D 8B F8 4C 8B F2 48 8B F1 E8` |
| `GetWeaponInAccuracyRecoveryTime` | `__m128 __fastcall GetWeaponInAccuracyRecoveryTime(__int64 a1)` | `rel32` | `0x7FFC1547EA40` | `0x7BEA40` | `E8 ? ? ? ? F3 0F 10 B7 ? ? ? ? F3 0F 5E F8` |
| `GetWorldFovResolver` | `float __fastcall GetWorldFovResolver(__int64 a1)` | `raw` | `0x7FFC154F56A0` | `0x8356A0` | `40 53 48 83 EC 50 48 8B D9 E8 ? ? ? ? 48 85 C0 74 ? 48 8B C8 48 83 C4 50 5B E9` |
| `GlobalLightUpdateState` | `_BYTE *__fastcall GlobalLightUpdateState(__int64 a1)` | `raw` | `0x7FFC15773B80` | `0xAB3B80` | `40 57 48 81 EC C0 00 00 00 48 8B F9 BA FF FF FF FF 48 8D 0D ? ? ? ? E8` |
| `HandleBulletPenetration` | `char __fastcall HandleBulletPenetration(__int64 a1, float *a2, __int64 a3, int a4, __int64 a5)` | `raw` | `0x7FFC15509910` | `0x849910` | `48 8B C4 44 89 48 ? 48 89 50 ? 48 89 48 ? 55` |
| `HandleEntityList` | `__int64 __fastcall HandleEntityList(__int64 a1, __int64 a2, __int64 a3, __int64 a4, __int64 a5, int a6, int a7)` | `rel32` | `0x7FFC14E83D90` | `0x1C3D90` | `E8 ? ? ? ? 84 C0 74 ? 48 63 03` |
| `HandleTeamIntro` | `void __fastcall HandleTeamIntro(__int64 a1, __int64 a2, char *a3)` | `raw` | `0x7FFC153EC3B0` | `0x72C3B0` | `48 83 EC ? ? ? ? ? 44 38 89` |
| `HostageServices` | `void *__fastcall HostageServices(__int64 a1)` | `stringref` | `0x7FFC154FC2C0` | `0x83C2C0` | `"CCSPlayer_HostageServices"` |
| `HudChatPrintf` | `__int64 HudChatPrintf(__int64 a1, unsigned int a2, __int64 a3, ...)` | `rel32` | `0x7FFC15DAA630` | `0x10EA630` | `E8 ? ? ? ? 49 8B 4E 20 BA ? ? ? ?` |
| `InGameMoneyServices` | `__int64 __fastcall InGameMoneyServices(int a1, _QWORD *a2)` | `stringref` | `0x7FFC154CDAC0` | `0x80DAC0` | `"CCSPlayerController_InGameMoneyServices"` |
| `Init` | `__int64 __fastcall Init(__int64 a1, __int64 a2, __int64 a3)` | `stringref` | `0x7FFC1587A090` | `0xBBA090` | `"CompositeMaterialPanoramaPanel_t::Init"` |
| `InitFilter` | `__int64 __fastcall InitFilter(__int64 a1, _DWORD *a2, __int64 a3, char a4, char a5)` | `raw` | `0x7FFC14FEC270` | `0x32C270` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 0F B6 41 ? 33 FF 24 C9 C7 41 ?` |
| `InitPlayerMovementTraceFilter` | `__int64 __fastcall InitPlayerMovementTraceFilter(__int64 a1, _DWORD *a2, __int64 a3, char a4)` | `raw` | `0x7FFC15528830` | `0x868830` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 0F B6 41 ? 33 FF C7 41 ?` |
| `InitTraceData` | `` | `raw` | `0x7FFC154E8E20` | `0x828E20` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8D 79 ? 33 F6 C7 47` |
| `InitTraceInfo` | `__int64 __fastcall InitTraceInfo(__int64 a1)` | `raw` | `0x7FFC162E53E0` | `0x16253E0` | `40 55 41 55 41 57 48 83 EC` |
| `InsecureEmitter` | `` | `raw` | `0x7FFC15936000` | `0xC76000` | `48 89 5C 24 20 56 48 83 EC 20 48 8B D9 48 89 6C 24 30 48 8B E9 48 8B 0D ? ? ? ? 48 8B 01` |
| `InventoryServices` | `__int64 __fastcall InventoryServices(int a1, _QWORD *a2)` | `stringref` | `0x7FFC154CDAC0` | `0x80DAC0` | `"CCSPlayerController_InventoryServices"` |
| `ItemServices` | `void *__fastcall ItemServices(__int64 a1)` | `stringref` | `0x7FFC15538D30` | `0x878D30` | `"CCSPlayer_ItemServices"` |
| `KillFeedbackEmitter` | `__int64 __fastcall KillFeedbackEmitter(__int64 a1, __int64 a2)` | `raw` | `0x7FFC155332A0` | `0x8732A0` | `48 89 5C 24 08 48 89 74 24 18 48 89 7C 24 20 55 41 56 41 57 48 8B EC 48 81 EC 80 00 00 00 44 8B` |
| `LevelInit` | `__int64 __fastcall LevelInit(__int64 a1)` | `raw` | `0x7FFC155B8680` | `0x8F8680` | `40 55 56 41 56 48 8D 6C 24 ? 48 81 EC ? ? ? ? 48` |
| `LoadDefaultKit` | `char __fastcall LoadDefaultKit(__int64 a1, KeyValues *a2, _DWORD *a3)` | `stringref` | `0x7FFC15D15CA0` | `0x1055CA0` | `"Unable to find "default" paint kit in "paint_kits_rarity""` |
| `LoadFileForMe` | `void __fastcall LoadFileForMe(__int64 a1)` | `raw` | `0x7FFC15604570` | `0x944570` | `40 55 57 41 56 48 83 EC 20 4C` |
| `LoadPath` | `void __fastcall LoadPath(signed int *a1, signed int a2, unsigned int a3)` | `rel32` | `0x7FFC153A36B0` | `0x6E36B0` | `E8 ? ? ? ? 8B 44 24 2C` |
| `LookupBone` | `__int64 __fastcall LookupBone(__int64 a1, __int64 a2)` | `rel32` | `0x7FFC155B0760` | `0x8F0760` | `E8 ? ? ? ? 48 8B 8D ? ? ? ? B3` |
| `MatchFoundHandler` | `` | `raw` | `0x7FFC159462D0` | `0xC862D0` | `48 83 EC ? 48 8B 0D ? ? ? ? 48 85 C9 74 ? ? ? ? 48 89 7C 24` |
| `ModulationUpdate` | `__int64 __fastcall ModulationUpdate(__int64 a1, char a2)` | `raw` | `0x7FFC156C2AE0` | `0xA02AE0` | `48 89 5C 24 08 57 48 83 EC 20 8B FA 48 8B D9 E8 ? ? ? ? 84 C0 0F 84` |
| `MovementServices` | `__int64 *MovementServices()` | `stringref` | `0x7FFC155260B0` | `0x8660B0` | `"CCSPlayer_MovementServices"` |
| `NoClipOnChange` | `__int64 __fastcall NoClipOnChange(__int64 a1)` | `raw` | `0x7FFC14E27290` | `0x167290` | `48 89 5C 24 10 48 89 74 24 18 48 89 7C 24 20 55 48 8B EC 48 83 EC 30 48 8D 05` |
| `NoSpread1` | `__int64 __fastcall NoSpread1(__int64 a1, __int64 a2, int a3)` | `raw` | `0x7FFC15967750` | `0xCA7750` | `48 89 5C 24 08 57 48 81 EC F0 00` |
| `OnAddEntity` | `__int64 __fastcall OnAddEntity(__int64 a1, __int64 a2, int a3)` | `raw` | `0x7FFC15650CA0` | `0x990CA0` | `48 89 74 24 ? 57 48 83 EC ? 41 B9 ? ? ? ? 41 8B C0 41 23 C1 48 8B F2 41 83 F8 ? 48 8B F9 44 0F 45 C8 41 81 F9 ? ? ? ? 73 ? FF 81` |
| `OnBodyGroupChoiceChanged` | `__int64 __fastcall OnBodyGroupChoiceChanged(__int64 a1, __int64 a2, int a3, _DWORD *a4)` | `raw` | `0x7FFC1570D8E0` | `0xA4D8E0` | `48 89 5C 24 08 57 48 83 EC 20 49 63 D8 49 8B F9 45 85 C0 78 20 3B 99 18 02 00 00 7D 18` |
| `OnEvent` | `void __fastcall OnEvent(__int64 a1, KeyValues *a2)` | `raw` | `0x7FFC15945A80` | `0xC85A80` | `40 53 57 48 81 EC 78 02 00 00 48 8B CA 48 8B FA` |
| `OnGlowTypeChanged` | `__int64 __fastcall OnGlowTypeChanged(__int64 a1)` | `raw` | `0x7FFC157F5BC0` | `0xB35BC0` | `48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 20 48 8B 05 ? ? ? ? 48 8B D9 F3 0F 10 41 4C` |
| `OnPostDataUpdate` | `` | `raw` | `0x7FFC15694160` | `0x9D4160` | `48 89 5C 24 08 48 89 74 24 18 55 57 41 56 48 8B EC 48 83 EC 50 45 8B F1 48 8B FA 48 8B F1 45 85` |
| `OnRemoveEntity` | `__int64 __fastcall OnRemoveEntity(__int64 a1, _QWORD *a2, int a3)` | `raw` | `0x7FFC15651500` | `0x991500` | `48 89 74 24 ? 57 48 83 EC ? 41 B9 ? ? ? ? 41 8B C0 41 23 C1 48 8B F2 41 83 F8 ? 48 8B F9 44 0F 45 C8 41 81 F9 ? ? ? ? 73 ? FF 89` |
| `OnSkeletonModelChanged` | `__int64 __fastcall OnSkeletonModelChanged(__int64 a1, __int64 a2, __int64 *a3)` | `raw` | `0x7FFC1570DAF0` | `0xA4DAF0` | `49 8B 00 48 89 81 B8 00 00 00 C6 81 B0 00 00 00 01 C3` |
| `PanelConstructorPointer` | `` | `raw` | `0x7FFC16315E60` | `0x1655E60` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 30 48 8B F1 48 8B FA B9 ? ? ? ? E8 ? ? ? ? 48 8B D8 48 85 C0 74 ? 48` |
| `ParseSubtickDuration` | `` | `raw` | `0x7FFC14D6DA00` | `0xADA00` | `40 55 48 8D AC 24 70 FD FF FF 48 81 EC 90 03 00 00 F2 0F 10 05 ? ? ? ? 48 8D 05` |
| `ParseSubtickFraction` | `` | `raw` | `0x7FFC14D6DD40` | `0xADD40` | `40 55 48 8D AC 24 40 FE FF FF 48 81 EC C0 02 00 00 F2 0F 10 05 ? ? ? ? 48 8D 05` |
| `ParticleCollection` | `__int64 __fastcall ParticleCollection(__int64 a1)` | `raw` | `0x7FFC14EB5420` | `0x1F5420` | `48 89 5C 24 ? 57 48 83 EC ? 0F 28 05` |
| `Pawn` | `__int64 __fastcall Pawn(__int64 a1, __int64 a2)` | `raw` | `0x7FFC14EDF5E0` | `0x21F5E0` | `48 89 5C 24 ? 57 48 83 EC ? ? ? ? 48 8B FA 48 8B D9 FF 90 ? ? ? ? 84 C0 0F 85` |
| `PerTick` | `void __fastcall PerTick(int *a1)` | `raw` | `0x7FFC158AA230` | `0xBEA230` | `40 55 56 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 48 8B B9 A0 00 00 00` |
| `PerTickOrchestrator` | `char __fastcall PerTickOrchestrator(_QWORD *a1)` | `raw` | `0x7FFC158ACE10` | `0xBECE10` | `48 8B C4 55 53 48 8D A8 ? ? ? ? 48 81 EC ? ? ? ? 80 B9 B1 13 00 00 00` |
| `PerformBatchedInvalidatePhysicsRecursive` | `void __fastcall PerformBatchedInvalidatePhysicsRecursive(char a1)` | `raw` | `0x7FFC15626CA0` | `0x966CA0` | `40 57 48 81 EC 90 00 00 00 84 C9 74 4D BF 01 00 00 00 F0 0F C1 3D ? ? ? ? FF C7 83 FF 01 0F 85 63 05 00 00 48 8D 0D ? ? ? ? 48 8D 15` |
| `PingServices` | `void *__fastcall PingServices(__int64 a1)` | `stringref` | `0x7FFC15539FD0` | `0x879FD0` | `"CCSPlayer_PingServices"` |
| `PlayVSound_client` | `__int64 __fastcall PlayVSound_client(__int64 a1)` | `raw` | `0x7FFC161F82F0` | `0x15382F0` | `48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 55 48 8D 6C 24 ? 48 81 EC ? ? ? ? 33 FF` |
| `PointerToGetInaccuracyFunction` | `` | `raw` | `0x7FFC1547DFD0` | `0x7BDFD0` | `48 89 5C 24 ? 55 56 57 48 81 EC ? ? ? ? 44` |
| `PointerToGetSpreadFunction` | `` | `raw` | `0x7FFC1547EFF0` | `0x7BEFF0` | `48 83 EC ? 48 63 91` |
| `PostDataUpdate` | `char __fastcall PostDataUpdate(__int64 a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFC1570EA80` | `0xA4EA80` | `48 8B C4 4C 89 40 18 89 50 10 55 57 48 8D A8 68 FE FF FF 48 81 EC 88 02 00 00 48 89 70 E0 48 8B` |
| `ProcessForceSubtickMoves` | `` | `raw` | `0x7FFC156BE580` | `0x9FE580` | `40 55 53 48 8D AC 24 68 FF FF FF 48 81 EC 98 01 00 00 8B 15 ? ? ? ? 48 8B D9 65 48 8B 04 25 58 00 00 00 B9 98 00 00 00 48 8B 04 D0 8B 04 01 39 05 ? ? ? ? 0F 8F B6 07 00 00` |
| `ProcessImpacts` | `__int64 __fastcall ProcessImpacts(_QWORD *a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFC156B7080` | `0x9F7080` | `48 8B C4 53 56 41 55` |
| `ProcessMovement` | `__int64 __fastcall ProcessMovement(__int64 a1, __int64 a2)` | `rel32` | `0x7FFC156C20C0` | `0xA020C0` | `E8 ? ? ? ? 48 8B 06 48 8B CE FF 90 ? ? ? ? 48 85 DB` |
| `QueueForceSubtickMove` | `` | `raw` | `0x7FFC156AFF20` | `0x9EFF20` | `48 83 EC 28 8B 0D ? ? ? ? 65 48 8B 04 25 58 00 00 00 BA 98 00 00 00 48 8B 04 C8 8B 04 02 39 05 ? ? ? ? 0F 8F F4 11 00 00` |
| `QueuePostDataUpdates` | `` | `raw` | `0x7FFC16197450` | `0x14D7450` | `48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 40 80 B9 DA 0B 00 00 00 49 8B D8 8B FA 48 8B F1 74 61` |
| `RegenerateWeaponSkin` | `void __fastcall RegenerateWeaponSkin(__int64 a1, char a2)` | `raw` | `0x7FFC154746D0` | `0x7B46D0` | `40 55 53 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 44 0F B6 FA 48 8B D9 BA ? ? ? ? 48 8D 0D ? ? ? ? E8 ? ? ? ?` |
| `RegenerateWeaponSkin_v2` | `void __fastcall RegenerateWeaponSkin_v2(__int64 a1, char a2)` | `raw` | `0x7FFC154746D0` | `0x7B46D0` | `40 55 53 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 44 0F B6 FA 48 8B D9 BA ? ? ? ? 48 8D 0D ? ? ? ? E8` |
| `RegenerateWeaponSkins` | `__int64 RegenerateWeaponSkins()` | `raw` | `0x7FFC15499290` | `0x7D9290` | `48 83 EC ? E8 ? ? ? ? 48 85 C0 0F 84 ? ? ? ? 48 8B 10` |
| `RenderDecals` | `_BYTE *__fastcall RenderDecals(__int64 a1, __int64 **a2, char a3, char a4)` | `raw` | `0x7FFC15DD5F90` | `0x1115F90` | `44 88 4C 24 ? 55 53` |
| `ReportHit` | `char __fastcall ReportHit(_QWORD *a1)` | `rel32` | `0x7FFC152C1D30` | `0x601D30` | `E8 ? ? ? ? 48 8B AC 24 D8 00 00 00 48 81 C4` |
| `RunCommand` | `void __fastcall RunCommand(__int64 a1, __int64 a2)` | `raw` | `0x7FFC156C4180` | `0xA04180` | `48 8B C4 48 81 EC ? ? ? ? 48 89 58 10` |
| `RunCommand_processor` | `void __fastcall RunCommand_processor(__int64 a1, __int64 a2)` | `raw` | `0x7FFC156C4180` | `0xA04180` | `48 8B C4 48 81 EC C8 00 00 00 48 89 58 10 48 89 68 18 48 8B EA 48 89 70 20 48 8B F1 48 89 78 F8` |
| `SOCreated` | `void __fastcall SOCreated(uintptr_t, uint64_t, uintptr_t, int)` | `raw` | `0x7FFC150478B0` | `0x3878B0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B FA 48 8B F1` |
| `Scope_callsite` | `__int64 __fastcall Scope_callsite(__int64 a1, __int64 a2)` | `rel32` | `0x7FFC15545670` | `0x885670` | `E8 ? ? ? ? 80 7C 24 34 ? 74 ?` |
| `SendChatMessage` | `__int64 SendChatMessage(__int64 a1, unsigned int a2, __int64 a3, ...)` | `rel32` | `0x7FFC15DAA630` | `0x10EA630` | `E8 ? ? ? ? 49 8B 4E 20 BA ? ? ? ?` |
| `SetBodyGroup` | `` | `raw` | `0x7FFC155C24A0` | `0x9024A0` | `85 D2 0F 88 ? ? ? ? 55 53 56 41 56 48 8B EC 48 83 EC 78` |
| `SetBodyGroup_inv` | `void __fastcall SetBodyGroup_inv(__int64 a1, int a2, const char *a3)` | `raw` | `0x7FFC15A807B0` | `0xDC07B0` | `85 D2 0F 88 ? ? ? ? 53 55` |
| `SetBodygroup` | `void __fastcall SetBodygroup(__int64 a1, int a2, int a3)` | `raw` | `0x7FFC155C24A0` | `0x9024A0` | `85 D2 0F 88 CB 01 00 00 55 53 56 41 56 48 8B EC 48 83 EC 78 45 8B F0 8B DA 48 8B F1 E8 ? ? ?` |
| `SetCollisionBounds` | `__int64 __fastcall SetCollisionBounds(__int64 a1, __int64 *a2)` | `raw` | `0x7FFC154EC1F0` | `0x82C1F0` | `48 83 EC ? F2 0F 10 02 8B 42 08` |
| `SetDynamicAttributeValue` | `__int64 __fastcall SetDynamicAttributeValue(__int64 a1, __int64 a2, _DWORD *a3)` | `raw` | `0x7FFC15CEE4A0` | `0x102E4A0` | `48 89 6C 24 ? 57 41 56 41 57 48 81 EC ? ? ? ? 48 8B FA C7 44 24 ? ? ? ? ? 4D 8B F8` |
| `SetDynamicAttributeValue_raw` | `__int64 __fastcall SetDynamicAttributeValue_raw(__int64 a1, __int64 a2, _DWORD *a3)` | `raw` | `0x7FFC15CEE4A0` | `0x102E4A0` | `48 89 6C 24 ? 57 41 56 41 57 48 81 EC ? ? ? ? 48 8B FA C7 44 24` |
| `SetMaterialGroup` | `void __fastcall SetMaterialGroup(__int64 a1, unsigned int a2)` | `raw` | `0x7FFC15714E00` | `0xA54E00` | `3B 91 C4 03 00 00 74 24 89 91 C4 03 00 00 48 8B 81 28 02 00 00 48 85 C0 74 12` |
| `SetMeshGroupMask` | `__int64 __fastcall SetMeshGroupMask(__int64 a1, __int64 a2)` | `raw` | `0x7FFC15716120` | `0xA56120` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8D 99` |
| `SetModel` | `__int64 __fastcall SetModel(__int64 a1, __int64 a2)` | `raw` | `0x7FFC155C37F0` | `0x9037F0` | `40 53 48 83 EC ? 48 8B D9 4C 8B C2 48 8B 0D ? ? ? ? 48 8D 54 24` |
| `SetPlayerReady` | `char __fastcall SetPlayerReady(__int64 a1, __int64 a2)` | `raw` | `0x7FFC15C072C0` | `0xF472C0` | `40 53 48 83 EC 20 48 8B DA 48 8D 15 ? ? ? ? 48 8B CB FF 15 ? ? ? ? 85 C0 75 14 BA` |
| `SetSelectedIndexFunctionPointer` | `` | `raw` | `0x7FFC1636FD50` | `0x16AFD50` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B F1 8B DA 48 83` |
| `SetTraceData` | `__int64 __fastcall SetTraceData(int *a1, _OWORD *a2)` | `rel32` | `0x7FFC154BCDF0` | `0x7FCDF0` | `E8 ? ? ? ? 8B 85 ? ? ? ? 48 8D 54 24 ? F2 0F 10 45` |
| `SetTraceInit` | `` | `rel32` | `0x7FFC157E0F50` | `0xB20F50` | `E8 ? ? ? ? F2 0F 10 ? 4C 8D ?` |
| `SetTypeKV3` | `unsigned __int64 *__fastcall SetTypeKV3(unsigned __int64 *a1, unsigned __int8 a2, unsigned __int8 a3)` | `raw` | `0x7FFC16503F80` | `0x1843F80` | `40 53 48 83 EC 30 4C 8B 11 41 B9 ? ? ? ? 49 83 CA 01 0F B6 C2 80 FA 06 48 8B D9 44 0F 45 C8` |
| `SetViewAngle` | `void __fastcall SetViewAngle(__int64 a1, int a2, __int64 *a3)` | `raw` | `0x7FFC157CD5B0` | `0xB0D5B0` | `85 D2 75 3D 48 63 81 ? ? ? ?` |
| `SetupCmd` | `__int64 __fastcall SetupCmd(__int64 a1)` | `raw` | `0x7FFC155A3630` | `0x8E3630` | `48 83 EC 28 E8 ? ? ? ? 8B 80` |
| `SetupMapInfo` | `` | `raw` | `0x7FFC15971D00` | `0xCB1D00` | `48 8B C4 48 89 58 ? 48 89 68 ? 48 89 70 ? 57 48 81 EC ? ? ? ? 0F 29 70 ? 48 8B EA 0F 29 78 ? 45 33 C0` |
| `SetupMove` | `__int64 __fastcall SetupMove(__int64 a1, int *a2)` | `raw` | `0x7FFC15A065B0` | `0xD465B0` | `48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 56 48 83 EC ? 48 8B EA 4C 8B F1 E8 ? ? ? ? 48 8D 15` |
| `SetupMovementMoves` | `__int64 __fastcall SetupMovementMoves(__int64 a1, __int64 a2, __int64 a3, __int64 a4)` | `raw` | `0x7FFC15E7027F` | `0x11B027F` | `48 8B ? E8 ? ? ? ? 48 8B 5C 24 ? 48 8B 6C 24 ? 48 83 C4 30` |
| `SharedRandomFloat` | `` | `raw` | `0x7FFC15717420` | `0xA57420` | `4C 8B DC 49 89 5B 08 49 89 73 10 57 48 81 EC 00 01 00 00 8B 05 ? ? ? ? 48 8D 54 24 40` |
| `ShowMessageBox` | `` | `raw` | `0x7FFC1598E880` | `0xCCE880` | `44 88 4C 24 ? 53 41 56` |
| `Shutdown` | `__int64 Shutdown()` | `raw` | `0x7FFC157CE460` | `0xB0E460` | `48 89 5C 24 08 55 56 57 41 54 41 55 41 56 41 57 48 81 EC 40 02 00 00 8B 0D ? ? ? ? BA 02 00 00` |
| `SomeTimingFromPawn` | `float __fastcall SomeTimingFromPawn(__int64 a1, int a2, unsigned int a3)` | `raw` | `0x7FFC1573F890` | `0xA7F890` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 49 63 D8 48 8B F1` |
| `SpectatorInput` | `__int64 __fastcall SpectatorInput(_DWORD *a1, __int64 a2, unsigned int a3)` | `raw` | `0x7FFC154C19A0` | `0x8019A0` | `48 89 5C 24 10 55 56 57 41 56 41 57 48 8B EC 48 83 EC 60 48 8B 01 41 8B F8 48 8B DA 48 8B F1 FF` |
| `SpreadSeedGen` | `__int64 __fastcall SpreadSeedGen(__int64 a1, __int64 a2, int a3)` | `raw` | `0x7FFC15967750` | `0xCA7750` | `48 89 5C 24 08 57 48 81 EC F0 00 00 00 F3 0F 10 0A 48 8D 8C 24 10 01 00 00 41 8B D8 48 8B FA E8` |
| `StartHierarchicalAttachment` | `char __fastcall StartHierarchicalAttachment(__int64 a1)` | `raw` | `0x7FFC15674C40` | `0x9B4C40` | `48 89 5C 24 10 48 89 6C 24 18 48 89 74 24 20 57 41 54 41 55 41 56 41 57 48 83 EC 30 48 8B F9 8B` |
| `TakeDamageOld` | `unsigned __int64 __fastcall TakeDamageOld(__int64 a1, __int64 a2, __int64 **a3)` | `raw` | `0x7FFC14EE43B0` | `0x2243B0` | `40 55 53 56 57 41 54 48 8D 6C 24 E0 48 81 EC 20 01 00 00 4D 8B E0 48 8B FA 48 8B F1 E8` |
| `TestSurfaces` | `void __fastcall TestSurfaces(__int64 a1, float a2, float a3, float a4, int a5, int a6, __int64 a7)` | `raw` | `0x7FFC154EF5E0` | `0x82F5E0` | `40 53 57 41 56 48 83 EC 50 8B` |
| `ThinkReturn` | `char __fastcall ThinkReturn(__int64 a1, _DWORD *a2)` | `raw` | `0x7FFC14FDAB7F` | `0x31AB7F` | `BA 04 00 00 00 FF 15 ? ? ? ? 84 C0 0F 84` |
| `ThirdPersonOffHandler` | `__int64 ThirdPersonOffHandler()` | `raw` | `0x7FFC157B2930` | `0xAF2930` | `48 83 EC 28 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 08 03 00 00 83 7C 24 ? 00 75 ? 48 8B 05 ? ? ? ? C6 80 29 02 00 00 00 C7 80 A8 06 00 00 00` |
| `ThirdPersonOnHandler` | `__int64 ThirdPersonOnHandler()` | `raw` | `0x7FFC157B2A10` | `0xAF2A10` | `48 83 EC 38 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 90 08 03 00 00 83 7C 24 ? 00 0F 85 ? ? ? ? 4C 8B 05 ? ? ? ? 41 8B 80 50 0B 00 00` |
| `ThirdPersonReset` | `` | `raw` | `0x7FFC157B0DE0` | `0xAF0DE0` | `48 8B 40 08 44 38 ? 75 10 44 88 ? 01` |
| `TraceCreate` | `char __fastcall TraceCreate(__int64 a1, __int64 a2, int a3, __int64 a4, int a5, char a6)` | `raw` | `0x7FFC154ED0D0` | `0x82D0D0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 56 41 57 48 83 EC 50 F2 0F 10 02` |
| `TraceGetInfo` | `__int64 __fastcall TraceGetInfo(__int64 a1, __int64 a2, float a3, unsigned __int64 *a4)` | `raw` | `0x7FFC154EF700` | `0x82F700` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC 60 48 8B E9 0F 29 74 24` |
| `TraceHandleBulletPen` | `char __fastcall TraceHandleBulletPen(__int64 a1, float *a2, __int64 a3, int a4, __int64 a5)` | `raw` | `0x7FFC15509910` | `0x849910` | `48 8B C4 44 89 48 20 48 89 50 10 48 89 48 08 55 57 41 57` |
| `TraceInitData` | `__int64 __fastcall TraceInitData(__int64 a1)` | `raw` | `0x7FFC154E8E20` | `0x828E20` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 20 48 8D 79 ? 33 F6 C7 47` |
| `TraceInitFilter` | `__int64 __fastcall TraceInitFilter(__int64 a1, _DWORD *a2, __int64 a3, char a4, char a5)` | `raw` | `0x7FFC14FEC270` | `0x32C270` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 0F B6 41 ? 33 FF 24` |
| `TraceInitInfo` | `__int64 __fastcall TraceInitInfo(__int64 a1)` | `raw` | `0x7FFC162E53E0` | `0x16253E0` | `40 55 41 55 41 57 48 83 EC 30` |
| `TracePlayerBBox` | `__int64 __fastcall TracePlayerBBox(__int64 a1, __int64 *a2, __int64 *a3)` | `raw` | `0x7FFC15859C60` | `0xB99C60` | `48 89 5C 24 ? 55 57 41 54 41 55 41 56` |
| `TraceShape` | `bool __fastcall TraceShape(_QWORD *a1, __int64 a2, __int64 a3, __int64 a4, _BYTE *a5, __int64 a6)` | `raw` | `0x7FFC15677100` | `0x9B7100` | `48 89 5C 24 ? 48 89 4C 24 ? 55 57` |
| `TraceToExit` | `char __fastcall TraceToExit(__int64 a1, __int64 a2, int a3, __int64 a4, int a5, char a6)` | `raw` | `0x7FFC154ED0D0` | `0x82D0D0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 56 41 57 48 83 EC ? F2 0F 10 02` |
| `TypeManager` | `__int64 __fastcall TypeManager(int a1, __int64 a2)` | `stringref` | `0x7FFC160C26A0` | `0x14026A0` | `"InfoForResourceTypeCCompositeMaterialKit"` |
| `UnknownParticleFunction` | `` | `raw` | `0x7FFC1566FB40` | `0x9AFB40` | `40 56 48 83 EC ? 41 8B F0` |
| `UnserializeEvent` | `__int64 __fastcall UnserializeEvent(__int64 a1, __int64 a2)` | `raw` | `0x7FFC1567AF60` | `0x9BAF60` | `48 8B C4 48 89 50 10 55 41 54 41 55 41 56 48 8D 68 D8 48 81 EC 08 01 00 00 48 89 58 D8 4C 8D B1` |
| `UntrustedFlagSetter` | `` | `raw` | `0x7FFC14E171D5` | `0x1571D5` | `74 26 C6 05 ? ? ? ? 01 33 C0 83 F8 01` |
| `UpdateGlobalVars` | `void *__fastcall UpdateGlobalVars(__int64 a1, void *a2)` | `raw` | `0x7FFC157CD000` | `0xB0D000` | `48 8B 0D ? ? ? ? 4C 8D 05 ? ? ? ? 48 85 D2` |
| `UpdateOnRemove` | `` | `raw` | `0x7FFC161B3140` | `0x14F3140` | `48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 40 48 8B D9 C6 05 ? ? ? ? 01 48 8B 49` |
| `UpdatePostProcessing` | `void __fastcall UpdatePostProcessing(__int64 a1, _BYTE *a2)` | `raw` | `0x7FFC15C0B450` | `0xF4B450` | `48 85 D2 0F 84 ? ? ? ? 48 89 5C 24 08 57 48 83 EC 60 80` |
| `UpdateSkybox` | `__int64 __fastcall UpdateSkybox(__int64 a1)` | `raw` | `0x7FFC14F1AEE0` | `0x25AEE0` | `48 89 5C 24 ? 57 48 83 EC ? 48 8B F9 E8 ? ? ? ? 48 8B 47` |
| `UpdateSubClass` | `void __fastcall UpdateSubClass(_QWORD *a1)` | `raw` | `0x7FFC14EBAFC0` | `0x1FAFC0` | `4C 8B DC 53 48 81 EC ? ? ? ? 48 8B 41 10 48 8B D9 8B 50 30 C1 EA 04` |
| `UpdateTurningInAccuracy` | `float *__fastcall UpdateTurningInAccuracy(float *a1)` | `rel32` | `0x7FFC154982F0` | `0x7D82F0` | `E8 ? ? ? ? F3 0F 10 87 ? ? ? ? 44 0F 2F C8` |
| `Use` | `` | `raw` | `0x7FFC15498AD0` | `0x7D8AD0` | `40 55 53 56 48 8D AC 24 C0 FE FF FF 48 81 EC 40 02 00 00 48 8B DA 48 8B F1 BA FF FF FF FF` |
| `UseServices` | `__int64 UseServices()` | `stringref` | `0x7FFC1556A330` | `0x8AA330` | `"CCSPlayer_UseServices"` |
| `ViewModelHideZoomed` | `__int64 __fastcall ViewModelHideZoomed(__int64 a1, __int64 a2, __int64 **a3)` | `raw` | `0x7FFC154888A0` | `0x7C88A0` | `48 89 5C 24 20 55 56 57 41 54 41 56 48 8B EC 48 83 EC 50 48 8D 05` |
| `WaterServices` | `__int64 *WaterServices()` | `stringref` | `0x7FFC1555F5C0` | `0x89F5C0` | `"CCSPlayer_WaterServices"` |
| `WeaponServices` | `__int64 *WeaponServices()` | `stringref` | `0x7FFC1555F970` | `0x89F970` | `"CCSPlayer_WeaponServices"` |
| `WriteSubtickFromEntry` | `` | `raw` | `0x7FFC1593F750` | `0xC7F750` | `48 89 5C 24 ? 55 57 41 56 48 8D 6C 24 ? 48 81 EC B0 00 00 00 8B 01 48 8B F9 81 4A 10 00 02` |
| `create_move_v2` | `void __fastcall create_move_v2(__int64 *a1, int a2, char a3)` | `raw` | `0x7FFC157B47A0` | `0xAF47A0` | `85 D2 0F 85 ? ? ? ? 48 8B C4 44 88 40` |
| `draw_smoke_array` | `__int64 __fastcall draw_smoke_array(__int64 a1, __int64 a2, __int64 a3, __int64 a4, __int64 a5, unsigned int *a6)` | `raw` | `0x7FFC15964800` | `0xCA4800` | `40 55 41 54 41 55 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 4C 8B E2` |
| `draw_view_punch_v2` | `float *__fastcall draw_view_punch_v2(int a1, __int64 a2, __int64 a3, float *a4, float *a5, float *a6)` | `raw` | `0x7FFC154EC990` | `0x82C990` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC ? 49 8B E9 49 8B F8` |
| `frame_stage_notify` | `__int64 __fastcall frame_stage_notify(__int64 a1, int a2)` | `raw` | `0x7FFC157BB3B1` | `0xAFB3B1` | `4C 8B 0D ? ? ? ? 48 8D 15 ? ? ? ? 48 8B 8F ? ? ? ? F3 41 0F 10 51 ? 45 8B 49 ? 0F 5A D2 66 49 0F 7E D0 FF 15 ? ? ? ? 48 8B 97 ? ? ? ? 48 8B 0D ? ? ? ? E8 ? ? ? ? E9` |
| `get_fov` | `float *__fastcall get_fov(int a1, __int64 a2, __int64 a3, float *a4, float *a5, float *a6)` | `raw` | `0x7FFC154EC990` | `0x82C990` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC ? 49 8B E9 49 8B F8` |
| `get_map_name` | `__int64 get_map_name()` | `raw` | `0x7FFC15BC6A20` | `0xF06A20` | `48 83 EC ? 48 8B 0D ? ? ? ? ? ? ? FF 90 ? ? ? ? 48 8B C8 48 83 C4` |
| `get_view_angles_v2` | `void __fastcall get_view_angles_v2(__int64 a1, int a2, __int64 a3)` | `raw` | `0x7FFC157BCC80` | `0xAFCC80` | `4D 85 C0 74 ? 85 D2 74` |
| `get_view_model` | `void __fastcall get_view_model(__int64 a1, float *a2, float *a3)` | `raw` | `0x7FFC15537660` | `0x877660` | `40 55 53 56 41 56 41 57 48 8B EC` |
| `is_demo_or_hltv` | `char is_demo_or_hltv()` | `raw` | `0x7FFC15BE7EE0` | `0xF27EE0` | `48 83 EC ? 48 8B 0D ? ? ? ? ? ? ? FF 90 ? ? ? ? 84 C0 75 ? 38 05` |
| `level_init_v2` | `__int64 __fastcall level_init_v2(__int64 a1, __int64 a2)` | `raw` | `0x7FFC157E37C0` | `0xB237C0` | `40 55 56 41 56 48 8D 6C 24 ? 48 81 EC ? ? ? ? 48 8B 0D` |
| `level_shutdown` | `__int64 level_shutdown()` | `raw` | `0x7FFC157E3A40` | `0xB23A40` | `48 83 EC ? 48 8B 0D ? ? ? ? 48 8D 15 ? ? ? ? 45 33 C9 45 33 C0 ? ? ? FF 50 ? 48 85 C0 74 ? 48 8B 0D ? ? ? ? 48 8B D0 ? ? ? 41 FF 50 ? 48 83 C4` |
| `mark_interp_latch_flags_dirty` | `void __fastcall mark_interp_latch_flags_dirty(__int64 a1, unsigned int a2)` | `raw` | `0x7FFC14ED8700` | `0x218700` | `40 53 56 57 48 83 EC ? 80 3D ? ? ? ? 00` |
| `on_add_entity_v2` | `__int64 __fastcall on_add_entity_v2(__int64 a1, __int64 a2, __int64 a3)` | `raw` | `0x7FFC15651210` | `0x991210` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 8B 81 ? ? ? ? 49 8B F0` |
| `override_view_short` | `void __fastcall override_view_short(__int64 a1, __int64 a2)` | `raw` | `0x7FFC15948C60` | `0xC88C60` | `40 57 48 83 EC ? 48 8B FA E8 ? ? ? ? BA` |
| `paintkit_prefab` | `__int64 __fastcall paintkit_prefab(__int64 *a1)` | `stringref` | `0x7FFC15D468F0` | `0x10868F0` | `"set item texture prefab"` |
| `paintkit_seed` | `__int64 __fastcall paintkit_seed(__int64 a1)` | `stringref` | `0x7FFC15BDA860` | `0xF1A860` | `"set item texture seed"` |
| `paintkit_wear` | `__int64 __fastcall paintkit_wear(__int64 a1)` | `stringref` | `0x7FFC15BDA860` | `0xF1A860` | `"set item texture wear"` |
| `remove_legs` | `void __fastcall remove_legs(__int64 *a1, __int64 *a2, __int64 a3, __int64 a4, __int64 a5)` | `raw` | `0x7FFC15DD9950` | `0x1119950` | `40 55 53 56 41 56 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? F2 0F 10 42` |
| `statTrak_killEater` | `__int64 __fastcall statTrak_killEater(__int64 a1)` | `stringref` | `0x7FFC15BDA860` | `0xF1A860` | `"kill eater"` |
| `statTrak_scoreType` | `__int64 statTrak_scoreType()` | `stringref` | `0x7FFC14DDBE80` | `0x11BE80` | `"kill eater score type"` |
| `unlock_inventory` | `char __fastcall unlock_inventory(__int64 a1)` | `raw` | `0x7FFC153E96C0` | `0x7296C0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8B E9 48 8B 0D ? ? ? ? ? ? ? FF 50` |
| `update_global_vars` | `void *__fastcall update_global_vars(__int64 a1, void *a2)` | `raw` | `0x7FFC157CD000` | `0xB0D000` | `48 8B 0D ? ? ? ? 4C 8D 05 ? ? ? ? 48 85 D2` |
| `update_post_processing_v2` | `void __fastcall update_post_processing_v2(__int64 a1)` | `raw` | `0x7FFC15C0FA06` | `0xF4FA06` | `48 89 AC 24 ? ? ? ? 45 33 ED` |

## `engine2.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `CClient_SendMovePacket` | `char __fastcall CClient_SendMovePacket(__int64 a1)` | `raw` | `0x7FFC28484FA0` | `0x64FA0` | `40 55 57 41 55 48 8D AC 24 90 E0 FF FF B8 70 20 00 00 E8 ? ? ? ? 48 2B E0 4C 8B E9 C7 44 24 20 FF FF FF FF` |
| `CGameEventSystem_PostEventAbstract` | `__int64 __fastcall CGameEventSystem_PostEventAbstract(_BYTE *a1, unsigned int a2, char a3, int a4, __int64 *a5, __int64 a6, __int64 a7, __int64 a8, char a9)` | `raw` | `0x7FFC28636530` | `0x216530` | `48 89 5C 24 10 48 89 74 24 18 48 89 7C 24 20 55 41 54 41 55 41 56 41 57 48 8D 6C 24 F1 48 81 EC A0 00 00 00 4C 8B 65 67 4C 8B F1` |
| `CHLTVClient_SendSnapshot` | `char __fastcall CHLTVClient_SendSnapshot(__int64 a1, __int64 a2)` | `raw` | `0x7FFC28542120` | `0x122120` | `48 89 54 24 10 48 89 4C 24 08 55 53 56 57 41 56 41 57 48 8D 6C 24 88 48 81 EC 78 01 00 00 48 8D 05 ? ? ? ? 48 C7 45 18 7A 02 00 00` |
| `CHLTVClient_SetSignonState` | `char __fastcall CHLTVClient_SetSignonState(__int64 a1, int a2, __int64 a3, int a4)` | `raw` | `0x7FFC28543790` | `0x123790` | `40 55 53 41 55 41 56 41 57 48 8D 6C 24 C9 48 81 EC E0 00 00 00 45 8B E8 8B DA 4C 8B F9 45 33 F6` |
| `CHostStateMgr_HostStateRequest_Start` | `void __fastcall CHostStateMgr_HostStateRequest_Start(__int64 a1, __int64 a2)` | `raw` | `0x7FFC28639AF0` | `0x219AF0` | `40 53 48 83 EC 40 8B 01 48 8B D9 C6 41 18 01 83 F8 02 74 07 83 F8 04 75 21 EB 0D 8B 49 20 83 E9 06 74 17 83 F9 01 74 12` |
| `CInputService_ProcessConVar` | `void __fastcall CInputService_ProcessConVar(__int64 a1, __int64 a2)` | `raw` | `0x7FFC285E3DB0` | `0x1C3DB0` | `48 89 5C 24 08 48 89 74 24 10 48 89 7C 24 18 55 41 54 41 55 41 56 41 57 48 8D AC 24 40 F3 FF FF 48 81 EC C0 0D 00 00` |
| `CNetworkGameClient_InternalProcessPacketEntities` | `void __fastcall CNetworkGameClient_InternalProcessPacketEntities(__int64 a1, __int64 a2)` | `raw` | `0x7FFC284683B0` | `0x483B0` | `40 55 56 57 41 56 41 57 48 8D AC 24 40 FF FF FF 48 81 EC C0 01 00 00 65 48 8B 04 25 58 00 00 00` |
| `CNetworkGameClient_ProcessServerInfo` | `char __fastcall CNetworkGameClient_ProcessServerInfo(__int64 a1, __int64 a2)` | `raw` | `0x7FFC2848B140` | `0x6B140` | `48 89 5C 24 08 57 48 83 EC 30 48 8B FA 48 8B D9 8B 0D ? ? ? ? BA 02 00 00 00 FF 15` |
| `CNetworkStringTableContainer_CreateStringTable` | `__int64 __fastcall CNetworkStringTableContainer_CreateStringTable(__int64 a1, const char *a2, __int64 a3)` | `raw` | `0x7FFC2852C7F0` | `0x10C7F0` | `40 53 41 56 48 83 EC 48 4C 8B F2 48 8B D9 48 8B 12 48 85 D2 0F 84 ? ? ? ? 80 79 34 00` |
| `CNetworkStringTableContainer_WriteUpdateMessageAtTick` | `__int64 __fastcall CNetworkStringTableContainer_WriteUpdateMessageAtTick(__int64 a1, __int64 a2, int a3, int a4, int a5)` | `raw` | `0x7FFC2852D470` | `0x10D470` | `44 89 4C 24 20 44 89 44 24 18 48 89 4C 24 08 55 53 56 57 41 54 41 55 41 57 48 8D 6C 24 F0` |
| `CServerSideClient_ProcessServerInfo` | `char __fastcall CServerSideClient_ProcessServerInfo(__int64 a1, __int64 a2)` | `raw` | `0x7FFC284A4B20` | `0x84B20` | `48 89 5C 24 20 55 56 57 41 54 41 56 48 8D AC 24 10 FE FF FF 48 81 EC F0 02 00 00` |
| `CSplitScreenSlot` | `char __fastcall CSplitScreenSlot(__int64 a1, __int64 a2, int a3, __int64 a4)` | `stringref` | `0x7FFC2866AF50` | `0x24AF50` | `"CSplitScreenSlot"` |
| `ClientCommand` | `char ClientCommand(__int64 a1, int a2, __int64 a3, ...)` | `raw` | `0x7FFC284C1270` | `0xA1270` | `48 8B C4 4C 89 40 18 4C 89 48 20 55 53 57 48 8D 68 A1 48 81 EC C0 00 00 00 33 FF 48 63 DA 48 39` |
| `Connect` | `void __fastcall Connect(__int64 a1, int a2, unsigned int a3, __int64 a4, unsigned int a5, char a6)` | `raw` | `0x7FFC2849F420` | `0x7F420` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 57 48 83 EC 40 44 89 81 3C 02 00 00 49 8B E9 44 8B` |
| `DisablePvsAccessor` | `__int64 __fastcall DisablePvsAccessor(_DWORD *a1, __int64 a2, int a3, char a4)` | `raw` | `0x7FFC2865E0D2` | `0x23E0D2` | `48 8D 0D ? ? ? ? 33 D2 FF 50` |
| `Engine_Disconnect_main` | `__int64 *Engine_Disconnect_main()` | `raw` | `0x7FFC285F2210` | `0x1D2210` | `48 89 5C 24 20 55 57 41 54 48 8B EC 48 83 EC 70 45 33 E4 48 C7 05` |
| `ExecuteStringCommand` | `char __fastcall ExecuteStringCommand(__int64 a1, __int64 a2)` | `raw` | `0x7FFC28540ED0` | `0x120ED0` | `40 53 56 48 81 EC 48 07 00 00 48 8B F1 48 8B DA 48 8B 4A 48 48 83 E1 FC 48 83 79 18 0F 76 03 48` |
| `ForceDemoRecordingFullUpdateAfterNextDeltaPacket` | `char __fastcall ForceDemoRecordingFullUpdateAfterNextDeltaPacket(__int64 a1, const char *a2)` | `raw` | `0x7FFC284492C0` | `0x292C0` | `48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 20 48 8B 1D ? ? ? ? 48 8B FA 48 8B F1 48 85 DB` |
| `GetFreeClient` | `` | `raw` | `0x7FFC284CF4B0` | `0xAF4B0` | `48 89 54 24 ? 53 56 57 41 56 48 83 EC` |
| `GetLevelName` | `` | `raw` | `0x7FFC28496540` | `0x76540` | `48 83 EC ? E8 ? ? ? ? 84 C0 74 ? 48 8D 05 ? ? ? ? 48 83 C4 ? C3 48 8B 0D ? ? ? ? 48 85 C9 74 ? 83 B9 ? ? ? ? ? 7C ? 48 8B 89 ? ? ? ? 48 8D 05 ? ? ? ? 48 85 C9 48 0F 45 C1 48 83 C4 ? C3 48 8D 05 ? ? ? ? 48 83 C4 ? C3 ? ? ? ? ? ? ? ? ? ? ? ? 48 83 EC` |
| `GetLevelNameShort` | `` | `raw` | `0x7FFC284965A0` | `0x765A0` | `48 83 EC ? E8 ? ? ? ? 84 C0 74 ? 48 8D 05 ? ? ? ? 48 83 C4 ? C3 48 8B 0D ? ? ? ? 48 85 C9 74 ? 83 B9 ? ? ? ? ? 7C ? 48 8B 89 ? ? ? ? 48 8D 05 ? ? ? ? 48 85 C9 48 0F 45 C1 48 83 C4 ? C3 48 8D 05 ? ? ? ? 48 83 C4 ? C3 ? ? ? ? ? ? ? ? ? ? ? ? B8` |
| `GetScreenAspectRatio` | `float __fastcall GetScreenAspectRatio(__int64 a1, int a2, int a3)` | `raw` | `0x7FFC284969F0` | `0x769F0` | `48 89 5C 24 08 57 48 83 EC 20 8B FA 48 8D 0D` |
| `HostStateRequest` | `` | `raw` | `0x7FFC2863BB20` | `0x21BB20` | `48 89 74 24 ? 57 48 83 EC ? 33 F6 48 8B FA 48 39 35` |
| `Host_FilterTime` | `bool __fastcall Host_FilterTime(__int64 a1, float *a2)` | `raw` | `0x7FFC286318F0` | `0x2118F0` | `48 89 5C 24 10 48 89 74 24 18 48 89 4C 24 08 57 48 81 EC A0 00 00 00 48 8B BC 24 D0 00 00 00` |
| `IsConnected` | `` | `raw` | `0x7FFC284964A0` | `0x764A0` | `48 8B 05 ? ? ? ? 48 85 C0 74 ? 83 B8 ? ? ? ? ? 0F 9D C0` |
| `IsHearingClient` | `` | `raw` | `0x7FFC284DCD30` | `0xBCD30` | `40 53 48 83 EC ? 48 8B D9 3B 51` |
| `IsInGame` | `bool IsInGame()` | `raw` | `0x7FFC28496470` | `0x76470` | `48 8B 05 ? ? ? ? 48 85 C0 74 ? 80 B8 ? ? ? ? 00 75 ? 83 B8 ? ? ? ? ? 7C` |
| `LoadGameInfo` | `char __fastcall LoadGameInfo(__int64 a1, const char *a2)` | `raw` | `0x7FFC285AE460` | `0x18E460` | `40 55 56 41 56 48 8D 6C 24 F0 48 81 EC 10 01 00 00 4C 8B F1 C7 44 24 40 00 00 00 00 48 8B CA 48` |
| `MountAddon` | `void __fastcall MountAddon(__int64 a1, const char *a2, char a3)` | `raw` | `0x7FFC285B4140` | `0x194140` | `48 85 D2 0F 84 DA 0A 00 00 48 8B C4 44 88 40 18 55 57 41 54 41 57 48 8D A8 C8 FC FF FF 48 81 EC` |
| `NetTimeoutDisconnect` | `__int64 __fastcall NetTimeoutDisconnect(__int64 a1, unsigned int a2, __int64 a3)` | `raw` | `0x7FFC284897A0` | `0x697A0` | `40 53 55 56 57 41 56 48 81 EC 80 00 00 00 0F 29 74 24 70 49 8B F8` |
| `OnSvCheatsChange` | `void __fastcall OnSvCheatsChange(__int64 a1, __int64 a2, _BYTE *a3, char *a4)` | `raw` | `0x7FFC284BC220` | `0x9C220` | `40 53 48 83 EC 20 48 8B 41 08 48 8B D9 8B 50 30 48 C1 EA 0C F6 C2 01 0F 85` |
| `ProcessTick` | `char __fastcall ProcessTick(__int64 a1, __int64 a2)` | `raw` | `0x7FFC2848AB10` | `0x6AB10` | `48 89 5C 24 20 55 57 41 57 48 81 EC F0 00 00 00 8B 7A 50 45 33 FF 44 38 3D ? ? ? ? 48 8B EA` |
| `QueueNewRequest` | `__int64 __fastcall QueueNewRequest(__int64 a1, __int64 a2)` | `raw` | `0x7FFC2863BCC0` | `0x21BCC0` | `48 89 6C 24 18 48 89 7C 24 20 41 56 48 83 EC 30 48 8B EA 48 8B F9 8B 0D ? ? ? ? BA 02 00 00` |
| `RegisterConCommand` | `_QWORD *__fastcall RegisterConCommand(_QWORD *a1, __int64 a2, __int128 *a3, __int64 a4, __int64 a5, __int128 *a6)` | `raw` | `0x7FFC2881DAC0` | `0x3FDAC0` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 57 48 83 EC 60 44 8B 15` |
| `RegisterConVar` | `__int128 *__fastcall RegisterConVar(_QWORD *a1, __int64 a2, __int64 a3, __int64 a4, __int128 *a5)` | `raw` | `0x7FFC2881C8D0` | `0x3FC8D0` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 54 41 56 41 57 48 81 EC D0 00 00` |
| `ReplyConnection` | `` | `raw` | `0x7FFC284C4AE0` | `0xA4AE0` | `48 8B C4 55 41 55 41 56` |
| `RunPrediction` | `void __fastcall RunPrediction(__int64 a1, unsigned int a2)` | `raw` | `0x7FFC284864B0` | `0x664B0` | `40 55 41 56 48 83 EC ? 80 B9` |
| `SetSignonState` | `char __fastcall SetSignonState(__int64 a1, unsigned int a2, unsigned int a3, __int64 a4)` | `raw` | `0x7FFC28480FA0` | `0x60FA0` | `44 89 44 24 18 89 54 24 10 55 53 56 57 41 55 41 56 41 57 48 8D 6C 24 D9 48 81 EC D0 00 00 00 8B` |
| `Tokenize` | `` | `raw` | `0x7FFC2881DF60` | `0x3FDF60` | `48 89 6C 24 20 4C 89 44 24 18 56 57 41 54 41 56 41 57 48 83 EC 70 48 8B F2 49 8B E8 8B 51 08 4C` |

## `inputsystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `AttachToWindow` | `int __fastcall AttachToWindow(__int64 a1, HWND a2)` | `raw` | `0x7FFC8B5839F0` | `0x39F0` | `48 89 5C 24 20 55 48 83 EC 20 48 63 41 30 48 8B EA 33 D2 48 8B D9 85 C0 7E 20 4C 8B C0 8B CA` |
| `EventHandler` | `void __fastcall SDL_EventHandler(__int64 a1, SDL_Event* event)` | `raw` | `0x7FFC8B584F01` | `0x4F01` | `53 48 81 EC ? ? ? ? 8B 02 48 8B DA 2D 00 04 00 00` |

## `matchmaking.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `InitializeGameSettings` | `char __fastcall InitializeGameSettings(__int64 a1)` | `raw` | `0x7FFC147CF270` | `0x10F270` | `40 53 48 81 EC 40 01 00 00 48 89 BC 24 58 01 00 00 48 8D 15 ? ? ? ? 48 8B F9 41 B0 01 48 8B 49 10 FF 15 ? ? ? ? 48 8B D8 48 85 C0 74 59` |

## `materialsystem2.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `ApplyMaterialVarsForBatch` | `` | `raw` | `0x7FFC7CBF8B80` | `0x18B80` | `4C 89 4C 24 20 4C 89 44 24 18 48 89 54 24 10 53 55 56 57 41 54 41 55 41 56 41 57 48 83 EC 78` |
| `BindIdentityInstanceIDBufferAndSetRenderState` | `char __fastcall BindIdentityInstanceIDBufferAndSetRenderState(__int64 *a1, __int64 a2, __int64 a3, __int64 a4)` | `stringref` | `0x7FFC7CC50000` | `0x70000` | `"BindIdentityInstanceIDBufferAndSetRenderState: GetMode == NULL? Can't Render\n"` |
| `BuildPassCommandData` | `int __fastcall BuildPassCommandData(__int64 a1, int a2, __int64 a3)` | `raw` | `0x7FFC7CBF8F80` | `0x18F80` | `89 54 24 10 55 53 56 57 41 54 41 55 41 56 41 57 48 8D AC 24 58 FE FF FF 48 81 EC A8 02 00 00` |
| `CacheGate` | `__int64 __fastcall CacheGate(__int64 a1, unsigned __int64 a2, __int64 a3, int a4, __int64 a5, int a6, char a7)` | `raw` | `0x7FFC7CC8E950` | `0xAE950` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 57 48 83 EC 60 80 39 00 45 8B D9` |
| `ComputeWorkItemsToSetupStaticCombosForMode` | `char __fastcall ComputeWorkItemsToSetupStaticCombosForMode(unsigned __int16 *a1, unsigned int a2, int *a3)` | `stringref` | `0x7FFC7CBF5F3C` | `0x15F3C` | `"CMaterialLayer::ComputeWorkItemsToSetupStaticCombosForMode(3154): Failed call to FindOrLoadStaticComboData()!\n"` |
| `CreateCommandBuffer` | `__int64 __fastcall CreateCommandBuffer(__int64 a1, __int64 a2, int a3, int a4, _DWORD *a5)` | `stringref` | `0x7FFC7CBF9820` | `0x19820` | `"\nCMaterialLayer::CreateCommandBuffer(4446): Find a graphics programmer! Trying to bind a "%s" shader that doesn't exist! for %s\n"` |
| `CreateMaterial` | `void* __fastcall CreateMaterial(void* a1, void** a2, const char* a3, void* a4, void* a5, char a6)` | `raw` | `0x7FFC7CC1C090` | `0x3C090` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 56 48 81 EC 10 01 00 00 48 8B 05 ? ? ? ? 4C 8B F2` |
| `DynamicShaderCompile` | `char __fastcall DynamicShaderCompile(__int64 a1, __int64 a2)` | `stringref` | `0x7FFC7CBF3FA0` | `0x13FA0` | `"CompileComboAndGetVariables_DynamicShaderCompile(), C:\buildworker\csgo_rel_win64\build\src\materialsystem2\material2.cpp:2786"` |
| `FindOrCreateStaticComboDataInCache` | `__int64 __fastcall FindOrCreateStaticComboDataInCache(__int64 a1, __int64 a2)` | `stringref` | `0x7FFC7CC8E0E0` | `0xAE0E0` | `"CVfxProgramData::FindOrCreateStaticComboDataInCache(4448): Error! Ref count !=0 for static combo data cache entry!\n"` |
| `FindOrLoadStaticComboData` | `__int64 __fastcall FindOrLoadStaticComboData(__int64 a1, __int64 a2, __int64 a3, __int64 a4, char a5)` | `stringref` | `0x7FFC7CC9DAE0` | `0xBDAE0` | `"Shader %s attribute "%s" has inconsistent value or type across multiple shaders of a feature combo! ["` |
| `FindParameter` | `__int64 __fastcall FindParameter(__int64 a1, __int64 a2)` | `raw` | `0x7FFC7CBF1E30` | `0x11E30` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 20 48 8B 59 20 48` |
| `FrameUpdate` | `__int64 __fastcall FrameUpdate(__int64 *a1)` | `raw` | `0x7FFC7CC1BAC0` | `0x3BAC0` | `48 89 4C 24 08 55 53 56 57 41 54 41 56 48 8B EC 48 83 EC 68 48 8D 05 ? ? ? ? 48 C7 45 C0` |
| `GetErrorMaterial` | `__int64 __fastcall GetErrorMaterial(__int64 a1, __int64 a2, __int64 a3, _QWORD *a4, char a5)` | `stringref` | `0x7FFC7CBF74D7` | `0x174D7` | `"CMaterialSystem2::GetErrorMaterial(529): GetErrorMaterial() called when m_pMaterialTypeManager == NULL!\n"` |
| `GetMode` | `__int64 __fastcall GetMode(__int64 a1, unsigned int *a2)` | `raw` | `0x7FFC7CBEBD40` | `0xBD40` | `48 89 5C 24 18 57 48 83 EC 30 8B 02 48 8B D9 39 05 ? ? ? ? 48 8B 0D ? ? ? ? 48 89 74 24` |
| `GetVertexShaderInputSignature` | `__int64 __fastcall GetVertexShaderInputSignature(__int64 a1)` | `raw` | `0x7FFC7CBEC8C0` | `0xC8C0` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 56 48 83 EC 30 F6 41 0B 01 4C 8B` |
| `Init` | `__int64 __fastcall Init(__int64 a1)` | `stringref` | `0x7FFC7CC16E40` | `0x36E40` | `"MaterialSystem2"` |
| `LoadShadersAndSetupModes` | `__int64 __fastcall LoadShadersAndSetupModes(__int64 a1, __int64 a2, unsigned int a3)` | `raw` | `0x7FFC7CBF0040` | `0x10040` | `44 89 44 24 18 48 89 54 24 10 53 56 41 54 41 55 48 81 EC 88 00 00 00 4C 8B E9 48 C7 44 24 60` |
| `PrepareSceneMaterial` | `float __fastcall PrepareSceneMaterial(__int64 a1, __int64 a2, float a3)` | `raw` | `0x7FFC7CBF1BE0` | `0x11BE0` | `48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 30 48 8B 59 ? 48 8B F2 48 63 79 ? 48 C1 E7 06` |
| `ProcessQueue` | `void __fastcall ProcessQueue(__int64 a1)` | `stringref` | `0x7FFC7CC1A5E0` | `0x3A5E0` | `"Compiling %i shaders:"` |
| `ReloadAndSync` | `void ReloadAndSync()` | `raw` | `0x7FFC7CC155C1` | `0x355C2` | `48 83 EC 20 48 8B 35 ? ? ? ? 48 8B CE E8 ? ? ? ? 48 8B CE E8 ? ? ? ? 80 BE A0 03 00 00 00 74 ?` |
| `SetVariableAndRenderState` | `` | `stringref` | `0x7FFC7CC0F9B0` | `0x2F9B0` | `"SetRenderStateValueFromVariable(1172): Unsupported render state type in material "%s"!\n"` |
| `UnloadAllMaterials` | `__int64 __fastcall UnloadAllMaterials(__int64 a1)` | `stringref` | `0x7FFC7CC19AA0` | `0x39AA0` | `"CMaterialSystem2::DynamicShaderCompile_UnloadAllMaterials(1084): ERROR!!! Shaders not freed before shader reload! (See spew above)\n\n"` |
| `UpdateParameter` | `_QWORD *__fastcall UpdateParameter(__int64 a1)` | `raw` | `0x7FFC7CBF2370` | `0x12370` | `48 89 7C 24 ? 41 56 48 83 EC ? 8B 81` |

## `networksystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `Init` | `` | `raw` | `0x7FFC24F3CED0` | `0xECED0` | `40 55 53 57 41 54 41 55 41 57 48 8D AC 24 98 FC FF FF 48 81 EC 68 04 00 00 4C 8B E9` |
| `ProcessMessages` | `` | `raw` | `0x7FFC24F0C090` | `0xBC090` | `48 8B C4 53 57 41 54 41 56 48 81 EC A8 00 00 00 48 89 70 D0 45 33 E4 4C 89 68 C8 48 8B D9 48 89` |
| `RegisterNetMessageHandlerAbstract` | `` | `raw` | `0x7FFC24F0CA10` | `0xBCA10` | `48 89 5C 24 10 48 89 6C 24 18 57 41 56 41 57 48 83 EC 50 4C 8B B4 24 90 00 00 00 41 8B D9` |
| `SendNetMessage` | `` | `raw` | `0x7FFC24F0E480` | `0xBE480` | `48 89 5C 24 10 48 89 6C 24 18 56 57 41 56 48 83 EC 40 41 0F B6 F0 48 8D 99 F8 73 00 00 4C 8B F2` |

## `panorama.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `DispatchEvent` | `void __fastcall DispatchEvent(int *a1, unsigned __int8 a2, __int64 a3)` | `raw` | `0x7FFC19D37100` | `0x97100` | `48 8B C4 48 89 58 18 88 50 10 55 56 57 41 54 41 55 41 56 41 57 48 8D A8 78 F7 FF FF 48 81 EC 50` |
| `GetPanelPointerFunctionPointer` | `` | `raw` | `0x7FFC19D4B5F0` | `0xAB5F0` | `4C 63 0A 4C 8B DA` |
| `MakeSymbolFunctionPointer` | `` | `raw` | `0x7FFC19D33FF0` | `0x93FF0` | `40 55 56 48 83 EC ? 48 63` |
| `OnDeletePanelFunctionPointer` | `` | `raw` | `0x7FFC19D4B240` | `0xAB240` | `48 85 D2 0F 84 ? ? ? ? 48 89 ? 24 ? 57 48 83 EC ? 48` |
| `RegisterEventHandlerFunctionPointer` | `` | `raw` | `0x7FFC19D4B950` | `0xAB950` | `48 89 5C 24 ? 66 89 54 24 ? 55 56 57 41 56 41 57 48 83 EC ? 48 8D 05 ? ? ? ? 48 C7 44 24 28 ? ? ? ? 48 89 44 24 ? 4D` |
| `RunFrame` | `__int64 __fastcall RunFrame(_QWORD *a1)` | `raw` | `0x7FFC19D483D0` | `0xA83D0` | `48 89 5C 24 10 48 89 6C 24 18 56 57 41 54 41 56 41 57 48 81 EC 80 00 00 00 45 33 F6 48 8B F1` |
| `RunScript` | `` | `raw` | `0x7FFC19D45E00` | `0xA5E00` | `48 89 5C 24 ? 4C 89 4C 24 ? 48 89 54 24 ? 55 56 57 41 54 41 55 41 56 41 57 48 8D` |
| `RunScriptFunctionPointer` | `` | `raw` | `0x7FFC19D45E00` | `0xA5E00` | `48 89 5C 24 ? 4C 89 4C 24 ? 48 89 54 24 ? 55 56 57 41 54 41 55 41 56 41 57 48 8D` |

## `particles.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `CParticleSystemMgr_CreateParticleCollection` | `__int64 __fastcall CParticleSystemMgr_CreateParticleCollection(__int64 a1, const char *a2, __int64 a3, __int64 a4, char a5, int a6, int a7)` | `raw` | `0x7FFC1F1C0DD0` | `0xA0DD0` | `4C 8B DC 49 89 5B 10 49 89 6B 18 49 89 73 20 57 41 56 41 57 48 81 EC 80 00 00 00 49 C7 43 B0 ? ? 00 00 48 8D 05 ? ? ? ? 49 89 43 A8` |
| `CParticleSystemMgr_FindParticleSystem` | `__int64 *__fastcall CParticleSystemMgr_FindParticleSystem(__int64 a1, __int64 *a2, const char *a3, char a4)` | `raw` | `0x7FFC1F1C0BC0` | `0xA0BC0` | `48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 57 41 56 41 57 48 81 EC 40 01 00 00 48 8D 05 ? ? ? ? 48 C7 44 24 28 ? ? 00 00 48 89 44 24 20` |
| `DrawArray` | `_BYTE *__fastcall DrawArray(__int64 a1, __int64 a2, __int64 a3, int a4, __int64 a5, __int64 a6, __int64 a7)` | `raw` | `0x7FFC1F1420B0` | `0x220B0` | `40 55 53 56 57 48 8D 6C 24` |
| `FindKeyVar` | `__int64 __fastcall FindKeyVar(const char *a1, unsigned int a2, int a3)` | `raw` | `0x7FFC1F15A650` | `0x3A650` | `48 89 5C 24 ? 57 48 81 EC ? ? ? ? 33 C0 8B DA` |
| `SetMaterialShaderType` | `void __fastcall SetMaterialShaderType(__int64 a1, int *a2)` | `raw` | `0x7FFC1F1BD8D0` | `0x9D8D0` | `48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 54 41 56 41 57 48 81 EC ? ? ? ? 4C 63 32` |

## `rendersystemdx11.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `BeginSubmittingDisplayLists` | `` | `stringref` | `0x7FFC26E4C430` | `0x3C430` | `"CRenderDeviceDx11::BeginSubmittingDisplayLists(1162): "` |
| `CompileShaderSourceMain` | `` | `stringref` | `0x7FFC26E4FA40` | `0x3FA40` | `"Shader compilation failed! Reported no errors.\n"` |
| `CreateConstantBuffer` | `` | `stringref` | `0x7FFC26E3F500` | `0x2F500` | `"CRenderDeviceBase::CreateConstantBuffer(1571): "` |
| `QueuePresentAndWait` | `` | `raw` | `0x7FFC26E44650` | `0x34650` | `40 55 53 57 41 54 41 55 48 8D 6C 24 C9 48 81 EC C0 00 00 00 48 8D 05 ? ? ? ? 4C 89 B4 24` |
| `ResizeBuffers` | `` | `raw` | `0x7FFC26E4DC70` | `0x3DC70` | `48 8B C4 55 53 56 57 41 54 48 8B EC 48 83 EC 70 4C 89 68 10 4D 8B E0 4C 89 70 18 4C 8B EA 4C 89` |
| `SetHardwareGammaRamp` | `` | `raw` | `0x7FFC26E4F6E0` | `0x3F6E0` | `48 89 5C 24 18 57 B8 B0 40 00 00 E8 ? ? ? ? 48 2B E0 0F 29 BC 24 90 40 00 00 0F 57 C9 0F 28` |
| `SetMode` | `` | `raw` | `0x7FFC26E49930` | `0x39930` | `44 89 4C 24 20 44 89 44 24 18 89 54 24 10 55 53 56 57 41 54 41 55 41 56 41 57 48 81 EC D8 02 00` |

## `resourcesystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `BlockingLoadResourceByName` | `` | `raw` | `0x7FFC7E777360` | `0x17360` | `40 53 55 57 48 81 EC 80 00 00 00 48 8B 01 49 8B E8 48 8B FA 48 8B D9 FF 90 98 01 00 00 83 F8 03` |
| `FindOrRegisterResourceByName` | `` | `raw` | `0x7FFC7E776D80` | `0x16D80` | `48 89 5C 24 18 48 89 74 24 20 57 48 81 EC A0 00 00 00 F7 02 FF FF FF 3F 41 0F B6 F8 48 8B DA 48` |
| `FrameUpdate` | `` | `raw` | `0x7FFC7E77C010` | `0x1C010` | `44 88 4C 24 20 44 89 44 24 18 89 54 24 10 55 56 41 54 41 55 41 56 48 8D 6C 24 A0 48 81 EC 60 01` |

## `scenesystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `AllocateAttributeListFunctionPointer` | `` | `raw` | `0x7FFC1F9C7D00` | `0xC7D00` | `40 55 48 83 EC ? 48 83 BA` |
| `BuildSceneInfoGpu` | `` | `raw` | `0x7FFC1F9850A0` | `0x850A0` | `4C 89 4C 24 20 4C 89 44 24 18 48 89 4C 24 08 55 48 8D AC 24 00 E3 FF FF B8 00 1E 00 00` |
| `CreateStaticShape` | `` | `raw` | `0x7FFC1F9B1A70` | `0xB1A70` | `48 8B C4 48 89 48 08 55 41 54 41 56 48 8D 68 D8 48 81 EC 10 01 00 00 4C 8B 65 50 48 8D 4D 80` |
| `CullView` | `` | `stringref` | `0x7FFC1F9E9270` | `0xE9270` | `"CSceneSystem::Thread_CullView(), C:\buildworker\csgo_rel_win64\build\src\scenesystem\scenesystem.cpp:3312"` |
| `DeleteObjectForReal` | `` | `raw` | `0x7FFC1F9CA530` | `0xCA530` | `40 53 56 41 54 48 83 EC 50 0F B6 82 9B 00 00 00 45 33 E4 48 8B DA 48 8B F1 F0 FF 8C 81 CC 30 00 00` |
| `DeleteSceneObjectFunctionPointer` | `` | `raw` | `0x7FFC1F9CB430` | `0xCB430` | `48 85 D2 0F 84 ? ? ? ? 48 8B C4 48 89 50` |
| `Dispatch` | `` | `raw` | `0x7FFC1F9EDD00` | `0xEDD00` | `48 8B C4 48 89 48 08 55 53 56 57 41 54 41 55 41 56 41 57 48 8D A8 B8 FE FF FF 48 81 EC 08 02 00` |
| `DrawAggeregateObject` | `` | `raw` | `0x7FFC1FA2CE30` | `0x12CE30` | `48 8B C4 4C 89 48 20 4C 89 40 ? 48 89 50 ? 55 53 41 57 48 8D A8` |
| `DrawAggregateSceneObjectArray` | `` | `raw` | `0x7FFC1F93BCB0` | `0x3BCB0` | `48 8B C4 48 89 50 ? 48 89 48 ? 55 53 56 57 41 54 41 55 41 56 41 57 48 8D A8 ? ? ? ? 48 81 EC ? ? ? ? 0F 29 70` |
| `DrawArrayLight` | `` | `raw` | `0x7FFC1F97AA40` | `0x7AA40` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 54 24` |
| `DrawObject_legacy` | `` | `raw` | `0x7FFC1F955B70` | `0x55B70` | `48 8B C4 53 57 41 54 48 81 EC D0 00 00 00 49 63 F9 49` |
| `DrawSkyboxArray` | `` | `raw` | `0x7FFC1FA4FA70` | `0x14FA70` | `45 85 C9 0F 8E ? ? ? ? 4C 8B DC 55` |
| `FrameUpdate` | `` | `raw` | `0x7FFC1F9E1C30` | `0xE1C30` | `48 8B C4 88 50 10 48 89 48 08 55 53 41 54 41 55 48 8D 68 A1 48 81 EC 98 00 00 00` |
| `GeneratePrimitives` | `` | `raw` | `0x7FFC1F9734A0` | `0x734A0` | `48 8B C4 48 89 58 08 48 89 50 10 55 56 57 41 54 41 55 41 56 41 57 48 81 EC ? ? ? ?` |
| `InitGfxObjects` | `` | `raw` | `0x7FFC1F9B3DB0` | `0xB3DB0` | `40 55 53 56 57 41 54 41 55 41 56 41 57 48 8D AC 24 08 FE FF FF 48 81 EC F8 02 00 00` |
| `RenderSceneDrawList` | `` | `raw` | `0x7FFC1F9ED9B0` | `0xED9B0` | `40 55 53 56 57 41 54 41 55 41 56 41 57 48 8D 6C 24 E1 48 81 EC D8 00 00 00 4C 8B 71 28 48 8B D9` |
| `ToneMapUpdate` | `` | `raw` | `0x7FFC1FA86DC0` | `0x186DC0` | `40 53 48 83 EC ? 48 8B D9 0F 29 74 24` |

## `schemasystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `InstallSchemaBindings` | `` | `raw` | `0x7FFC7E6975D0` | `0x375D0` | `40 53 48 83 EC 20 48 8B DA 48 8B D1 48 8D 0D ? ? ? ? E8 ? ? ? ? 85 C0 74 08 32 C0` |
| `RegisterModuleAndBuiltins` | `` | `raw` | `0x7FFC7E6706F0` | `0x106F0` | `48 89 54 24 10 53 56 57 41 55 41 56 41 57 48 83 EC 48 45 33 ED 49 63 C0 33 FF 44 89 AC 24 90 00` |
| `VerifySchemaBindingConsistency` | `` | `raw` | `0x7FFC7E6658F0` | `0x58F0` | `88 54 24 10 55 53 57 41 54 41 55 48 8B EC 48 81 EC 80 00 00 00 65 48 8B 04 25 58 00 00 00` |

## `server.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `AcceptInput` | `` | `raw` | `0x7FFC18667EE0` | `0x1237EE0` | `48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 49 8B F0 48 8B D9 48 8B 0D` |
| `AddEntityIOEvent` | `` | `raw` | `0x7FFC18641F00` | `0x1211F00` | `48 89 5C 24 18 4C 89 4C 24 20 48 89 4C 24 08 55 56 57 41 54 41 55 41 56 41 57 48 83 EC 20` |
| `BotNavIgnore` | `` | `raw` | `0x7FFC176F5EBA` | `0x2C5EBA` | `0F 84 ? ? ? ? 80 B8 ? ? ? ? 00 0F 84 ? ? ? ? 80 3D ? ? ? ? 00 74 15` |
| `CCSGameRules__sm_mapGcBanInformation` | `` | `raw` | `0x7FFC17CF28A7` | `0x8C28A7` | `48 8D 0D ? ? ? ? 48 89 45 ? 0F 11 45` |
| `CTakeDamageInfo` | `` | `raw` | `0x7FFC182A2290` | `0xE72290` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC ? 45 33 F6 48 C7 41` |
| `CanAcquire` | `` | `raw` | `0x7FFC17E88470` | `0xA58470` | `44 89 44 24 ? 48 89 54 24 ? 48 89 4C 24 ? 55 53 56 57 41 55 41 56 41 57 48 8B EC` |
| `CanUse` | `` | `raw` | `0x7FFC17EC8DF0` | `0xA98DF0` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 54 41 55 41 56 41 57 48 83 EC ? 48 8B 01 48 8B FA` |
| `CheckSteamBan` | `` | `raw` | `0x7FFC17D243B0` | `0x8F43B0` | `41 54 48 81 EC ? ? ? ? BA ? ? ? ? 48 8D 0D ? ? ? ? E8 ? ? ? ? 48 85 C0` |
| `CheckTransmit` | `` | `raw` | `0x7FFC1813E810` | `0xD0E810` | `48 8B C4 4C 89 48 ? 48 89 50 ? 48 89 48 ? 55 48 8D A8 ? ? ? ? 48 81 EC ? ? ? ? 48 89 58 ? 48 89 70` |
| `ClientPrint` | `` | `raw` | `0x7FFC17D7B340` | `0x94B340` | `48 85 C9 0F 84 ? ? ? ? 48 89 5C 24 ? 55` |
| `ClientPrintAll` | `` | `raw` | `0x7FFC182CBA00` | `0xE9BA00` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 83 EC ? 8B E9 49 8B D9` |
| `CreateEntityByName` | `` | `raw` | `0x7FFC17FA3210` | `0xB73210` | `48 83 EC 48 C6 44 24 30 00` |
| `DispatchParticleEffect` | `` | `raw` | `0x7FFC17FFFFD0` | `0xBCFFD0` | `48 89 5C 24 08 48 89 74 24 10 48 89 7C 24 18 4C 89 74 24 20 55 48 8D 6C 24 D1` |
| `DispatchSpawn` | `` | `raw` | `0x7FFC18058040` | `0xC28040` | `48 89 5C 24 ? 57 48 83 EC ? 48 8B DA 48 8B F9 48 85 C9 0F 84 ? ? ? ? 48 85 D2` |
| `EmitSoundFilter` | `` | `raw` | `0x7FFC182643D0` | `0xE343D0` | `40 53 48 83 EC ? 4C 89 4C 24 ? 48 8B D9 45 8B C8` |
| `EmitSoundParams` | `` | `raw` | `0x7FFC180AA010` | `0xC7A010` | `48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 55 48 8B EC 48 81 EC ? ? ? ? 33 C0` |
| `EndTouch` | `` | `raw` | `0x7FFC1876CCE0` | `0x133CCE0` | `40 53 41 55 48 83 EC ? 83 BA` |
| `EquipWeapon` | `` | `raw` | `0x7FFC17ECD130` | `0xA9D130` | `48 89 5C 24 ? 57 48 83 EC ? 48 83 79 ? ? 48 8B FA 48 8B D9 75 ? E8 ? ? ? ? 48 8B 53` |
| `FindEntityByClassName` | `` | `raw` | `0x7FFC17FB0EE0` | `0xB80EE0` | `48 83 EC 68 45 33 C9` |
| `FindEntityByName` | `` | `raw` | `0x7FFC17FB1420` | `0xB81420` | `48 81 EC 88 ? ? ? 4D 85 C0` |
| `FindUseEntity` | `` | `raw` | `0x7FFC17ECE470` | `0xA9E470` | `4C 89 44 24 ? F3 0F 11 4C 24 ? 55 53 56` |
| `FireOutputInternal` | `` | `raw` | `0x7FFC1865A030` | `0x122A030` | `4C 89 4C 24 ? 48 89 4C 24 ? 53 56` |
| `GameSystems` | `` | `raw` | `0x7FFC179594F6` | `0x5294F6` | `8B 05 ? ? ? ? 83 E8 ? 48 63 F8 0F 88` |
| `GetCSWeaponDataFromKey` | `` | `raw` | `0x7FFC17958A30` | `0x528A30` | `48 89 5C 24 ? 57 48 83 EC ? 33 FF 4C 8B CA 8B D9` |
| `GetEyeAngles` | `` | `raw` | `0x7FFC17F11640` | `0xAE1640` | `48 89 5C 24 ? 57 48 81 EC ? ? ? ? 48 8B F9 48 8B DA 48 8B 89 ? ? ? ? 48 85 C9` |
| `GetSpawnGroups` | `` | `raw` | `0x7FFC1799D930` | `0x56D930` | `40 56 48 83 EC ? 48 89 5C 24 ? 48 8D B1` |
| `GiveNamedItem` | `__int64 __fastcall GiveNamedItem(__int64 a1, const char *a2, int a3, __int64 a4, char a5, unsigned __int64 *a6)` | `raw` | `0x7FFC17E87670` | `0xA57670` | `48 89 5C 24 08 48 89 74 24 10 48 89 7C 24 20 44 89 44 24 18 55 41 54 41 55 41 56 41 57 48 8D AC 24 40 FF FF FF 48 81 EC C0 01 00 00 4D 8B E1 45 8B E8` |
| `GravityTouch` | `` | `raw` | `0x7FFC182B2E60` | `0xE82E60` | `48 89 5C 24 ? 57 48 83 EC ? 48 8B 02 48 8B F9 48 8B CA 48 8B DA FF 90 ? ? ? ? 84 C0 74 ? F3 0F 10 8F` |
| `IGameSystem_InitAllSystems_pFirst` | `` | `raw` | `0x7FFC17953DB3` | `0x523DB3` | `48 8B 1D ? ? ? ? 48 85 DB 0F 84 ? ? ? ? BD` |
| `IGameSystem_LoopPostInitAllSystems_pEventDispatcher` | `` | `raw` | `0x7FFC17959C19` | `0x529C19` | `48 39 1D ? ? ? ? 74 ? 39 05` |
| `Init` | `` | `raw` | `0x7FFC17FB92C0` | `0xB892C0` | `40 53 48 83 EC ? 48 8B 01 48 8B D9 FF 50 ? 48 8B 03` |
| `InputTestActivator` | `` | `raw` | `0x7FFC17F67AA0` | `0xB37AA0` | `48 89 5C 24 ? 57 48 83 EC ? 4C 8B 02` |
| `InputTriggerForActivatedPlayer` | `` | `raw` | `0x7FFC18172560` | `0xD42560` | `48 89 5C 24 18 56 48 83 EC 20 48 8B 1A` |
| `InputTriggerForAllPlayers` | `` | `raw` | `0x7FFC18172640` | `0xD42640` | `40 55 53 41 54 41 56 48 8B EC 48 83 EC ? 4C 8B F1` |
| `LegacyGameEventListener` | `` | `raw` | `0x7FFC17FB60F0` | `0xB860F0` | `48 8B 15 ? ? ? ? 48 85 D2 74 ? 83 F9 ? 77` |
| `NetworkStateChanged` | `` | `raw` | `0x7FFC1866BF70` | `0x123BF70` | `4C 8B C2 48 8B D1 48 8B 09` |
| `PostThink` | `` | `raw` | `0x7FFC17631DA0` | `0x201DA0` | `40 55 53 56 57 41 54 48 8D 6C 24 ? 48 81 EC ? ? ? ? 4C 89 AC 24` |
| `ProcessUsercmds` | `` | `raw` | `0x7FFC17F26D70` | `0xAF6D70` | `48 8B C4 44 88 48 20 44 89 40 18 48 89 50 10 53` |
| `Remove` | `` | `raw` | `0x7FFC17FE7EB0` | `0xBB7EB0` | `48 85 C9 74 ? 48 8B D1 48 8B 0D ? ? ? ?` |
| `RemovePlayerItem` | `` | `raw` | `0x7FFC17ECB8C0` | `0xA9B8C0` | `48 85 D2 0F 84 ? ? ? ? 48 89 5C 24 ? 57 48 83 EC ? 48 8B DA 48 8B F9 E8` |
| `Say` | `` | `raw` | `0x7FFC18095B40` | `0xC65B40` | `44 89 4C 24 ? 44 88 44 24` |
| `SayText2Filter` | `` | `raw` | `0x7FFC182CCCE0` | `0xE9CCE0` | `48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 55 41 56 41 57 48 8D 6C 24 ? 48 81 EC ? ? ? ? 45 0F B6 F0` |
| `SayTextFilter` | `` | `raw` | `0x7FFC182CD170` | `0xE9D170` | `48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 55 41 56 41 57 48 8D 6C 24 ? 48 81 EC ? ? ? ? 49 8B F8` |
| `SetEntityName` | `` | `raw` | `0x7FFC1866D4D0` | `0x123D4D0` | `48 89 5C 24 10 57 48 83 EC 20 48 8B D9 4C 8B C2` |
| `SetGravityScale` | `` | `raw` | `0x7FFC1781EA80` | `0x3EEA80` | `48 89 5C 24 ? 57 48 83 EC ? F3 0F 10 81 ? ? ? ? 48 8B F9 0F 29 74 24 ? 0F 28 F1 0F 2E C6 7A ? 74` |
| `SetGroundEntity` | `` | `raw` | `0x7FFC181B51C0` | `0xD851C0` | `48 89 5C 24 ? 55 56 57 41 55 41 57 48 83 EC ? 44 8B 89` |
| `SetModel` | `` | `raw` | `0x7FFC17F31770` | `0xB01770` | `40 53 48 83 EC ? 48 8B D9 4C 8B C2 48 8B 0D ? ? ? ? 48 8D 54 24 ? 48 8B 01 FF 50 ? 48 8B 54 24 ? 48 8B CB E8 ? ? ? ? 48 83 C4 ? 5B C3` |
| `SetMoveType` | `` | `raw` | `0x7FFC1781F580` | `0x3EF580` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC ? 41 0F B6 F0` |
| `SetOrAddAttributeValueByName` | `` | `raw` | `0x7FFC1834E9A0` | `0xF1E9A0` | `40 53 55 41 56 48 81 EC 90 00 00 00` |
| `SetPawn` | `` | `raw` | `0x7FFC17F34050` | `0xB04050` | `44 88 4C 24 ? 53 57 41 54 41 56 41 57 48 83 EC` |
| `StartTouch` | `` | `raw` | `0x7FFC17822620` | `0x3F2620` | `40 57 41 56 48 83 EC ? 48 8B 01` |
| `SwitchTeam` | `__int64 __fastcall SwitchTeam(__int64 a1, unsigned int a2)` | `raw` | `0x7FFC17E6A300` | `0xA3A300` | `40 53 57 48 81 EC 88 00 00 00 48 8B D9 8B FA 8B CA E8 ? ? ? ? 48 85 C0 0F 84 3A 02 00 00` |
| `TerminateRound` | `_BYTE *__fastcall TerminateRound(__int64 a1, __int64 a2, unsigned int a3, __int64 a4)` | `raw` | `0x7FFC17D4C8E0` | `0x91C8E0` | `48 8B C4 4C 89 48 20 48 89 48 08 55 56 41 56 41 57 48 8D 68 A1 48 81 EC E8 00 00 00 4C 8D B1` |
| `Think` | `double __fastcall Think(__int64 a1)` | `raw` | `0x7FFC17D34F10` | `0x904F10` | `40 55 53 41 55 41 57 48 8D 6C 24 C1 48 81 EC A8 00 00 00 80 79 48 00 4C 8B F9 4C 8B 2D` |
| `Touch` | `` | `raw` | `0x7FFC182CAF40` | `0xE9AF40` | `40 55 53 57 48 8D 6C 24 ? 48 81 EC ? ? ? ? 48 8B 02 48 8B F9` |
| `TraceFunc` | `` | `raw` | `0x7FFC17F8FD50` | `0xB5FD50` | `48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 54 41 56 41 57 48 81 EC ? ? ? ? 45 33 E4` |

## `soundsystem.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `PlayVSND` | `` | `raw` | `0x7FFC236FA1D0` | `0x2A1D0` | `48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 55 48 8D 6C 24 ? 48 81 EC ? ? ? ? 33 F6 C7 45 ? ? ? ? ? 4C 8B C1` |
| `PlayVSound` | `_UNKNOWN **__fastcall PlayVSound(__int64 a1, __int64 a2, int a3, int a4)` | `raw` | `0x7FFC23A19830` | `0x349830` | `48 8B C4 48 89 58 08 57 48 81 EC ? ? ? ? 33 FF 48 8B D9` |
| `SomeUtlSymbolFunc` | `__int64 __fastcall SomeUtlSymbolFunc(__int64 a1, unsigned int a2)` | `raw` | `0x7FFC23780730` | `0xB0730` | `48 89 74 24 18 57 48 83 EC 20 48 63 F2 48 8B F9 3B 71 30` |
| `StartSoundEvent` | `` | `raw` | `0x7FFC23887AC0` | `0x1B7AC0` | `40 53 55 56 48 83 EC 20 83 B9 ?? ?? ?? ?? 00 49 8B D8 48 8B F2 48 8B E9 74 ?? C7 02 00 00 00 00` |

## `tier0.dll`

| Name | Prototype | Resolve | VA | RVA | Pattern |
| --- | --- | --- | --- | --- | --- |
| `CreateInterface` | `void *__fastcall CreateInterface(const char *pName, int *pReturnCode)` | `raw` | `0x7FFC7B4E0B90` | `0x210B90` | `4C 8B 0D ? ? ? ? 4C 8B D2 4C 8B D9 4D 85 C9 74 2E 49 8B 41 08 4D 8B C3 4C 2B C0` |
| `LoadKV3` | `` | `raw` | `0x7FFC7B3F8E80` | `0x128E80` | `48 89 5C 24 08 57 48 83 EC 70 4C 8B D1 48 C7 C0 FF FF FF FF 48 FF C0 41 80 3C 00 00 75 F6` |
| `LoadKeyValues` | `` | `rel32` | `0x7FFC7B3F8F50` | `0x128F50` | `E8 ? ? ? ? 8B 4C 24 34 0F B6 D8` |
| `Plat_FloatTime` | `double __fastcall Plat_FloatTime()` | `raw` | `0x7FFC7B416AF0` | `0x146AF0` | `48 83 EC 28 48 83 3D ? ? ? ? 00 75 05 E8 ? ? ? ? 48 8D 4C 24 30 FF 15 ? ? ? ? 48 8B 4C 24 30 48 8B 05 ? ? ? ? 48 3B C8 73 05 48 8B C8 EB 07 48 89 0D ? ? ? ? 48 2B 0D ? ? ? ? 0F 57 C0 78 12` |
| `Plat_GetTime` | `unsigned __int64 __fastcall Plat_GetTime()` | `raw` | `0x7FFC7B416930` | `0x146930` | `48 83 EC 28 48 8D 4C 24 30 E8 ? ? ? ? 48 8B 44 24 30 48 83 C4 28 C3` |
| `Plat_MSTime` | `unsigned __int64 __fastcall Plat_MSTime()` | `raw` | `0x7FFC7B416B70` | `0x146B70` | `40 53 48 83 EC 20 48 8B 1D ? ? ? ? 48 85 DB 75 0C E8 ? ? ? ? 48 8B 1D ? ? ? ? 48 8D 4C 24 30 FF 15 ? ? ? ? 48 8B 44 24 30 48 8B 0D ? ? ? ? 48 3B C1 73 05 48 8B C1 EB 07 48 89 05 ? ? ? ? 48 2B 05 ? ? ? ? 33 D2 48 F7 F3 48 8B C8 48 69 C2 E8 03 00 00 69 C9 E8 03 00 00` |
| `UtlBuffer` | `` | `raw` | `0x7FFC7B323F10` | `0x53F10` | `48 89 5C 24 ? 57 48 83 EC ? 8B 41 ? 8D 7A` |

