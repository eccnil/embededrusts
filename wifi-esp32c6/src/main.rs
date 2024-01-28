use std::{thread::sleep, time::Duration};

use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};
use toml_cfg::toml_config;

#[toml_config]
pub struct Setup {
    #[default("no_ssid")]
    ssid: &'static str,
    #[default("")]
    password: &'static str,
    #[default("")]
    mqtt_server: &'static str,
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // geting setup (passwords and config)
    let setup: Setup = SETUP;
    log::info!("config gets {}, mqtt {}", setup.ssid, setup.mqtt_server);

    //taking singletone things
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    //create wifi driver
    let mut wifi = EspWifi::new(peripherals.modem, sysloop, Some(nvs)).unwrap();
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: setup.ssid.try_into().unwrap(),
        password: setup.password.try_into().unwrap(),
        auth_method: AuthMethod::None,
        ..Default::default()
    }))
    .unwrap();

    //start wifi
    wifi.start().unwrap();
    wifi.connect().unwrap();

    //confirm connection
    while !wifi.is_connected().unwrap() {
        let _config = wifi.get_configuration().unwrap();
    }

    //wait to get ip
    log::info!("conectado");
    while !wifi.sta_netif().is_up().unwrap() {
        sleep(Duration::from_millis(100));
    }
    let ip = wifi.sta_netif().get_ip_info().unwrap();
    log::info!("{:?}", ip);
}
