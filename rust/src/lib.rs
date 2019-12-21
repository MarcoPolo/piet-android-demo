#![cfg(target_os = "android")]
use android_logger::{Config, FilterBuilder};
use jni_glue::{Argument, Env};
use jni_sys::{jboolean, jobject, JNI_TRUE};
use log::Level;
use log::*;

fn init_log() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace) // limit log level
            .with_tag("Rust"), // logs will show under mytag tag
    );
}

#[no_mangle]
pub extern "system" fn Java_io_marcopolo_pietdemo_MainActivity_initRust(env: &Env, _this: jobject) {
    init_log();
    info!("Hello from rust!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
