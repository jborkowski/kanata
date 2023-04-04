use anyhow::Result;
use log::{debug, info};
use parking_lot::Mutex;
use rdev::{grab, Event};
use std::sync::mpsc::Sender;
use std::sync::Arc;

use super::*;

static PRESSED_KEYS: Lazy<Mutex<HashSet<OsCode>>> = Lazy::new(|| Mutex::new(HashSet::default()));

impl Kanata {
    /// Enter an infinite loop that listens for OS key events and sends them to the processing
    /// thread.
    pub fn event_loop(kanata: Arc<Mutex<Self>>, tx: Sender<KeyEvent>) -> Result<()> {
        info!("entering the event loop");

        let k = kanata.lock();

        drop(k);

        let callback = move |event: Event| -> Option<Event> {
            match KeyEvent::try_from(event.clone()) {
                Ok(mut key_event) => {
                    check_for_exit(&key_event);

                    if !MAPPED_KEYS.lock().contains(&key_event.code) {
                        return Some(event);
                    }

                    // Unlike Linux, macOS does not use a separate value for repeat. However, our co
                    // needs to differentiate between initial press and repeat press.
                    debug!("event loop: {:?}", key_event);
                    match key_event.value {
                        KeyValue::Release => {
                            PRESSED_KEYS.lock().remove(&key_event.code);
                        }
                        KeyValue::Press => {
                            let mut pressed_keys = PRESSED_KEYS.lock();
                            if pressed_keys.contains(&key_event.code) {
                                key_event.value = KeyValue::Repeat;
                            } else {
                                pressed_keys.insert(key_event.code);
                            }
                        }
                        _ => {}
                    }

                    if let Err(e) = tx.send(key_event) {
                        panic!("failed to send on channel: {:?}", e)
                    }
                    None
                }
                Err(_) => Some(event),
            }
        };

        if let Err(error) = grab(callback) {
            panic!("Grabing envent error: {:?}", error)
        }

        Ok(())
    }

    pub fn check_release_non_physical_shift(&mut self) -> Result<()> {
        Ok(())
    }
}
