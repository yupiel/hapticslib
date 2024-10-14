use std::sync::mpsc::Receiver;

use buttplug::client::ButtplugClient;

use super::channel::HapticsMessage;

mod kill;
// Since I export both start and stop for here I namespace them
pub mod scan;
mod stopall;
mod vibrate;

pub use kill::kill;
pub use stopall::stop_all;
pub use vibrate::vibrate;

pub struct HandlerContext {
    pub haptics_client: ButtplugClient,
    pub haptics_receiver: Receiver<HapticsMessage>,
}
