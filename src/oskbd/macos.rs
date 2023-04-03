//! Contains the input/output code for keyboards on macOS.

use core::ffi::c_void;
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use rdev::{simulate, EventType};
use serde::de::IntoDeserializer;
use std::io;

use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventTapProxy};

use crate::custom_action::*;
use crate::keys::*;

/// Handle for writing keys to the OS.
pub struct KbdOut {}

impl KbdOut {
    pub fn new() -> Result<Self, io::Error> {
        Ok(Self {})
    }

    pub fn write(&mut self, event_type: EventType) -> Result<(), io::Error> {
        log::info!("input ev: {:?}", event_type);
        use std::{thread, time};
        let event_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
            .ok()
            .unwrap();

        thread::sleep(time::Duration::from_millis(20));

        unsafe {
            let event = convert_native_with_source(&event_type, event_source)
                .expect("Failed creating event");
            // event.post(CGEventTapLocation::HID );

            let state_ptr: *const c_void = std::mem::transmute(&self);

            event.post_from_tap(state_ptr);

            Ok(())
        }
    }

    pub fn write_key(&mut self, key: OsCode, value: KeyValue) -> Result<(), io::Error> {
        let event = event_type_from_oscode(key, value);
        self.write(event)
    }

    pub fn press_key(&mut self, key: OsCode) -> Result<(), io::Error> {
        self.write_key(key, KeyValue::Press)
    }

    pub fn release_key(&mut self, key: OsCode) -> Result<(), io::Error> {
        self.write_key(key, KeyValue::Release)
    }

    pub fn send_unicode(&mut self, _c: char) -> Result<(), io::Error> {
        Ok(())
    }

    pub fn click_btn(&mut self, btn: Btn) -> Result<(), io::Error> {
        log::debug!("click btn: {:?}", btn);

        Ok(())
    }

    pub fn release_btn(&mut self, btn: Btn) -> Result<(), io::Error> {
        log::debug!("release btn: {:?}", btn);

        Ok(())
    }

    pub fn scroll(&mut self, direction: MWheelDirection, distance: u16) -> Result<(), io::Error> {
        log::debug!("scroll: {direction:?} {distance:?}");
        match direction {
            MWheelDirection::Up => self.write(EventType::Wheel {
                delta_x: distance.try_into().unwrap(),
                delta_y: 0,
            }),
            MWheelDirection::Right => self.write(EventType::Wheel {
                delta_x: 0,
                delta_y: distance.try_into().unwrap(),
            }),
            MWheelDirection::Down => self.write(EventType::Wheel {
                delta_x: -(i64::try_from(distance).unwrap()),
                delta_y: 0,
            }),
            MWheelDirection::Left => self.write(EventType::Wheel {
                delta_x: 0,
                delta_y: -(i64::try_from(distance).unwrap()),
            }),
        }
    }
}

fn event_type_from_oscode(code: OsCode, value: KeyValue) -> EventType {
    match value {
        KeyValue::Release => EventType::KeyRelease(OsCode::as_key(code)),
        KeyValue::Press | KeyValue::Repeat => EventType::KeyPress(OsCode::as_key(code)),
    }
}

fn clear(event_type: EventType) -> EventType {
    match event_type {
        EventType::KeyRelease(code) => EventType::KeyRelease(code),
        EventType::KeyPress(code) => EventType::KeyRelease(code),
        _ => todo!(),
    }
}

unsafe fn convert_native_with_source(
    event_type: &EventType,
    source: CGEventSource,
) -> Option<CGEvent> {
    match event_type {
        EventType::KeyPress(key) => {
            let code = OsCode::from_key(*key)?.as_u16();
            CGEvent::new_keyboard_event(source, code, true).ok()
        }
        EventType::KeyRelease(key) => {
            let code = OsCode::from_key(*key)?.as_u16();
            CGEvent::new_keyboard_event(source, code, false).ok()
        }
        _ => None,
    }
}
