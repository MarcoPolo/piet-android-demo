#![cfg(target_os = "android")]
use android_logger::{Config, FilterBuilder};
use jni_glue::{Argument, Env};
use jni_sys::{jboolean, jobject, JNI_TRUE};
use log::Level;
use log::*;
use piet::kurbo::Line;
use std::panic;

use piet::{Color, ImageFormat, RenderContext};
use piet_common::Device;

fn init_log() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace) // limit log level
            .with_tag("Rust"), // logs will show under mytag tag
    );
    panic::set_hook(Box::new(|panic_info| {
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

pub fn do_stuff<'a>(bitmap: &'a mut piet_common::BitmapTarget<'a, '_>) {}

#[no_mangle]
pub extern "system" fn Java_io_marcopolo_pietdemo_MainActivity_initRust(env: &Env, _this: jobject) {
    init_log();
    info!("Hello from rust!");
    let device = Device::new(env).unwrap();
    let width = 640;
    let height = 480;
    let mut bitmap = device.bitmap_target(width, height, 1.0).unwrap();
    let mut rc = bitmap.render_context();
    rc.clear(Color::WHITE);
    let brush = rc.solid_brush(Color::rgb8(0x00, 0x00, 0x80));
    rc.stroke(Line::new((10.0, 10.0), (100.0, 50.0)), &brush, 1.0);
    rc.finish().unwrap();
    let raw_pixels = bitmap.into_raw_pixels(ImageFormat::RgbaPremul).unwrap();
    info!("Got image. Now saving");
    image::save_buffer(
        "temp-image.png",
        &raw_pixels,
        width as u32,
        height as u32,
        image::ColorType::RGBA(8),
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
