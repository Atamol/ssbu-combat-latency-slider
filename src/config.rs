const CONFIG_DIR: &str = "sd:/config/ssbu-better-latency-slider";
const CONFIG_PATH: &str = "sd:/config/ssbu-better-latency-slider/config.txt";

pub const DEFAULT_LATENCY: u8 = 2;
const MAX_LATENCY: u8 = 3;

const DEFAULT_CONFIG: &str = "\
# ssbu-better-latency-slider config
#
# 起動時に適用されるレイテンシの初期値 (0-3)
# ゲーム中は十字キーで上書き可能 (左0 / 上1 / 右2 / 下3)

default_latency=2
";

pub fn load_default_latency() -> u8 {
    let content = match std::fs::read_to_string(CONFIG_PATH) {
        Ok(c) => c,
        Err(_) => {
            let _ = std::fs::create_dir_all(CONFIG_DIR);
            let _ = std::fs::write(CONFIG_PATH, DEFAULT_CONFIG);
            return DEFAULT_LATENCY;
        }
    };

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(val) = line.strip_prefix("default_latency=") {
            if let Ok(n) = val.trim().parse::<u8>() {
                return n.min(MAX_LATENCY);
            }
        }
    }

    DEFAULT_LATENCY
}
