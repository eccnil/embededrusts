use toml_cfg::toml_config;

#[toml_config]
pub struct Setup {
    #[default("no_ssid")]
    pub ssid: &'static str,
    #[default("")]
    pub password: &'static str,
    #[default("")]
    pub mqtt_server: &'static str,
}

impl Setup {
    pub fn load() -> Self {
        SETUP
    }
}
