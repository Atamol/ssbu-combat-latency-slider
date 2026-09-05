#![feature(restricted_std)]

#[cfg(not(feature = "fixed"))]
mod config;
mod log;
mod offsets;

use offsets::LOC_SET_ONLINE_LATENCY;
use skyline::hooks::InlineCtx;

#[cfg(not(feature = "fixed"))]
use offsets::{LOC_UPDATE_CSS, LOC_UPDATE_ROOM};

#[cfg(not(feature = "fixed"))]
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Writes the latency mid match, which needs a poll thread because the css and room hooks stop
/// running there. Off since setting it at the character select is enough
#[cfg(not(feature = "fixed"))]
const MID_MATCH_INPUT: bool = false;

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

// The game computes the latency once per connection but reads the byte every frame, so holding the
// pointer is what lets a press land mid match
#[cfg(not(feature = "fixed"))]
static LATENCY_PTR: AtomicUsize = AtomicUsize::new(0);

// Reading the pad before nn::hid is up aborts, and the css and room hooks only run well past that,
// so the first of those releases the poll
#[cfg(not(feature = "fixed"))]
static HID_READY: AtomicBool = AtomicBool::new(false);

/// Npad slots to read. Online never has more than two local controllers, so sweeping all nine that
/// `combined_buttons` walks is wasted work
#[cfg(not(feature = "fixed"))]
const SCANNED_PADS: [u32; 3] = [0, 1, 0x20];

#[cfg(not(feature = "fixed"))]
unsafe fn pressed_buttons() -> ninput::Buttons {
    let mut buttons = ninput::Buttons::default();
    for id in SCANNED_PADS {
        if let Some(pad) = ninput::Controller::get_from_id(id) {
            buttons |= pad.pressed_buttons;
        }
    }
    buttons
}

#[cfg(not(feature = "fixed"))]
unsafe fn handle_user_input() {
    let pressed = pressed_buttons();

    let previous = CURRENT_LATENCY;
    if pressed.contains(ninput::Buttons::LEFT) {
        CURRENT_LATENCY = 0;
    } else if pressed.contains(ninput::Buttons::UP) {
        CURRENT_LATENCY = 1;
    } else if pressed.contains(ninput::Buttons::RIGHT) {
        CURRENT_LATENCY = 2;
    } else if pressed.contains(ninput::Buttons::DOWN) {
        CURRENT_LATENCY = 3;
    }

    if CURRENT_LATENCY == previous {
        return;
    }

    let mut live = false;
    if MID_MATCH_INPUT {
        let ptr = LATENCY_PTR.load(Ordering::SeqCst);
        if ptr != 0 {
            *(ptr as *mut u8) = CURRENT_LATENCY;
            live = true;
        }
    }
    log::changed(CURRENT_LATENCY, live);
}

// The css and room hooks do not run during a match, so the input is polled from its own thread
#[cfg(not(feature = "fixed"))]
fn spawn_input_poll() {
    std::thread::spawn(|| loop {
        if HID_READY.load(Ordering::Relaxed) {
            unsafe { handle_user_input() };
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    });
}

#[cfg(not(feature = "fixed"))]
#[skyline::hook(offset = LOC_UPDATE_ROOM.get_offset_in_memory().unwrap(), inline)]
unsafe fn update_room_hook(_: &InlineCtx) {
    HID_READY.store(true, Ordering::Relaxed);
    handle_user_input();
}

#[cfg(not(feature = "fixed"))]
#[skyline::hook(offset = LOC_UPDATE_CSS.get_offset_in_memory().unwrap())]
unsafe fn update_css_hook(arg: u64) {
    HID_READY.store(true, Ordering::Relaxed);
    handle_user_input();
    call_original!(arg)
}

#[skyline::hook(offset = LOC_SET_ONLINE_LATENCY.get_offset_in_memory().unwrap(), inline)]
unsafe fn set_online_latency_hook(ctx: &InlineCtx) {
    // x19 points to the latency byte the game is about to use
    let ptr = ctx.registers[19].x() as *mut u8;
    log::applied(*ptr, CURRENT_LATENCY);
    *ptr = CURRENT_LATENCY;

    #[cfg(not(feature = "fixed"))]
    if MID_MATCH_INPUT {
        LATENCY_PTR.store(ptr as usize, Ordering::SeqCst);
    }
}

#[skyline::main(name = "ssbu-better-latency-slider")]
pub fn main() {
    #[cfg(feature = "fixed")]
    log::startup(CURRENT_LATENCY);

    #[cfg(not(feature = "fixed"))]
    unsafe {
        CURRENT_LATENCY = config::load_default_latency();
        log::startup(CURRENT_LATENCY);

        if ensure_hooks!(LOC_UPDATE_ROOM, LOC_UPDATE_CSS, LOC_SET_ONLINE_LATENCY) {
            skyline::install_hooks!(update_room_hook, update_css_hook, set_online_latency_hook);
        }
        if MID_MATCH_INPUT {
            spawn_input_poll();
        }
    }

    #[cfg(feature = "fixed")]
    if ensure_hooks!(LOC_SET_ONLINE_LATENCY) {
        skyline::install_hooks!(set_online_latency_hook);
    }
}
