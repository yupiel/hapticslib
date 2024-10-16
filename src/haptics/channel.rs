use std::sync::{
    atomic::AtomicBool, mpsc::Sender, LazyLock, Mutex, OnceLock, RwLock, RwLockReadGuard,
};

pub static HAPTICS_SENDER: OnceLock<RwLock<Sender<HapticsMessage>>> = OnceLock::new();
pub static HAPTICS_IS_SCANNING: AtomicBool = AtomicBool::new(false);
pub static HAPTICS_DEVICES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Clone, Debug)]
pub enum HapticsMessage {
    Kill,
    Ping,
    ScanStart,
    ScanStop,
    StopAll,
    Vibrate(f64),
}

pub fn get_haptics_sender<'hs>() -> Result<RwLockReadGuard<'hs, Sender<HapticsMessage>>, String> {
    match HAPTICS_SENDER.get() {
        Some(sender_rwlock) => match sender_rwlock.read() {
            Ok(inner_sender) => Ok(inner_sender),
            Err(_) => Err("Haptics Sender has been poisoned. Try reconnecting.".into()),
        },
        None => {
            Err("Haptics connection was not previously established. Ignoring kill request".into())
        }
    }
}

pub fn send_haptics_message(message: HapticsMessage) -> Result<(), String> {
    match get_haptics_sender()?.send(message.clone()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "Haptics thread could not receive {:?} message. Connection died. Please re-establish.",
            message
        )),
    }
}

pub fn set_or_update_haptics_sender(
    new_haptics_sender: Sender<HapticsMessage>,
) -> Result<(), String> {
    match HAPTICS_SENDER.get() {
        Some(inner_sender) => {
            let mut sender_writer = match inner_sender.write() {
                Ok(inner_writer) => inner_writer,
                Err(poison_error) => {
                    return Err(format!(
                        "Failed updating Haptics Communicator. Message: {}",
                        poison_error
                    ))
                }
            };
            *sender_writer = new_haptics_sender;

            drop(sender_writer);
            Ok(())
        }
        None => match HAPTICS_SENDER.set(RwLock::new(new_haptics_sender)) {
            Ok(_) => Ok(()),
            Err(_) => {
                Err("Tried overwriting existing Haptics Sender. This shouldn't happen.".into())
            }
        },
    }
}
