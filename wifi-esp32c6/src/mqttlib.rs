use embedded_svc::mqtt::client::EventPayload;
use esp_idf_hal::sys::esp_mqtt_client_stop;
use esp_idf_svc::{
    handle::RawHandle,
    mqtt::client::{EspMqttClient, MqttClientConfiguration},
};
use std::thread::{self};
/// Connects to mqtt server.
///
/// * It requires being already connected to a network (ie: wife).
/// * Then it waits (blocking) until the connection is performed.
/// * A new thread (in eventloop) is created to attend the connection in the background
/// * An event_handler to process the incoming messages. For example `LoggerEventHandler` whill log
/// the payload and topic, but you should create your own
///
/// # example
///
/// ```  
///    let event_handler = LoggerEventHandler {};
///    let mut client = connect_mqtt("mqtt://192.168.0.100", "my_client_name", event_handler);
///    client.publish ("my_topic", QoS::AtLeastOnce, false, "hello world".as_bytes()).unwrap();
/// ```
///
/// #more
///
/// thread is created and last until the internal thread of the mqtt server dies.
/// when this treads dies, then `next()` also dies and everything is ok
/// to disconnect use the function `disconnect_mqtt`
pub fn connect_mqtt<'a, EH>(
    server: &'a str,
    client_name: &'a str,
    mut message_handler: EH,
) -> EspMqttClient<'a>
where
    EH: EventHandler + Send + 'static,
{
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
                    log::trace!("mqtt event: {:?}", m.payload());
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
                            message_handler.handle(topic, data);
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

/// disconnects from mqtt gracefully
///
/// * last will message will not appear
/// * the trhread created in connect_mqtt is destroyed
/// * the client varible is taked and disposed
/// * no more events will be received.
pub fn disconnect_mqtt(client: EspMqttClient) {
    let client = RawHandle::handle(&client);
    unsafe {
        esp_mqtt_client_stop(client);
    }
}

/// handler of mqtt message received events
pub trait EventHandler {
    fn handle(&mut self, topic: Option<&str>, payload: &[u8]) -> ();
}

/// basic implementation of EventHandler
/// transforms the payload to string and prints it on the log
pub struct LoggerEventHandler;
impl EventHandler for LoggerEventHandler {
    fn handle(&mut self, topic: Option<&str>, payload: &[u8]) -> () {
        let msg = String::from_utf8_lossy(payload);
        log::info!("got message in topic {:?} with payload {:?}", topic, msg);
    }
}
