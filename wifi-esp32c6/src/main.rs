use std::{
    thread::{self, sleep},
    time::Duration,
};

use embedded_svc::{
    mqtt::client::{Event, QoS},
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    mqtt::client::{EspMqttClient, EspMqttEvent, MqttClientConfiguration},
    nvs::EspDefaultNvsPartition,
    wifi::EspWifi,
};
use log::error;
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

    //mqtt client creation
    let mqtt_config = MqttClientConfiguration {
        client_id: "esptest".into(),
        ..Default::default()
    };
    let (mut mqtt_client, mut mqtt_connection) =
        EspMqttClient::new(setup.mqtt_server, &mqtt_config).unwrap();

    thread::spawn(move || {
        log::info!("entering mqtt loop");
        while let msg = mqtt_connection.next() {
            match msg {
                Err(e) => log::error!("mqtt message error {}", e),
                Ok(m) => log::info!("mqtt event: {:?}", m.payload()),
            }
        }
        log::warn!("exiting mqtt loop");
    });

    //subscribe to something
    mqtt_client.subscribe("EspNow/gw/radar", QoS::AtLeastOnce);

    //publish something
    let publication_result =
        mqtt_client.publish("hellotest", QoS::AtLeastOnce, false, "hello".as_bytes());
    match publication_result {
        Ok(r) => log::info!("publication ok {r}"),
        Err(x) => log::error!("publication failed {:?}", x),
    }

    loop {
        sleep(Duration::from_secs(1));
    }
}
