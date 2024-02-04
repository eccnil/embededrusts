use toml_cfg::toml_config;

#[toml_config]
pub struct Setup {
    #[default("no_ssid")]
    pub ssid: &'static str,
    #[default("")]
    pub password: &'static str,
    #[default("mqtt://192.168.0.1:1883")]
    pub mqtt_server: &'static str,
}

impl Setup {
    pub fn load() -> Self {
        SETUP
    }
}
