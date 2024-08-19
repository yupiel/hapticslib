use log::{debug, error};

use super::HandlerContext;

pub async fn vibrate(ctx: &HandlerContext, strength: f64) {
    debug!("HapticsMesage::Vibrate Received");

    for device in ctx.haptics_client.devices() {
        device
            .vibrate(&buttplug::client::ScalarValueCommand::ScalarValue(strength))
            .await
            .inspect_err(|err| {
                error!(
                    "Failed to send vibration command to device. Message: {}",
                    err
                );
            })
            .unwrap_or(());
    }
}
