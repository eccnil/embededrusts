use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::{modem::Modem, sys::EspError};
use esp_idf_svc::wifi::EspWifi;
use std::{thread::sleep, time::Duration};

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
pub fn wifi_sync_connect<'a>(
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
