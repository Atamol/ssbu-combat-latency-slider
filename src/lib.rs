#![feature(restricted_std)]

#[cfg(not(feature = "fixed"))]
mod config;
mod offsets;

use offsets::LOC_SET_ONLINE_LATENCY;
use skyline::hooks::InlineCtx;

#[cfg(not(feature = "fixed"))]
use offsets::{LOC_UPDATE_CSS, LOC_UPDATE_ROOM};

#[cfg(feature = "0f")]
static CURRENT_LATENCY: u8 = 0;
#[cfg(feature = "1f")]
static CURRENT_LATENCY: u8 = 1;
#[cfg(feature = "2f")]
static CURRENT_LATENCY: u8 = 2;
#[cfg(feature = "3f")]
static CURRENT_LATENCY: u8 = 3;

#[cfg(not(feature = "fixed"))]
static mut CURRENT_LATENCY: u8 = config::DEFAULT_LATENCY;

#[cfg(not(feature = "fixed"))]
unsafe fn handle_user_input() {
    if ninput::any::is_press(ninput::Buttons::LEFT) {
        CURRENT_LATENCY = 0;
    } else if ninput::any::is_press(ninput::Buttons::UP) {
        CURRENT_LATENCY = 1;
    } else if ninput::any::is_press(ninput::Buttons::RIGHT) {
        CURRENT_LATENCY = 2;
    } else if ninput::any::is_press(ninput::Buttons::DOWN) {
        CURRENT_LATENCY = 3;
    }
}

#[cfg(not(feature = "fixed"))]
#[skyline::hook(offset = LOC_UPDATE_ROOM.get_offset_in_memory().unwrap(), inline)]
unsafe fn update_room_hook(_: &InlineCtx) {
    handle_user_input();
}

#[cfg(not(feature = "fixed"))]
#[skyline::hook(offset = LOC_UPDATE_CSS.get_offset_in_memory().unwrap())]
unsafe fn update_css_hook(arg: u64) {
    handle_user_input();
    call_original!(arg)
}

#[skyline::hook(offset = LOC_SET_ONLINE_LATENCY.get_offset_in_memory().unwrap(), inline)]
unsafe fn set_online_latency_hook(ctx: &InlineCtx) {
    // x19 points to the latency byte the game is about to use
    *(ctx.registers[19].x() as *mut u8) = CURRENT_LATENCY;
}

#[skyline::main(name = "ssbu-better-latency-slider")]
pub fn main() {
    #[cfg(not(feature = "fixed"))]
    unsafe {
        CURRENT_LATENCY = config::load_default_latency();

        if ensure_hooks!(LOC_UPDATE_ROOM, LOC_UPDATE_CSS, LOC_SET_ONLINE_LATENCY) {
            skyline::install_hooks!(update_room_hook, update_css_hook, set_online_latency_hook);
        }
    }

    #[cfg(feature = "fixed")]
    if ensure_hooks!(LOC_SET_ONLINE_LATENCY) {
        skyline::install_hooks!(set_online_latency_hook);
    }
}
