use std::{sync::mpsc::{Receiver, SendError}, thread};

use buttplug::{client::{ButtplugClient, ButtplugClientError}, core::{connector::new_json_ws_client_connector, errors::ButtplugError}};
use log::{error, info};

use crate::haptics::handlers::{self, HandlerContext};

use super::channel::{set_or_update_haptics_sender, HapticsMessage, HAPTICS_SENDER};

pub fn haptics_create_connection(websocket_address: String) -> Result<String, SendError<HapticsMessage>> {
    match HAPTICS_SENDER.get().unwrap().read().unwrap().send(HapticsMessage::Ping) {
        Ok(_) => return Ok("Thread is still alive and well. Ignoring new connection request.".into()),
        Err(_) => {
            // Try to force kill the thread if just the sending of the message failed for some inexplicable reason
            HAPTICS_SENDER.get().unwrap().read().unwrap().send(HapticsMessage::Kill).unwrap_or(());
        }
    }

    let (tx, rx) = std::sync::mpsc::channel::<HapticsMessage>();
    set_or_update_haptics_sender(tx);

    haptics_spawn_thread(websocket_address, rx);

    Ok("Connection established successfully.".into())
}

pub fn haptics_scan_start() -> Result<String, SendError<HapticsMessage>> {
    HAPTICS_SENDER.get().unwrap().read().unwrap().send(HapticsMessage::ScanStart)
        .map(|_| "Scanning for connected devices started...".into())
}

pub fn haptics_scan_stop() -> Result<String, SendError<HapticsMessage>> {
    HAPTICS_SENDER.get().unwrap().read().unwrap().send(HapticsMessage::ScanStop)
        .map(|_| "Scanning for connected devices stopped.".into())
}

pub fn haptics_ping() -> Result<String, SendError<HapticsMessage>> {
    HAPTICS_SENDER.get().unwrap().read().unwrap().send(HapticsMessage::Ping)
        .map(|_| "Heister's Haptics thread is alive and well.".into())
}

pub fn haptics_vibrate(strength: i32) -> Result<String, SendError<HapticsMessage>> {
    HAPTICS_SENDER.get().unwrap().read().unwrap().send(HapticsMessage::Vibrate(
        strength.clamp(0, 100) as f64 / 100_f64)
    ).map(|_| format!("Set haptics strength to: {}%", strength))
}

pub fn haptics_stop_all() -> Result<String, SendError<HapticsMessage>> {
    HAPTICS_SENDER.get().unwrap().read().unwrap().send(HapticsMessage::StopAll)
        .map(|_| "Stopped all connected devices.".into())
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

            client.start_scanning().await.unwrap();
            client.stop_scanning().await.unwrap();

            info!("Finished scanning. Listing devices.");
            for device in client.devices().iter() {
                info!("Device {} [{}] found.", device.display_name().clone().unwrap_or("Unknown".into()), device.name());
            }

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
                            HapticsMessage::Ping => {/*  This is just used to check if the main loop is still alive */},
                            HapticsMessage::ScanStart => {
                                handlers::scan::start(&main_loop_context).await;
                             },
                            HapticsMessage::ScanStop => { 
                                info!("Cannot stop scanning because no scan is running.")
                             },
                            HapticsMessage::StopAll => {
                                handlers::stop_all(&main_loop_context).await;
                            },
                            HapticsMessage::Vibrate(strength) => {
                                handlers::vibrate(&main_loop_context, strength).await;
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
