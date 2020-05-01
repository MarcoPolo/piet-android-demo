#![cfg(target_os = "android")]
use android_logger::Config;
use jni_android_sys::android::graphics::Canvas;
use jni_android_sys::java::lang::String as JavaString;

use jni_glue::{Argument, Env};
use jni_sys::jobject;
use log::Level;
use log::*;
use piet::kurbo::Line;
use piet::{Color, ImageFormat, RenderContext};
use piet_common::{AndroidRenderContext, CanvasContext, Device};
use piet_test;
use std::panic;

pub fn enable_backtrace() {
    use std::env;
    let key = "RUST_BACKTRACE";
    env::set_var(key, "1");
}

fn init_log() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace) // limit log level
            .with_tag("Rust"), // logs will show under mytag tag
    );
    enable_backtrace();
    panic::set_hook(Box::new(move |panic_info| {
        let location = panic_info.location().unwrap();
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            format!("panic occurred: {:?}", s)
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            format!("panic occurred: {:?}", s)
        } else {
            "panic occurred".to_string()
        };
        error!("{} at {}", msg, location);
    }));
}

fn draw_on_render_context(rc: &mut AndroidRenderContext<'_>) {
    rc.clear(Color::WHITE);
    let brush = rc.solid_brush(Color::rgb8(0x00, 0x00, 0x80));
    rc.stroke(Line::new((10.0, 10.0), (100.0, 50.0)), &brush, 1.0);
    rc.finish().unwrap();
}

#[no_mangle]
pub extern "system" fn Java_io_marcopolo_pietdemo_MainActivity_initRust(
    env: &Env,
    _this: jobject,
    cache_path: Argument<JavaString>,
) {
    init_log();
    let cache_path = unsafe {
        cache_path
            .with_unchecked(env)
            .expect("Cache path not given")
    };
    let cache_path = cache_path.to_string_lossy();

    let device = Device::new(env).unwrap();
    let width = 640;
    let height = 480;
    let mut bitmap = device.bitmap_target(width, height, 1.0).unwrap();
    let mut rc = bitmap.render_context();
    draw_on_render_context(&mut rc);
    let raw_pixels = bitmap
        .to_raw_pixels(ImageFormat::RgbaSeparate)
        .expect("Failed to get raw_pixels");
    let image_path = format!("{}/temp-image.png", cache_path);
    image::save_buffer(
        &image_path,
        &raw_pixels,
        width as u32,
        height as u32,
        image::ColorType::RGBA(8),
    )
    .expect("Failed to save buffer");
    info!("Saved image to {}", image_path);
}

use std::cell::RefCell;
thread_local! {
    pub static CURRENT_TEST: RefCell<usize> = RefCell::new(2);
}
static MAX_TESTS: usize = 6;

#[no_mangle]
pub extern "system" fn Java_io_marcopolo_pietdemo_MainActivity_drawNext(
    env: &Env,
    _this: jobject,
    cache_path: Argument<JavaString>,
) {
    CURRENT_TEST.with(|current_test| {
        *current_test.borrow_mut() += 1;
        *current_test.borrow_mut() %= MAX_TESTS;
    });
}

#[no_mangle]
pub extern "system" fn Java_io_marcopolo_pietdemo_PietDemoView_onDraw(
    env: &Env,
    _this: jobject,
    canvas: Argument<Canvas>,
) {
    info!("Here in onDraw");
    let canvas = unsafe { canvas.with_unchecked(env).unwrap() };
    let canvas_context = CanvasContext::new_from_canvas(canvas);
    // Constant for Nexus 5x x86 sim
    static SCALE_FACTOR: f32 = 2.625 * 160.0 / 96.0;
    let mut android_render_context = AndroidRenderContext::new(canvas_context, SCALE_FACTOR);
    let n = CURRENT_TEST.with(|current_test| *current_test.borrow());
    // if n == 2 {
    //     // Skipping because of unhandled image format
    //     return;
    // }
    piet_test::draw_test_picture(
        &mut android_render_context,
        CURRENT_TEST.with(|current_test| *current_test.borrow()),
    );
}
