use toml_cfg::toml_config;

/// loads configuration
#[toml_config]
pub struct Setup {
    /// Access Point SSID
    #[default("no_ssid")]
    pub ssid: &'static str,
    /// Access Point password
    #[default("")]
    pub password: &'static str,
    /// mqtt server url, this module is not configured to have any kind of security
    #[default("mqtt://192.168.0.1:1883")]
    pub mqtt_server: &'static str,
}

impl Setup {
    /// loads configuration from `cfg.toml` file of the main crate if it exists
    /// this file needs to have this format (assuming the main crate is
    /// named `wifi-esp32c6`)
    ///
    /// ```toml
    /// [wifi-esp32c6]
    /// ssid = "your_ap_ssid"
    /// password = "you_ap_fancy_password"
    /// mqtt_server = "mqtt://192.168.0.1:1883"
    /// ```
    pub fn load() -> Self {
        SETUP
    }
}
