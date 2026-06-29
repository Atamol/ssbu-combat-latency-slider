use once_cell::sync::OnceCell;

#[macro_export]
macro_rules! ensure_hooks {
    ($($f:expr),*) => {{
        let mut is_successful = true;
        $(
            if $f.get_offset_in_memory().is_none() && is_successful {
                skyline::error::show_error(
                    420,
                    "ssbu-better-latency-slider failed to load.\0",
                    format!("Error: Failed to find {} in memory.\n\n{}\n\n{}\0",
                    $f.location_name,
                    "This may be the result of an incompatible mod being loaded, or SSBU being updated.",
                    "The mod will NOT be enabled, but you can continue playing normally."
                    ).as_str()
                );

                is_successful = false;
            }
        )*

        is_successful
    }};
}

fn byte_search(needle: &[u8]) -> Option<usize> {
    let search_space = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8;
        let end = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const u8;

        let length = end.offset_from(start) as usize;

        std::slice::from_raw_parts(start, length)
    };

    search_space.windows(needle.len()).position(|w| w == needle)
}

pub struct SSBUMemoryLocation<'a> {
    signature: &'a [u8],

    start_offset: isize,

    pub location_name: &'a str,

    cached_offset: OnceCell<Option<usize>>,
}

impl SSBUMemoryLocation<'_> {
    pub fn get_offset_in_memory(&self) -> Option<usize> {
        *self.cached_offset.get_or_init(|| unsafe {
            let r = byte_search(self.signature)
                .map(|e| (e as *const u8).offset(self.start_offset) as usize);

            if let Some(r) = r {
                println!(
                    "[ssbu-better-latency-slider] Found {} at {r:#09x?}",
                    self.location_name
                );
            }

            r
        })
    }
}

pub static LOC_SET_ONLINE_LATENCY: SSBUMemoryLocation = SSBUMemoryLocation {
    signature: &[
        0xfd, 0x7b, 0x42, 0xa9, 0xf4, 0x4f, 0x41, 0xa9, 0xe8, 0x07, 0x43, 0xfc, 0xc0, 0x03, 0x5f,
        0xd6, 0x60, 0x1e, 0x44, 0x38, 0xf6, 0xff, 0xff, 0x17,
    ],
    start_offset: 0,
    location_name: "set_online_latency",
    cached_offset: OnceCell::new(),
};

#[cfg(not(feature = "fixed"))]
pub static LOC_UPDATE_ROOM: SSBUMemoryLocation = SSBUMemoryLocation {
    signature: &[
        0xff, 0x03, 0x01, 0xd1, 0xf6, 0x57, 0x01, 0xa9, 0xf4, 0x4f, 0x02, 0xa9, 0xfd, 0x7b, 0x03,
        0xa9, 0xfd, 0xc3, 0x00, 0x91, 0x08, 0xcc, 0x41, 0xb9, 0xf3, 0x03, 0x00, 0xaa, 0xa8, 0x04,
        0x00, 0x35, 0x68, 0xf6, 0x40, 0xf9, 0x08, 0x41, 0x40, 0xf9, 0xa9, 0x64, 0x83, 0x52, 0xea,
        0x03, 0x00, 0x32, 0x0a, 0x69, 0x29, 0x38, 0x28, 0xd5, 0x01, 0xd0, 0x08, 0xed, 0x43, 0xf9,
        0xa8, 0x03, 0x00, 0xb4, 0x15, 0x05, 0x40, 0xf9, 0xb4, 0x22, 0x03, 0x91, 0xe0, 0x03, 0x14,
        0xaa,
    ],
    start_offset: 0,
    location_name: "update_room",
    cached_offset: OnceCell::new(),
};

#[cfg(not(feature = "fixed"))]
pub static LOC_UPDATE_CSS: SSBUMemoryLocation = SSBUMemoryLocation {
    signature: &[
        0xea, 0x0f, 0x18, 0xfc, 0xe9, 0x23, 0x01, 0x6d, 0xfc, 0x6f, 0x02, 0xa9, 0xfa, 0x67,
        0x03, 0xa9, 0xf8, 0x5f, 0x04, 0xa9, 0xf6, 0x57, 0x05, 0xa9, 0xf4, 0x4f, 0x06, 0xa9,
        0xfd, 0x7b, 0x07, 0xa9, 0xfd, 0xc3, 0x01, 0x91, 0xff, 0x83, 0x0b, 0xd1, 0x08, 0x3c,
        0x41, 0xb9, 0xf3, 0x03, 0x00, 0xaa, 0x08, 0x01, 0x00, 0x35, 0x68, 0xce, 0x40, 0xf9,
        0x08, 0x01, 0x40, 0xf9, 0x00, 0x01, 0x40, 0xf9, 0xe1, 0x03, 0x00, 0x32, 0xe7, 0x8a,
        0x75, 0x94, 0x60, 0xfa, 0x46, 0xf9, 0x1d, 0xf2, 0x62, 0x94,
    ],
    start_offset: 0,
    location_name: "update_css",
    cached_offset: OnceCell::new(),
};
