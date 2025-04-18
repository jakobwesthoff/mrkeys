#![allow(improper_ctypes_definitions)]
#![allow(static_mut_refs)]
// ===========================================================================
// Most of this keygrabber code was borrowed from rdev, licensed under MIT
// License
// https://github.com/Narsil/rdev
// ===========================================================================

mod event;
mod external_type;

use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use core_graphics::event::{
    CGEvent, CGEventFlags, CGEventTapLocation, CGEventTapProxy, CGEventType,
};
use core_graphics::event::{CGKeyCode, EventField};
use event::{Event, EventType, Key};
use external_type::*;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::convert::TryInto;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tokio::sync::broadcast;

#[link(name = "Cocoa", kind = "framework")]
unsafe extern "C" {}

struct State {
    sender: Option<broadcast::Sender<Event>>,
    last_flags: CGEventFlags,
    pressed_keys: HashSet<Key>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sender: None,
            last_flags: CGEventFlags::empty(),
            pressed_keys: Default::default(),
        }
    }
}

lazy_static! {
    static ref STATE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::default()));
}

/// Errors that occur when trying to capture OS events.
/// Be careful on Mac, not setting accessibility does not cause an error
/// it justs ignores events.
#[derive(Debug)]
#[non_exhaustive]
pub enum ListenError {
    /// MacOS
    EventTap,
    /// MacOS
    LoopSource,
}

unsafe extern "C" fn raw_callback(
    _proxy: CGEventTapProxy,
    _type: CGEventType,
    cg_event: CGEvent,
    _user_info: *mut c_void,
) -> CGEvent {
    let mut state = STATE.lock().unwrap();
    if let Some(event) = process_cg_event(_type, &cg_event, &mut state) {
        if let Some(ref mut sender) = state.sender {
            sender.send(event).unwrap();
        }
    }
    cg_event
}

pub fn listen() -> Result<(), ListenError> {
    // Unsafe: this is okay to do, as we are checking the response values to ensure we got a valid
    // tap
    let tap = unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let tap = CGEventTapCreate(
            CGEventTapLocation::HID, // HID, Session, AnnotatedSession,
            kCGHeadInsertEventTap,
            CGEventTapOption::ListenOnly,
            kCGEventMaskForAllEvents,
            raw_callback,
            nil,
        );
        if tap.is_null() {
            return Err(ListenError::EventTap);
        }

        Ok(tap)
    }?;

    // Unsafe: Is okay to do because we are validating the return to be okay.
    let _loop = unsafe {
        let _loop = CFMachPortCreateRunLoopSource(nil, tap, 0);
        if _loop.is_null() {
            return Err(ListenError::LoopSource);
        }

        Ok(_loop)
    }?;

    // FIXME: Unsafe: Check if CFRunLoopAddSource can take empty current_loop. Otherwise a check is
    // missing here.
    let current_loop = unsafe { CFRunLoopGetCurrent() };

    // FIXME: Check API if there are any return values to ensure the calls were sucessful.
    unsafe {
        CFRunLoopAddSource(current_loop, _loop, kCFRunLoopCommonModes);
        CGEventTapEnable(tap, true);
        // Will block running the loop
        CFRunLoopRun();
    }

    Ok(())
}

fn process_cg_event(_type: CGEventType, cg_event: &CGEvent, state: &mut State) -> Option<Event> {
    let option_type = match _type {
        CGEventType::KeyDown => {
            let code = cg_event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE);
            let keycode: CGKeyCode = code.try_into().ok()?;
            let key = Key::from(keycode);
            if state.pressed_keys.contains(&key) {
                None
            } else {
                state.pressed_keys.insert(key);
                Some(EventType::KeyPress(key))
            }
        }
        CGEventType::KeyUp => {
            let code = cg_event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE);
            let keycode: CGKeyCode = code.try_into().ok()?;
            let key = Key::from(keycode);
            state.pressed_keys.remove(&key);
            Some(EventType::KeyRelease(key))
        }
        CGEventType::FlagsChanged => {
            // This handles keys like shift, meta, ctrl, command, alt and so on...
            let code = cg_event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE);
            let code: CGKeyCode = code.try_into().ok()?;
            let flags = cg_event.get_flags();

            if flags < state.last_flags {
                state.last_flags = flags;
                Some(EventType::KeyRelease(Key::from(code)))
            } else {
                state.last_flags = flags;
                Some(EventType::KeyPress(Key::from(code)))
            }
        }
        _ => None,
    };
    if let Some(event_type) = option_type {
        // let name = match event_type {
        //     EventType::KeyPress(_) => {
        //         let code =
        //             cg_event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) as u32;
        //         let flags = cg_event.get_flags();
        //         keyboard_state.create_string_for_key(code, flags)
        //         None
        //     }
        //     _ => None,
        // };
        return Some(Event {
            event_type,
            time: SystemTime::now(),
            name: None,
        });
    }
    None
}

pub fn get_channel() -> broadcast::Receiver<Event> {
    let mut state = STATE.lock().unwrap();

    match state.sender {
        Some(ref sender) => sender.subscribe(),
        None => {
            let (sender, receiver) = broadcast::channel::<Event>(32);
            state.sender.replace(sender);

            // FIXME: Handle possible errors more gracefully
            std::thread::spawn(move || {
                listen().unwrap();
            });

            receiver
        }
    }
}
