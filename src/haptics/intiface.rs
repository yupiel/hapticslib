use std::{sync::mpsc::{channel, Receiver}, thread};

use buttplug::{client::{ButtplugClient, ButtplugClientError}, core::{connector::new_json_ws_client_connector, errors::ButtplugError}};
use log::{error, info};

use crate::haptics::{channel::HAPTICS_DEVICES, handlers::{self, HandlerContext}};

use super::channel::{send_haptics_message, set_or_update_haptics_sender, HapticsMessage};

pub fn haptics_create_connection(websocket_address: String) -> String {
    if send_haptics_message(HapticsMessage::Ping).is_ok() {
        return "Thread is still alive and well. Ignoring new connection request.".into();
    }

    let (tx, rx) = channel::<HapticsMessage>();
    if let Err(set_or_update_error) = set_or_update_haptics_sender(tx) {
        return set_or_update_error;
    }

    haptics_spawn_thread(websocket_address, rx);

    "Connection established successfully.".into()
}

pub fn haptics_scan_start() -> String {
    match send_haptics_message(HapticsMessage::ScanStart) {
        Ok(_) => "Scanning for connected devices started...".into(),
        Err(err) => err
    }
}

pub fn haptics_scan_stop() -> String {
    match send_haptics_message(HapticsMessage::ScanStop) {
        Ok(_) => "Scanning for connected devices stopped.".into(),
        Err(err) => err
    }
}

pub fn haptics_ping() -> String {
    match send_haptics_message(HapticsMessage::Ping) {
        Ok(_) => "Heister's Haptics thread is alive and well.".into(),
        Err(err) => err
    }
}

pub fn haptics_stop_all() -> String {
    match send_haptics_message(HapticsMessage::StopAll) {
        Ok(_) => "Stopped all connected devices.".into(),
        Err(err) => err
    }
}

pub fn haptics_vibrate(strength: i32) -> String {
    match send_haptics_message(HapticsMessage::Vibrate(strength.clamp(0, 100) as f64 / 100_f64)) {
        Ok(_) => format!("Set haptics strength to: {}%", strength),
        Err(err) => err
    }
}

pub fn haptics_kill() -> String {
    match send_haptics_message(HapticsMessage::Kill) {
        Ok(_) => "Killing Haptics thread...".into(),
        Err(err) => err
    }
}

pub fn haptics_spawn_thread(websocket_address: String, mpsc_receiver: Receiver<HapticsMessage>) {
    thread::spawn(move || {
        let tokio_reactor = tokio::runtime::Runtime::new().unwrap();

        tokio_reactor.block_on(async move {
            let connector = new_json_ws_client_connector(&format!("ws://{}", websocket_address));
            let client = ButtplugClient::new("PD2 Heister's Haptics");
            
            info!("Attempting connection to intiface...");

            if let Err(e) = client.connect(connector).await {
                match e {
                    ButtplugClientError::ButtplugConnectorError(error) => {
                        error!("Can't connect to Intiface server. Dropping connection. Message: {}", error);
                        return;
                    }
                    ButtplugClientError::ButtplugError(error) => match error {
                        ButtplugError::ButtplugHandshakeError(error) => {
                            error!("Handshake issue with Intiface server. Dropping connection. Message: {}", error);
                            return;
                        }
                        error => {
                            error!("Unexpected error when trying to connect to Intiface! Message: {}", error);
                            return;
                        }
                    },
                }
            }

            info!("Connected to intiface! Scanning for devices...");

            //TODO: Do we want to do this?
            client.start_scanning().await.unwrap();
            client.stop_scanning().await.unwrap();

            //Load initial list of devices
            let mut device_list = HAPTICS_DEVICES.lock().unwrap();
            for device in client.devices() {
                device_list.push(device.name().into());
            }
            drop(device_list);

            let main_loop_context = HandlerContext {
                haptics_client: client,
                haptics_receiver: mpsc_receiver
            };

            'main: loop {
                match main_loop_context.haptics_receiver.recv() {
                    Ok(haptics_message) => {
                        if !main_loop_context.haptics_client.connected() {
                            error!("Client lost connection to intiface WebSocket. Closing Thread.");
                            break 'main;
                        }

                        match haptics_message {
                            HapticsMessage::Kill => {
                                handlers::kill(&main_loop_context).await;
                                break 'main;
                            },
                            HapticsMessage::Ping => {
                                // This is just used to check if the main loop is still alive
                                continue 'main;
                            },
                            HapticsMessage::ScanStart => {
                                handlers::scan::start(&main_loop_context).await;
                                continue 'main;
                            },
                            HapticsMessage::ScanStop => { 
                                info!("Cannot stop scanning because no scan is running.");
                                continue 'main;
                            },
                            HapticsMessage::StopAll => {
                                handlers::stop_all(&main_loop_context).await;
                                continue 'main;
                            },
                            HapticsMessage::Vibrate(strength) => {
                                handlers::vibrate(&main_loop_context, strength).await;
                                continue 'main;
                            },
                        }
                    },
                    Err(_) => {
                        error!("Sender died... this shouldn't happen.");
                        break 'main;
                    }
                }
            }
        });
    });
}
