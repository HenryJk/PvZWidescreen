#![allow(non_snake_case)]

#[repr(C)]
pub struct IDirect3D7 {
    unknown: [u8; 4],
}



#[repr(C)]
pub struct StdBasicString {
    unknown: [u8; 28],
}

#[repr(C)]
pub struct StdVector {
    unknown: [u8; 16],
}

#[repr(C)]
pub struct StdMap {
    unknown: [u8; 12],
}

#[repr(C)]
pub struct StdList {
    unknown: [u8; 12],
}

#[repr(C)]
pub struct StdSet {
    unknown: [u8; 12],
}

#[repr(C)]
pub struct ResourceManager {
    unknown: [u8; 204],
}

#[repr(C)]
pub struct InternetManager {}

#[repr(C)]
pub struct BetaSupport {
    unknown: [u8; 592],
}

#[repr(C)]
pub struct GameSelector {
    unknown: [u8; 300],
}

#[repr(C)]
pub struct SeedChooserScreen {
    unknown: [u8; 3392],
}

#[repr(C)]
pub struct AwardScreen {
    unknown: [u8; 156],
}

#[repr(C)]
pub struct CreditScreen {
    unknown: [u8; 248],
}

#[repr(C)]
pub struct ChallengeScreen {
    unknown: [u8; 476],
}

#[repr(C)]
pub struct TodFoley {
    unknown: [u8; 18040],
}

#[repr(C)]
pub struct PoolEffect {}

#[repr(C)]
pub struct ZenGarden {}

#[repr(C)]
pub struct EffectSystem {}

#[repr(C)]
pub struct ReanimatorCache {}

#[repr(C)]
pub struct ProfileMgr {}

#[repr(C)]
pub struct PlayerInfo {}

#[repr(C)]
pub struct LevelStats {}

#[repr(C)]
pub struct PopDRMComm {}

#[repr(C)]
pub struct Music {}

#[repr(C)]
pub struct D3DDEVICEDESC7 {}