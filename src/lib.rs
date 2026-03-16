#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod platform;

#[cfg(target_os = "android")]
use std::ffi::c_void;

/// Entry point for a ios app.
#[cfg(target_os = "ios")]
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn ios_main() {
    oslog::OsLogger::new("eframe_app")
        .level_filter(log::LevelFilter::Info)
        .init()
        .unwrap();

    let native_options = xframe::NativeOptions {
        // On iOS 'run_and_return' ('EventLoop::run_app_on_demand')
        // is not not supported.
        run_and_return: false,
        ..Default::default()
    };

    let _ = run(native_options);
}

/// Entry point for a android app.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: xframe::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = xframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    let _ = run(options);
}

/// Entry point for a desktop app - called from `main.rs/main`.
pub fn desktop_main() -> xframe::Result {
    #[cfg(target_os = "macos")]
    oslog::OsLogger::new("egui_app")
        .level_filter(log::LevelFilter::Info)
        .init()
        .unwrap();

    let native_options = xframe::NativeOptions {
        viewport: xframe::egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_min_inner_size([400.0, 400.0])
            .with_fullsize_content_view(true)
            .with_title_shown(false)
            .with_titlebar_shown(false)
            .with_icon(
                xframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    run(native_options)
}

fn run(native_options: xframe::NativeOptions) -> xframe::Result {
    let xframe::CreateContext {
        egui_ctx,
        proxy,
        runner,
    } = xframe::get_create_context::<crate::app::ExtEvent>(
        crate::app::config::CONF_APP_NAME,
        native_options,
    )?;

    let xframe_proxy = std::sync::Arc::new(proxy);

    #[cfg(target_os = "android")]
    set_android_proxy(xframe_proxy.clone());

    let main_application = app::MainApplication::new(egui_ctx, xframe_proxy);

    runner.run_app(Box::new(main_application))
}

// -----------------------------------------------------------------
// Android specific:

/// Called from android main activity on insets-listener events.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "system" fn Java_one_lopes_eguiapp_MainActivity_onDisplayInsets(
    _: *mut c_void,
    _: *mut c_void,
    top_inset: i32,
    right_inset: i32,
    bottom_inset: i32,
    left_inset: i32,
) {
    let insets = xframe::epaint::MarginF32 {
        left: left_inset as f32,
        right: right_inset as f32,
        top: top_inset as f32,
        bottom: bottom_inset as f32,
    };

    let _ = get_android_proxy().send_safe_area_insets(insets);
}

/// Called from android main activity on back-press events.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "system" fn Java_one_lopes_eguiapp_MainActivity_onBackPressLocation(
    _: *mut c_void,
    _: *mut c_void,
    x_location: f32,
    y_location: f32,
    touch_phase: i16,
) {
    let position = xframe::winit::dpi::PhysicalPosition {
        x: x_location as f64,
        y: y_location as f64,
    };

    let touch_phase = match touch_phase {
        1 => xframe::winit::event::TouchPhase::Started,
        2 => xframe::winit::event::TouchPhase::Moved,
        3 => xframe::winit::event::TouchPhase::Ended,
        _ => unreachable!(
            "from java received touch phase value '{touch_phase}' is not in the expected range 1..3"
        ),
    };

    let _ = get_android_proxy().send_touch_location(position, touch_phase);
}

/// Proxy must be set when calling this, panics otherwise.
#[cfg(target_os = "android")]
fn get_android_proxy<'s>() -> &'s std::sync::Arc<xframe::XFrameProxy<crate::app::ExtEvent>> {
    #[expect(static_mut_refs)]
    unsafe { &ANDROID_PROXY }
        .as_ref()
        .expect("Android proxy must be set at this point")
}

#[cfg(target_os = "android")]
fn set_android_proxy(proxy: std::sync::Arc<xframe::XFrameProxy<crate::app::ExtEvent>>) {
    unsafe { ANDROID_PROXY = Some(proxy) };
}

#[cfg(target_os = "android")]
static mut ANDROID_PROXY: Option<std::sync::Arc<xframe::XFrameProxy<crate::app::ExtEvent>>> = None;
