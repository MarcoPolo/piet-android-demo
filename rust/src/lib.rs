#![cfg(target_os = "android")]
use android_logger::Config;
use jni_android_sys::android::graphics::Canvas;
use jni_android_sys::java::lang::String as JavaString;

use druid_shell;
use jni_glue::{Argument, Env};
use jni_sys::jobject;
use log::Level;
use log::*;
use time::get_time;
// use piet::kurbo::Line;
use piet::ImageFormat;
use piet::*;
use piet_common::{AndroidTextLayout, FontBuilder, Piet, Text, TextLayoutBuilder};
// use piet::{Color, ImageFormat, RenderContext};
use piet_common::{AndroidRenderContext, CanvasContext, Device};
use std::panic;

use std::any::Any;

use druid_shell::kurbo::{Line, Rect, Vec2};
use druid_shell::piet::{Color, RenderContext};

use druid_shell::{
    Application, Cursor, FileDialogOptions, FileSpec, HotKey, KeyEvent, KeyModifiers, Menu,
    MouseEvent, RunLoop, SysMods, TimerToken, WinCtx, WinHandler, WindowBuilder, WindowHandle,
};

pub fn enable_backtrace() {
    use std::env;
    let key = "RUST_BACKTRACE";
    env::set_var(key, "1");
}

fn init_log() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Error) // limit log level
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

#[no_mangle]
pub extern "system" fn Java_io_marcopolo_pietdemo_MainActivity_initRust(
    env: &Env,
    _this: jobject,
    cache_path: Argument<JavaString>,
) {
    init_log();
}

#[no_mangle]
pub extern "system" fn Java_io_marcopolo_pietdemo_MainActivity_startDruid(
    env: &Env,
    _this: jobject,
) {
    // main()
    perftest_main()
}

#[no_mangle]
pub extern "system" fn Java_io_marcopolo_pietdemo_PietDemoView_onDraw(
    env: &Env,
    _this: jobject,
    canvas: Argument<Canvas>,
) {
    info!("Here in onDraw");
    // let canvas = unsafe { canvas.with_unchecked(env).unwrap() };
    // let mut canvas_context = CanvasContext::new_from_canvas(&canvas);
    // let mut android_render_context = AndroidRenderContext::new(&mut canvas_context);
    // draw_on_render_context(&mut android_render_context);
}

const BG_COLOR: Color = Color::rgb8(0x27, 0x28, 0x22);
const FG_COLOR: Color = Color::rgb8(0xf0, 0xf0, 0xea);

#[derive(Default)]
struct HelloState {
    size: (f64, f64),
    handle: WindowHandle,
}

impl WinHandler for HelloState {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn paint(&mut self, piet: &mut piet_common::Piet, _ctx: &mut dyn WinCtx) -> bool {
        let (width, height) = self.size;
        let rect = Rect::new(0.0, 0.0, width, height);
        piet.fill(rect, &BG_COLOR);
        piet.stroke(Line::new((10.0, 50.0), (90.0, 90.0)), &FG_COLOR, 1.0);
        false
    }

    fn command(&mut self, id: u32, ctx: &mut dyn WinCtx) {
        match id {
            0x100 => {
                self.handle.close();
                Application::quit();
            }
            0x101 => {
                let options = FileDialogOptions::new().show_hidden().allowed_types(vec![
                    FileSpec::new("Rust Files", &["rs", "toml"]),
                    FileSpec::TEXT,
                    FileSpec::JPG,
                ]);
                let filename = ctx.open_file_sync(options);
                info!("result: {:?}", filename);
            }
            _ => info!("unexpected id {}", id),
        }
    }

    fn key_down(&mut self, event: KeyEvent, ctx: &mut dyn WinCtx) -> bool {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
        let id = ctx.request_timer(deadline);
        info!("keydown: {:?}, timer id = {:?}", event, id);
        false
    }

    fn wheel(&mut self, delta: Vec2, mods: KeyModifiers, _ctx: &mut dyn WinCtx) {
        info!("mouse_wheel {:?} {:?}", delta, mods);
    }

    fn mouse_move(&mut self, event: &MouseEvent, ctx: &mut dyn WinCtx) {
        ctx.set_cursor(&Cursor::Arrow);
        info!("mouse_move {:?}", event);
    }

    fn mouse_down(&mut self, event: &MouseEvent, _ctx: &mut dyn WinCtx) {
        info!("mouse_down {:?}", event);
    }

    fn mouse_up(&mut self, event: &MouseEvent, _ctx: &mut dyn WinCtx) {
        info!("mouse_up {:?}", event);
    }

