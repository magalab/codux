#![allow(unexpected_cfgs)]

#[cfg(target_os = "macos")]
use cocoa::{
    appkit::{NSApp, NSWindow, NSWindowButton},
    base::{NO, YES, id},
    foundation::NSRect,
};
use gpui::Window;
#[cfg(target_os = "macos")]
use objc::{
    msg_send,
    runtime::{
        Class, Imp, Object, Sel, class_getInstanceMethod, method_getImplementation,
        method_setImplementation,
    },
    sel, sel_impl,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
#[cfg(target_os = "macos")]
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
#[cfg(target_os = "macos")]
static REOPEN_HANDLER_INSTALLED: AtomicBool = AtomicBool::new(false);
#[cfg(target_os = "macos")]
static ORIGINAL_REOPEN_HANDLER: AtomicUsize = AtomicUsize::new(0);
#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
pub(crate) fn install_dock_reopen_handler() {
    if REOPEN_HANDLER_INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }

    unsafe {
        let app = NSApp();
        let delegate: id = msg_send![app, delegate];
        if delegate.is_null() {
            REOPEN_HANDLER_INSTALLED.store(false, Ordering::SeqCst);
            return;
        }

        let delegate_class: *const Class = msg_send![delegate, class];
        let method = class_getInstanceMethod(
            delegate_class,
            Sel::register("applicationShouldHandleReopen:hasVisibleWindows:"),
        );
        if method.is_null() {
            REOPEN_HANDLER_INSTALLED.store(false, Ordering::SeqCst);
            return;
        }

        let original = method_getImplementation(method);
        ORIGINAL_REOPEN_HANDLER.store(original as usize, Ordering::SeqCst);
        let replacement: Imp = std::mem::transmute(
            dock_should_handle_reopen as unsafe extern "C" fn(&mut Object, Sel, id, bool),
        );
        let _ = method_setImplementation(method.cast_mut(), replacement);
    }
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn install_dock_reopen_handler() {}

#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
pub(crate) fn configure_main_window_controls(window: &mut Window) {
    configure_native_window_buttons(window, false);
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn configure_main_window_controls(_window: &mut Window) {}

#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
pub(in crate::app) fn configure_child_window_controls(window: &mut Window) {
    configure_native_window_buttons(window, true);
}

#[cfg(not(target_os = "macos"))]
pub(in crate::app) fn configure_child_window_controls(_window: &mut Window) {}

#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
pub(in crate::app) fn configure_document_child_window_controls(window: &mut Window) {
    configure_native_window_buttons(window, false);
}

#[cfg(not(target_os = "macos"))]
pub(in crate::app) fn configure_document_child_window_controls(_window: &mut Window) {}

#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
fn configure_native_window_buttons(window: &mut Window, close_only: bool) {
    let Some(ns_window) = appkit_window(window) else {
        return;
    };

    unsafe {
        let close_button = ns_window.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
        let min_button = ns_window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
        let zoom_button = ns_window.standardWindowButton_(NSWindowButton::NSWindowZoomButton);

        set_button_hidden(min_button, close_only);
        set_button_hidden(zoom_button, close_only);
        if !close_only {
            offset_window_button(close_button);
            offset_window_button(min_button);
            offset_window_button(zoom_button);
        }
    }
}

#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
unsafe fn set_button_hidden(button: id, hidden: bool) {
    if button.is_null() {
        return;
    }
    unsafe {
        let _: () = msg_send![button, setHidden: if hidden { YES } else { NO }];
    }
}

#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
unsafe fn offset_window_button(button: id) {
    if button.is_null() {
        return;
    }
    unsafe {
        let mut frame: NSRect = msg_send![button, frame];
        frame.origin.y += 5.0;
        let _: () = msg_send![button, setFrame: frame];
    }
}

#[cfg(target_os = "macos")]
unsafe extern "C" fn dock_should_handle_reopen(
    this: &mut Object,
    cmd: Sel,
    application: id,
    _has_visible_windows: bool,
) {
    let original = ORIGINAL_REOPEN_HANDLER.load(Ordering::SeqCst);
    if original == 0 {
        return;
    }

    let original: unsafe extern "C" fn(&mut Object, Sel, id, bool) =
        unsafe { std::mem::transmute(original) };
    unsafe {
        original(this, cmd, application, false);
    }
}

#[cfg(target_os = "macos")]
fn appkit_view(window: &mut Window) -> Option<id> {
    let handle = HasWindowHandle::window_handle(window).ok()?;
    let RawWindowHandle::AppKit(handle) = handle.as_raw() else {
        return None;
    };
    let ns_view = handle.ns_view.as_ptr() as id;
    (!ns_view.is_null()).then_some(ns_view)
}

#[cfg(target_os = "macos")]
fn appkit_window(window: &mut Window) -> Option<id> {
    let ns_view = appkit_view(window)?;
    unsafe {
        let ns_window: id = msg_send![ns_view, window];
        (!ns_window.is_null()).then_some(ns_window)
    }
}
