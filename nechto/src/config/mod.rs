use std::collections::HashMap;
use std::path::Path;

use tracing::error;

pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub render: RenderConfig,
}

#[derive(Clone)]
pub struct RenderConfig {
    pub vulkan_enable_debug: bool,
}

impl Config {
    pub fn parse(text: &str) -> Self {
        let map = ValueMap::parse_ini(text);

        Self {
            window_width: map.u32("window.width", 800),
            window_height: map.u32("window.height", 600),
            render: RenderConfig {
                vulkan_enable_debug: map.bool("vulkan.enable_debug", false),
            },
        }
    }

    pub fn parse_file<P: AsRef<Path>>(path: P) -> Self {
        let text = std::fs::read_to_string(path).unwrap_or("".to_owned());

        Self::parse(&text)
    }
}

struct ValueMap {
    values: HashMap<String, String>,
}

impl ValueMap {
    pub fn parse_ini(text: &str) -> Self {
        let mut values = HashMap::new();

        for line in text.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with('#') {
                continue;
            }

            let Some((left, right)) = line.split_once('=') else {
                error!("skipped invalid config line: {}", line);

                continue;
            };

            values.insert(left.trim().to_owned(), right.trim().to_owned());
        }

        Self { values }
    }

    pub fn string(&self, key: &str, default: String) -> String {
        self.values
            .get(key)
            .map(|value| value.to_owned())
            .unwrap_or(default)
    }

    pub fn u32(&self, key: &str, default: u32) -> u32 {
        self.values
            .get(key)
            .map(|value| value.parse().unwrap())
            .unwrap_or(default)
    }

    pub fn bool(&self, key: &str, default: bool) -> bool {
        self.values
            .get(key)
            .map(|value| value.parse().unwrap())
            .unwrap_or(default)
    }
}
