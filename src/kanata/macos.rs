use anyhow::Result;
use crossbeam_channel::Sender;
use log::{info, debug};
use parking_lot::Mutex;
use rdev::{grab, Event};
use std::sync::Arc;

use super::*;

static PRESSED_KEYS: Lazy<Mutex<HashSet<OsCode>>> = Lazy::new(|| Mutex::new(HashSet::new()));

impl Kanata {
    /// Enter an infinite loop that listens for OS key events and sends them to the processing
    /// thread.
    pub fn event_loop(kanata: Arc<Mutex<Self>>, tx: Sender<KeyEvent>) -> Result<()> {
        info!("entering the event loop");
        {
            let mut mapped_keys = MAPPED_KEYS.lock();
            *mapped_keys = kanata.lock().mapped_keys.clone();
        }

        let callback = move |event: Event| -> Option<Event> {
            match KeyEvent::try_from(event.clone()) {
                Ok(mut key_event) => {
                    check_for_exit(&key_event);

                    // unwrap is safe because the KeyEvent conversion above would've returned false otherwise

                    if !MAPPED_KEYS.lock().contains(&key_event.code) {
                        Some(event)
                    } else {
                        // Unlike Linux, macOS does not use a separate value for repeat. However, our code
                        // needs to differentiate between initial press and repeat press.
                        log::debug!("event loop: {:?}", key_event);
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

                        tx.send(key_event).unwrap();
                        None
                    }
                }
                Err(_) => {
                    Some(event)
                },
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
