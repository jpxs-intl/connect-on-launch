mod addresses;
mod util;

use clap::Parser;
use retour::RawDetour;
use std::mem;
use std::sync::OnceLock;
use std::{cell::Cell, net::Ipv4Addr};
use util::*;

#[cfg(target_os = "windows")]
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};

#[derive(Parser, Debug)]
struct ServerInfo {
    #[arg(long)]
    address: Ipv4Addr,

    #[arg(long)]
    port: u16,

    #[arg(long)]
    passworded: bool,
}

static SERVER_ADDRESS: OnceLock<ServerInfo> = const { OnceLock::new() };
static BASE_ADDRESS: OnceLock<usize> = const { OnceLock::new() };

static DRAW_ORIGINAL: OnceLock<fn()> = const { OnceLock::new() };
static DRAW_HOOK: OnceLock<RawDetour> = const { OnceLock::new() };

thread_local! {
    static HAS_CONNECTED: Cell<bool> = const { Cell::new(false) };
}

fn connect_game_to_server(server_info: &ServerInfo) {
    let connect_address: *mut u32 = address_from_base(addresses::SERVER_IP_ADDRESS) as *mut u32;
    let connect_port: *mut u16 = address_from_base(addresses::SERVER_PORT_ADDRESS) as *mut u16;
    let auth_address: *mut u32 = address_from_base(addresses::AUTH_IP_ADDRESS) as *mut u32;
    let auth_port: *mut u16 = address_from_base(addresses::AUTH_PORT_ADDRESS) as *mut u16;
    let passworded: *mut u32 = address_from_base(addresses::SERVER_PASSWORDED_ADDRESS) as *mut u32;
    let game_state: *mut u32 = address_from_base(addresses::GAME_STATE_ADDRESS) as *mut u32;

    let converted_address = server_info.address.to_bits();

    unsafe {
        *connect_address = converted_address;
        *connect_port = server_info.port;
        *auth_address = converted_address;
        *auth_port = server_info.port;
        *passworded = u32::from(!server_info.passworded);
        *game_state = 2;
    }
}

fn connect_hook() {
    if HAS_CONNECTED.get() {
        if let Some(func) = DRAW_ORIGINAL.get() {
            func();
        }

        return;
    }

    let Some(server_info) = SERVER_ADDRESS.get() else {
        return;
    };

    connect_game_to_server(server_info);

    HAS_CONNECTED.set(true);
}

#[cfg(target_os = "windows")]
#[unsafe(no_mangle)]
extern "system" fn DllMain(_: HINSTANCE, reason: DWORD, _: LPVOID) -> BOOL {
    use winapi::um::winnt::DLL_PROCESS_ATTACH;

    if reason != DLL_PROCESS_ATTACH {
        return 1;
    }

    initialize();
    1
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn initialize() {
    let Some((base_address, _)) = get_process_base() else {
        println!("Failed to retrieve Sub Rosa base address!");
        return;
    };
    BASE_ADDRESS.set(base_address).unwrap();

    let cli_info = ServerInfo::try_parse();

    if let Ok(server_info) = cli_info {
        SERVER_ADDRESS.set(server_info).unwrap();
    } else {
        println!("Failed to retrieve server info from command line!");
        return;
    }

    // Calculate function offset and cast to function ptr
    let func_ptr = address_from_base(addresses::HOOK_ADDRESS);

    // Create and enable hook
    let hook = unsafe { RawDetour::new(func_ptr, connect_hook as *const ()) }.unwrap();
    unsafe { hook.enable() }.unwrap();

    // Set the thread local for original function ptr to the trampoline
    DRAW_ORIGINAL
        .set(unsafe { mem::transmute::<&(), fn()>(hook.trampoline()) })
        .unwrap();
    DRAW_HOOK.set(hook).unwrap();
}
