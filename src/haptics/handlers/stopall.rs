use log::{debug, error};

use super::HandlerContext;

pub async fn stop_all(ctx: &HandlerContext) {
    debug!("HapticsMesage::StopAll Received");

    ctx.haptics_client
        .stop_all_devices()
        .await
        .inspect_err(|err| {
            error!("Failed to stop all device vibration. Message: {}", err);
        })
        .unwrap_or(());
}
