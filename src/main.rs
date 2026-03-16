#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/// Binary - entry point for desktop apps.
fn main() -> xframe::Result {
    xframe_app::desktop_main()
}