    fn timer(&mut self, id: TimerToken, _ctx: &mut dyn WinCtx) {
        info!("timer fired: {:?}", id);
    }

    fn size(&mut self, width: u32, height: u32, _ctx: &mut dyn WinCtx) {
        let dpi = self.handle.get_dpi();
        let dpi_scale = dpi as f64 / 96.0;
        let width_f = (width as f64) / dpi_scale;
        let height_f = (height as f64) / dpi_scale;
        self.size = (width_f, height_f);
    }

    fn destroy(&mut self, _ctx: &mut dyn WinCtx) {
        Application::quit()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    Application::init();

    // let mut file_menu = Menu::new();
    // file_menu.add_item(
    //     0x100,
    //     "E&xit",
    //     Some(&HotKey::new(SysMods::Cmd, "q")),
    //     true,
    //     false,
    // );
    // file_menu.add_item(
    //     0x101,
    //     "O&pen",
    //     Some(&HotKey::new(SysMods::Cmd, "o")),
    //     true,
    //     false,
    // );
    // let mut menubar = Menu::new();
    // menubar.add_dropdown(Menu::new(), "Application", true);
    // menubar.add_dropdown(file_menu, "&File", true);

    let mut run_loop = RunLoop::new();
    let mut builder = WindowBuilder::new();
    builder.set_handler(Box::new(HelloState::default()));
    builder.set_title("Hello example");
    // builder.set_menu(menubar);
    let window = builder.build().unwrap();

    window.show();
    run_loop.run();
}

struct PerfTest {
    handle: WindowHandle,
    size: (f64, f64),
    last_time: f64,
    text: Option<AndroidTextLayout>,
}

impl WinHandler for PerfTest {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn paint(&mut self, piet: &mut piet_common::Piet, _ctx: &mut dyn WinCtx) -> bool {
        let (width, height) = self.size;
        let rect = Rect::new(0.0, 0.0, width, height);
        piet.fill(rect, &BG_COLOR);

        piet.stroke(Line::new((0.0, height), (width, 0.0)), &FG_COLOR, 1.0);

        let th = ::std::f64::consts::PI * (get_time().nsec as f64) * 2e-9;
        let dx = 100.0 * th.sin();
        let dy = 100.0 * th.cos();
        piet.stroke(
            Line::new((100.0, 100.0), (100.0 + dx, 100.0 - dy)),
            &FG_COLOR,
            1.0,
        );

        let font = piet
            .text()
            .new_font_by_name("Consolas", 15.0)
            .build()
            .unwrap();

        let now = get_time();
        let now = now.sec as f64 + 1e-9 * now.nsec as f64;
        let msg = format!("{:3.1}ms", 1e3 * (now - self.last_time));
        self.last_time = now;
        let layout = piet.text().new_text_layout(&font, &msg).build().unwrap();
        piet.draw_text(&layout, (10.0, 210.0), &FG_COLOR);

        if self.text.is_none() {
            let msg = "Hello DWrite! This is a somewhat longer string of text intended to provoke slightly longer draw times.";
            self.text = Some(piet.text().new_text_layout(&font, &msg).build().unwrap());
        }
        let dy = 15.0;
        let x0 = 210.0;
        let y0 = 10.0;
        for i in 0..60 {
            let y = y0 + (i as f64) * dy;
            piet.draw_text(&self.text.as_ref().unwrap(), (x0, y), &FG_COLOR);
        }

        true
    }

    fn command(&mut self, id: u32, _ctx: &mut dyn WinCtx) {
        match id {
            0x100 => self.handle.close(),
            _ => println!("unexpected id {}", id),
        }
    }

    fn key_down(&mut self, event: KeyEvent, _ctx: &mut dyn WinCtx) -> bool {
        println!("keydown: {:?}", event);
        false
    }

    fn size(&mut self, width: u32, height: u32, _ctx: &mut dyn WinCtx) {
        let dpi = self.handle.get_dpi();
        let dpi_scale = dpi as f64 / 96.0;
        let width_f = (width as f64) / dpi_scale;
        let height_f = (height as f64) / dpi_scale;
        self.size = (width_f, height_f);
    }

    fn destroy(&mut self, _ctx: &mut dyn WinCtx) {
        Application::quit()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn perftest_main() {
    Application::init();

    let mut run_loop = RunLoop::new();
    let mut builder = WindowBuilder::new();
    let perf_test = PerfTest {
        size: Default::default(),
        handle: Default::default(),
        last_time: 0.0,
        text: None,
    };
    builder.set_handler(Box::new(perf_test));
    builder.set_title("Performance tester");

    let window = builder.build().unwrap();
    window.show();
    run_loop.run();
}
