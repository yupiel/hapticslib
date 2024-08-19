use std::sync::atomic::Ordering;

use log::{debug, error, info, warn};

use crate::haptics::channel::{HapticsMessage, HAPTICS_DEVICES, HAPTICS_IS_SCANNING};

use super::HandlerContext;

pub async fn start(ctx: &HandlerContext) {
    debug!("HapticsMesage::ScanStart Received");

    info!("Starting scanning for devices...");
    HAPTICS_IS_SCANNING.store(true, Ordering::SeqCst);

    ctx.haptics_client.start_scanning().await.unwrap();

    'scan: loop {
        match ctx.haptics_receiver.recv() {
            Ok(haptics_message) => match haptics_message {
                HapticsMessage::Kill => {
                    super::kill(ctx).await;
                    // TODO: How do i do this?
                    // Send message via sender and break out of scan loop, so the next message handled is kill (unsafe could be interrupted by other stuff)
                    // Return something from this function and handle accordingly in main loop?
                    // Delete the Sender 💀
                }
                HapticsMessage::Ping => {
                    info!("Haptics is still in scanning mode...");
                }
                HapticsMessage::ScanStart => {
                    warn!("Scan is already in progress");
                }
                HapticsMessage::ScanStop => {
                    stop(ctx).await;
                    break 'scan;
                }
                HapticsMessage::StopAll => {
                    stop(ctx).await;
                    super::stop_all(ctx).await;

                    break 'scan;
                }
                HapticsMessage::Vibrate(_) => {
                    warn!("Cannot set strength when scanning for devices.");
                }
            },
            Err(_) => error!("Sender died... this shouldn't happen."),
        }
    }
}

pub async fn stop(ctx: &HandlerContext) {
    debug!("HapticsMesage::ScanStop Received");

    info!("Stopped scanning for devices. Returning List.");
    ctx.haptics_client.stop_scanning().await.unwrap();

    HAPTICS_IS_SCANNING.store(false, Ordering::SeqCst);

    match HAPTICS_DEVICES.lock() {
        Ok(mut device_list) => {
            device_list.clear();

            for connected_device in ctx.haptics_client.devices() {
                device_list.push(connected_device.name().into());
            }

            drop(device_list);
        }
        Err(err) => error!(
            "Device list Mutex was poisoned. This shouldn't happen. Message: {}",
            err
        ),
    }
}
