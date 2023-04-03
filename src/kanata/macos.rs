use anyhow::Result;
use crossbeam_channel::Sender;
use log::{debug, info};
use parking_lot::Mutex;
use rdev::{grab, Event};
use std::{sync::Arc, thread};

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

        let (preprocess_tx, preprocess_rx) = crossbeam_channel::bounded(10);
        start_event_preprocessor(preprocess_rx, tx);

        let callback = move |event: Event| -> Option<Event> {
            match KeyEvent::try_from(event.clone()) {
                Ok(mut key_event) => {
                    check_for_exit(&key_event);

                    if !MAPPED_KEYS.lock().contains(&key_event.code) {
                        return Some(event);
                    }

                    // Unlike Linux, macOS does not use a separate value for repeat. However, our co
                    // needs to differentiate between initial press and repeat press.
                    log::debug!("event loop: {:?}", key_event);
                    // match key_event.value {
                    //     KeyValue::Release => {
                    //         PRESSED_KEYS.lock().remove(&key_event.code);
                    //     }
                    //     KeyValue::Press => {
                    //         let mut pressed_keys = PRESSED_KEYS.lock();
                    //         if pressed_keys.contains(&key_event.code) {
                    //             key_event.value = KeyValue::Repeat;
                    //         } else {
                    //             pressed_keys.insert(key_event.code);
                    //         }
                    //     }
                    //     _ => {}
                    // }

                    try_send_panic(&preprocess_tx, key_event);
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

fn try_send_panic(tx: &Sender<KeyEvent>, kev: KeyEvent) {
    if let Err(e) = tx.try_send(kev) {
        panic!("failed to send on channel: {:?}", e)
    }
}

fn start_event_preprocessor(preprocess_rx: Receiver<KeyEvent>, process_tx: Sender<KeyEvent>) {
    thread::spawn(move || loop {
        match preprocess_rx.try_recv() {
            Ok(kev) => try_send_panic(&process_tx, kev),
            Err(TryRecvError::Empty) => {
                thread::sleep(time::Duration::from_millis(1));
            }
            Err(TryRecvError::Disconnected) => {
                panic!("channel disconnected")
            }
        }
    });
}
