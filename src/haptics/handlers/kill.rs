use log::{debug, error, info};

use super::HandlerContext;

pub async fn kill(ctx: &HandlerContext) {
    debug!("HapticsMesage::Kill Received");

    ctx.haptics_client.stop_scanning().await.unwrap_or(());
    ctx.haptics_client.stop_all_devices().await.unwrap_or(());
    ctx.haptics_client
        .disconnect()
        .await
        .inspect_err(|err| {
            error!(
                "Disconnecting Heister's Haptics Client failed. Message: {}",
                err
            );
        })
        .unwrap_or(());

    info!("Kill Signal received. Closing Thread.")
}
