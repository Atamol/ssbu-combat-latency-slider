// The mod draws nothing on screen, so `cargo skyline listen` is the only way to see what ran
use std::sync::atomic::{AtomicI16, AtomicU32, Ordering};
use std::sync::OnceLock;
use std::time::Instant;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";

static START: OnceLock<Instant> = OnceLock::new();
static LAST_LOGGED: AtomicI16 = AtomicI16::new(-1);
static CALLS: AtomicU32 = AtomicU32::new(0);

fn emit(color: &str, tag: &str, args: std::fmt::Arguments<'_>) {
    let ms = START.get_or_init(Instant::now).elapsed().as_millis();
    println!(
        "{}[+{:>4}.{:03}s]{} {}{}[slider:{}]{} {}",
        DIM,
        ms / 1000,
        ms % 1000,
        RESET,
        BOLD,
        color,
        tag,
        RESET,
        args
    );
}

pub fn startup(latency: u8) {
    emit(CYAN, "cfg", format_args!("startup latency={latency}"));
}

#[cfg(not(feature = "fixed"))]
pub fn changed(latency: u8, live: bool) {
    let note = match live {
        true => "  (written live)",
        false => "",
    };
    emit(MAGENTA, "key", format_args!("{latency}F{note}"));
}

pub fn applied(game_wanted: u8, latency: u8) {
    let n = CALLS.fetch_add(1, Ordering::Relaxed) + 1;
    let signature = ((game_wanted as i16) << 4) | latency as i16;
    let changed = LAST_LOGGED.swap(signature, Ordering::SeqCst) != signature;
    // Powers of ten, so a per frame call shows itself without flooding the log
    let milestone = matches!(n, 1 | 10 | 100 | 1000 | 10000);
    if !changed && !milestone {
        return;
    }
    emit(
        YELLOW,
        "net",
        format_args!("handshake latency={latency}   (game wanted {game_wanted})   call #{n}"),
    );
}
