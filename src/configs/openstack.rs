use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct OspConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for OspConfig<'a> {
    fn default() -> Self {
        OspConfig {
            format: "on [$symbol$cloud(\\($project\\))]($style) ",
            symbol: "☁️  ",
            style: "bold yellow",
            disabled: false,
        }
    }
}
