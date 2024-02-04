use std::{thread::sleep, time::Duration};

use embedded_svc::mqtt::client::QoS;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use wifi_esp32c6::{
    mqttlib::{connect_mqtt, disconnect_mqtt, LoggerEventHandler},
    setup::Setup,
    wifilib::wifi_sync_connect,
};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // geting setup (passwords and config)
    let setup: Setup = Setup::load();
    log::info!("config gets {}, mqtt {}", setup.ssid, setup.mqtt_server);
    let ssid = &setup.ssid;
    let password = &setup.password;

    //taking singletone things
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();
    let modem = peripherals.modem;

    //connect to wifi
    let _wifi = wifi_sync_connect(modem, sysloop, nvs, ssid, password).unwrap();

    //connect to mqtt
    let event_handler = LoggerEventHandler {};
    let mut client = connect_mqtt(setup.mqtt_server, "esptest", event_handler);

    // publish something
    let publication_result = client.publish(
        "my_topic",
        QoS::AtLeastOnce,
        false,
        "hello world".as_bytes(),
    );
    match publication_result {
        Ok(r) => log::info!("publication ok {r}"),
        Err(x) => log::error!("publication failed {:?}", x),
    }

    // subscribe to something
    let _ = client.subscribe("EspNow/gw/radar", QoS::AtLeastOnce);

    // attend during some thime incomming events
    sleep(Duration::from_secs(10));

    //disconnect
    disconnect_mqtt(client);

    //and halt
}
