use cocoa::base::id;
use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventTapProxy, CGEventType, CGKeyCode};
use std::ffi::c_void;

pub type CFMachPortRef = *const c_void;
pub type CFIndex = u64;
pub type CFAllocatorRef = id;
pub type CFRunLoopSourceRef = id;
pub type CFRunLoopRef = id;
pub type CFRunLoopMode = id;

// https://developer.apple.com/documentation/coregraphics/cgeventtapplacement?language=objc
pub type CGEventTapPlacement = u32;
#[allow(non_upper_case_globals)]
pub const kCGHeadInsertEventTap: u32 = 0;

// https://developer.apple.com/documentation/coregraphics/cgeventtapoptions?language=objc
#[allow(non_upper_case_globals)]
#[repr(u32)]
pub enum CGEventTapOption {
    ListenOnly = 1,
}

// https://developer.apple.com/documentation/coregraphics/cgeventmask?language=objc
pub type CGEventMask = u64;

// FIXME: Not the right name nor the correct definition anymore
#[allow(non_upper_case_globals)]
pub const kCGEventMaskForAllEvents: u64 = (1 << CGEventType::KeyDown as u64)
    + (1 << CGEventType::KeyUp as u64)
    + (1 << CGEventType::FlagsChanged as u64);

#[cfg(target_os = "macos")]
#[link(name = "Cocoa", kind = "framework")]
unsafe extern "C" {
    #[allow(improper_ctypes)]
    pub fn CGEventTapCreate(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOption,
        eventsOfInterest: CGEventMask,
        callback: QCallback,
        user_info: id,
    ) -> CFMachPortRef;
    pub fn CFMachPortCreateRunLoopSource(
        allocator: CFAllocatorRef,
        tap: CFMachPortRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFRunLoopMode);
    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    pub fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    pub fn CFRunLoopRun();

    pub static kCFRunLoopCommonModes: CFRunLoopMode;

}
pub type QCallback = unsafe extern "C" fn(
    proxy: CGEventTapProxy,
    _type: CGEventType,
    cg_event: CGEvent,
    user_info: *mut c_void,
) -> CGEvent;

/// Option
pub const ALT: CGKeyCode = 58;
/// Option_Right
pub const ALT_GR: CGKeyCode = 61;
pub const BACKSPACE: CGKeyCode = 51;
pub const CAPS_LOCK: CGKeyCode = 57;
pub const CONTROL_LEFT: CGKeyCode = 59;
pub const CONTROL_RIGHT: CGKeyCode = 62;
pub const DOWN_ARROW: CGKeyCode = 125;
pub const ESCAPE: CGKeyCode = 53;
pub const F1: CGKeyCode = 122;
pub const F10: CGKeyCode = 109;
pub const F11: CGKeyCode = 103;
pub const F12: CGKeyCode = 111;
pub const F2: CGKeyCode = 120;
pub const F3: CGKeyCode = 99;
pub const F4: CGKeyCode = 118;
pub const F5: CGKeyCode = 96;
pub const F6: CGKeyCode = 97;
pub const F7: CGKeyCode = 98;
pub const F8: CGKeyCode = 100;
pub const F9: CGKeyCode = 101;
pub const FUNCTION: CGKeyCode = 63;
pub const LEFT_ARROW: CGKeyCode = 123;
pub const META_LEFT: CGKeyCode = 55;
pub const META_RIGHT: CGKeyCode = 54;
pub const RETURN: CGKeyCode = 36;
pub const RIGHT_ARROW: CGKeyCode = 124;
pub const SHIFT_LEFT: CGKeyCode = 56;
pub const SHIFT_RIGHT: CGKeyCode = 60;
pub const SPACE: CGKeyCode = 49;
pub const TAB: CGKeyCode = 48;
pub const UP_ARROW: CGKeyCode = 126;
pub const BACK_QUOTE: CGKeyCode = 50;
pub const NUM1: CGKeyCode = 18;
pub const NUM2: CGKeyCode = 19;
pub const NUM3: CGKeyCode = 20;
pub const NUM4: CGKeyCode = 21;
pub const NUM5: CGKeyCode = 23;
pub const NUM6: CGKeyCode = 22;
pub const NUM7: CGKeyCode = 26;
pub const NUM8: CGKeyCode = 28;
pub const NUM9: CGKeyCode = 25;
pub const NUM0: CGKeyCode = 29;
pub const MINUS: CGKeyCode = 27;
pub const EQUAL: CGKeyCode = 24;
pub const KEY_Q: CGKeyCode = 12;
pub const KEY_W: CGKeyCode = 13;
pub const KEY_E: CGKeyCode = 14;
pub const KEY_R: CGKeyCode = 15;
pub const KEY_T: CGKeyCode = 17;
pub const KEY_Y: CGKeyCode = 16;
pub const KEY_U: CGKeyCode = 32;
pub const KEY_I: CGKeyCode = 34;
pub const KEY_O: CGKeyCode = 31;
pub const KEY_P: CGKeyCode = 35;
pub const LEFT_BRACKET: CGKeyCode = 33;
pub const RIGHT_BRACKET: CGKeyCode = 30;
pub const KEY_A: CGKeyCode = 0;
pub const KEY_S: CGKeyCode = 1;
pub const KEY_D: CGKeyCode = 2;
pub const KEY_F: CGKeyCode = 3;
pub const KEY_G: CGKeyCode = 5;
pub const KEY_H: CGKeyCode = 4;
pub const KEY_J: CGKeyCode = 38;
pub const KEY_K: CGKeyCode = 40;
pub const KEY_L: CGKeyCode = 37;
pub const SEMI_COLON: CGKeyCode = 41;
pub const QUOTE: CGKeyCode = 39;
pub const BACK_SLASH: CGKeyCode = 42;
pub const KEY_Z: CGKeyCode = 6;
pub const KEY_X: CGKeyCode = 7;
pub const KEY_C: CGKeyCode = 8;
pub const KEY_V: CGKeyCode = 9;
pub const KEY_B: CGKeyCode = 11;
pub const KEY_N: CGKeyCode = 45;
pub const KEY_M: CGKeyCode = 46;
pub const COMMA: CGKeyCode = 43;
pub const DOT: CGKeyCode = 47;
pub const SLASH: CGKeyCode = 44;
