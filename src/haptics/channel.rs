use std::sync::{mpsc::Sender, OnceLock, RwLock};

pub static HAPTICS_SENDER: OnceLock<RwLock<Sender<HapticsMessage>>> = OnceLock::new();

pub enum HapticsMessage {
    Kill,
    Ping,
    Scan,
    Strength(f64),
    StopAll,
}

pub fn set_or_update_haptics_sender(new_haptics_sender: Sender<HapticsMessage>) {
    match HAPTICS_SENDER.get() {
        Some(inner_sender) => {
            let mut sender_writer = inner_sender.write().unwrap();
            *sender_writer = new_haptics_sender;

            drop(sender_writer);
        }
        None => HAPTICS_SENDER.set(RwLock::new(new_haptics_sender)).unwrap(),
    }
}
