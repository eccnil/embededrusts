use std::{
    thread::{self, sleep},
    time::Duration,
};

use embedded_svc::{
    mqtt::client::{EventPayload, QoS},
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};
use esp_idf_hal::{modem::Modem, peripherals::Peripherals, sys::EspError};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    mqtt::client::{EspMqttClient, MqttClientConfiguration},
    nvs::EspDefaultNvsPartition,
    wifi::EspWifi,
};
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
    let mut client = connect_mqtt(setup.mqtt_server, "esptest");

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

    //subscribe to something
    let _ = client.subscribe("EspNow/gw/radar", QoS::AtLeastOnce);

    loop {
        sleep(Duration::from_millis(100));
    }
}

/// Connects to mqtt server.
///
/// * It requires being already connected to a network (ie: wife).
/// * Then it waits (blocking) until the connection is performed.
/// * A new thread (in eventloop) is created to attend the connection in the background
///
/// # example
///
/// ```  
///    let mut client = connect_mqtt("mqtt://192.168.0.100", "my_client_name");
///    client.publish ("my_topic", QoS::AtLeastOnce, false, "hello world".as_bytes()).unwrap();
/// ```
//TODO: como parametro una lista de topics
//TODO: aceptar una funcion como parametro que atienda las subscripciones
fn connect_mqtt<'a>(server: &'a str, client_name: &'a str) -> EspMqttClient<'a> {
    //mqtt client creation
    let mqtt_config = MqttClientConfiguration {
        client_id: client_name.into(),
        ..Default::default()
    };
    let (mqtt_client, mut mqtt_connection) = EspMqttClient::new(server, &mqtt_config).unwrap();

    thread::spawn(move || {
        log::info!("entering mqtt loop");
        loop {
            let msg = mqtt_connection.next();
            match msg {
                Err(e) => log::error!("mqtt message error {}", e),
                Ok(m) => {
                    log::info!("mqtt event: {:?}", m.payload());
                    match m.payload() {
                        EventPayload::Connected(_) => {
                            log::info!("connection status updated to connected");
                        }
                        EventPayload::Received {
                            id: _,
                            topic,
                            data,
                            details: _,
                        } => {
                            log::info!(
                                "message received to topic {:?} with content {:?}",
                                topic,
                                String::from_utf8_lossy(data)
                            );
                        }
                        _ => (),
                    }
                }
            }
        }
    });

    //client needs to be returned to avoid it being disposed
    mqtt_client
}

/// connectes to wifi
///
/// * connectes to a AP (Access Point)
/// * connectes syncronous, waiting untill get an ip from the ap
///
/// # Panincs
/// - configuring the phisical modem
/// - connecting to the AP -> check credentials
/// - getting an the interface up (getting an ip)
///
/// # Example
/// ```
/// let peripherals = Peripherals::take().unwrap();
/// let sysloop = EspSystemEventLoop::take().unwrap();
/// let nvs = EspDefaultNvsPartition::take().unwrap();
/// let _ = wifi_sync_connect(peripherals.modem,sysloop, nvs, "myssid", "mypasswd").unwrap();
/// ```
fn wifi_sync_connect<'a>(
    modem: Modem,
    sysloop: esp_idf_svc::eventloop::EspEventLoop<esp_idf_svc::eventloop::System>,
    nvs: esp_idf_svc::nvs::EspNvsPartition<esp_idf_svc::nvs::NvsDefault>,
    ssid: &'a str,
    password: &'a str,
) -> Result<EspWifi<'a>, EspError> {
    //create wifi driver
    let mut wifi = EspWifi::new(modem, sysloop, Some(nvs)).unwrap();
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),
        password: password.try_into().unwrap(),
        auth_method: AuthMethod::None,
        ..Default::default()
    }))?;

    //start wifi
    log::info!("wifi starting");
    wifi.start()?;
    log::info!("wifi conecting");
    wifi.connect()?;

    //confirm connection
    log::info!("wifi waiting to connect");
    while !wifi.is_connected()? {
        sleep(Duration::from_millis(100));
    }

    //wait to get ip
    log::info!("conectado");
    while !wifi.sta_netif().is_up()? {
        sleep(Duration::from_millis(100));
    }
    let ip = wifi.sta_netif().get_ip_info()?;
    log::info!("{:?}", ip);
    Ok(wifi)
}
