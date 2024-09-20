// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;
use gdk4_sys as gdk;
use gtk4_sys as gtk;

mod manual;

pub use manual::*;

#[allow(unused_imports)]
use std::ffi::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong, c_void};
#[allow(unused_imports)]
use libc::{size_t, ssize_t, time_t, off_t, intptr_t, uintptr_t, FILE};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GtkLayerShellEdge = c_int;
pub const GTK_LAYER_SHELL_EDGE_LEFT: GtkLayerShellEdge = 0;
pub const GTK_LAYER_SHELL_EDGE_RIGHT: GtkLayerShellEdge = 1;
pub const GTK_LAYER_SHELL_EDGE_TOP: GtkLayerShellEdge = 2;
pub const GTK_LAYER_SHELL_EDGE_BOTTOM: GtkLayerShellEdge = 3;
pub const GTK_LAYER_SHELL_EDGE_ENTRY_NUMBER: GtkLayerShellEdge = 4;

pub type GtkLayerShellKeyboardMode = c_int;
pub const GTK_LAYER_SHELL_KEYBOARD_MODE_NONE: GtkLayerShellKeyboardMode = 0;
pub const GTK_LAYER_SHELL_KEYBOARD_MODE_EXCLUSIVE: GtkLayerShellKeyboardMode = 1;
pub const GTK_LAYER_SHELL_KEYBOARD_MODE_ON_DEMAND: GtkLayerShellKeyboardMode = 2;
pub const GTK_LAYER_SHELL_KEYBOARD_MODE_ENTRY_NUMBER: GtkLayerShellKeyboardMode = 3;

pub type GtkLayerShellLayer = c_int;
pub const GTK_LAYER_SHELL_LAYER_BACKGROUND: GtkLayerShellLayer = 0;
pub const GTK_LAYER_SHELL_LAYER_BOTTOM: GtkLayerShellLayer = 1;
pub const GTK_LAYER_SHELL_LAYER_TOP: GtkLayerShellLayer = 2;
pub const GTK_LAYER_SHELL_LAYER_OVERLAY: GtkLayerShellLayer = 3;
pub const GTK_LAYER_SHELL_LAYER_ENTRY_NUMBER: GtkLayerShellLayer = 4;

extern "C" {

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gtk_layer_auto_exclusive_zone_enable(window: *mut gtk::GtkWindow);
    pub fn gtk_layer_auto_exclusive_zone_is_enabled(window: *mut gtk::GtkWindow) -> gboolean;
    pub fn gtk_layer_get_anchor(window: *mut gtk::GtkWindow, edge: GtkLayerShellEdge) -> gboolean;
    pub fn gtk_layer_get_exclusive_zone(window: *mut gtk::GtkWindow) -> c_int;
    pub fn gtk_layer_get_keyboard_mode(window: *mut gtk::GtkWindow) -> GtkLayerShellKeyboardMode;
    pub fn gtk_layer_get_layer(window: *mut gtk::GtkWindow) -> GtkLayerShellLayer;
    pub fn gtk_layer_get_major_version() -> c_uint;
    pub fn gtk_layer_get_margin(window: *mut gtk::GtkWindow, edge: GtkLayerShellEdge) -> c_int;
    pub fn gtk_layer_get_micro_version() -> c_uint;
    pub fn gtk_layer_get_minor_version() -> c_uint;
    pub fn gtk_layer_get_monitor(window: *mut gtk::GtkWindow) -> *mut gdk::GdkMonitor;
    pub fn gtk_layer_get_namespace(window: *mut gtk::GtkWindow) -> *const c_char;
    pub fn gtk_layer_get_protocol_version() -> c_uint;
    pub fn gtk_layer_get_zwlr_layer_surface_v1(window: *mut gtk::GtkWindow) -> *mut zwlr_layer_surface_v1;
    pub fn gtk_layer_init_for_window(window: *mut gtk::GtkWindow);
    pub fn gtk_layer_is_layer_window(window: *mut gtk::GtkWindow) -> gboolean;
    pub fn gtk_layer_is_supported() -> gboolean;
    pub fn gtk_layer_set_anchor(window: *mut gtk::GtkWindow, edge: GtkLayerShellEdge, anchor_to_edge: gboolean);
    pub fn gtk_layer_set_exclusive_zone(window: *mut gtk::GtkWindow, exclusive_zone: c_int);
    pub fn gtk_layer_set_keyboard_mode(window: *mut gtk::GtkWindow, mode: GtkLayerShellKeyboardMode);
    pub fn gtk_layer_set_layer(window: *mut gtk::GtkWindow, layer: GtkLayerShellLayer);
    pub fn gtk_layer_set_margin(window: *mut gtk::GtkWindow, edge: GtkLayerShellEdge, margin_size: c_int);
    pub fn gtk_layer_set_monitor(window: *mut gtk::GtkWindow, monitor: *mut gdk::GdkMonitor);
    pub fn gtk_layer_set_namespace(window: *mut gtk::GtkWindow, name_space: *const c_char);

}
