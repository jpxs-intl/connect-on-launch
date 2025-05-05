#[cfg(target_os = "windows")]
pub mod platform {
    pub const HOOK_ADDRESS: usize = 0xFE1B0;

    pub const SERVER_IP_ADDRESS: usize = 0x6D04B1D4;
    pub const SERVER_PORT_ADDRESS: usize = 0x6D04B1D8;
    pub const AUTH_IP_ADDRESS: usize = 0x6D04B1E8;
    pub const AUTH_PORT_ADDRESS: usize = 0x6D04B1EC;
    pub const SERVER_PASSWORDED_ADDRESS: usize = 0x2B15FF3C;
    pub const GAME_STATE_ADDRESS: usize = 0x43EBFAA4;
}

#[cfg(target_os = "linux")]
pub mod platform {
    pub const HOOK_ADDRESS: usize = 0x14F4AA;

    pub const SERVER_IP_ADDRESS: usize = 0x1C8DAD54;
    pub const SERVER_PORT_ADDRESS: usize = 0x1C8DAD58;
    pub const AUTH_IP_ADDRESS: usize = 0x1C8DAD68;
    pub const AUTH_PORT_ADDRESS: usize = 0x1C8DAD6C;
    pub const SERVER_PASSWORDED_ADDRESS: usize = 0x1A47D85C;
    pub const GAME_STATE_ADDRESS: usize = 0x1CE18FE4;
}

pub use platform::*;
