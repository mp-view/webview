#![deny(clippy::all)]

mod user_event;

#[macro_use]
extern crate napi_derive;

use tao::{
  event::{Event, StartCause, WindowEvent},
  event_loop::{ControlFlow, EventLoopBuilder},
  window::WindowBuilder,
};
use user_event::UserEvent;
use window_vibrancy::{apply_mica, apply_vibrancy, NSVisualEffectMaterial};
use wry::{
  dpi::{LogicalPosition, LogicalSize},
  http::Request,
  Rect, WebViewBuilder,
};

#[cfg(target_os = "windows")]
use tao::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};

use napi::Result;

#[napi]
pub fn create_webview() -> Result<()> {
  let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

  #[allow(unused_mut)]
  let mut builder = WindowBuilder::new()
    .with_decorations(false)
    .with_always_on_top(true)
    .with_transparent(true);

  #[cfg(target_os = "windows")]
  {
    builder = builder.with_undecorated_shadow(false);
  }

  let window = builder.build(&event_loop).unwrap();

  #[cfg(target_os = "windows")]
  apply_mica(&window, None)
    .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

  #[cfg(target_os = "macos")]
  apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

  #[cfg(target_os = "windows")]
  window.set_undecorated_shadow(true);

  let html_content = include_str!("ui/index.html");

  let proxy = event_loop.create_proxy();
  let handler = move |req: Request<String>| {
    let body = req.body();
    let mut req = body.split([':', ',']);
    match req.next().unwrap() {
      "minimize" => {
        let _ = proxy.send_event(UserEvent::Minimize);
      }
      "maximize" => {
        let _ = proxy.send_event(UserEvent::Maximize);
      }
      "drag_window" => {
        let _ = proxy.send_event(UserEvent::DragWindow);
      }
      "close" => {
        let _ = proxy.send_event(UserEvent::CloseWindow);
      }
      "pin" => {
        let _ = proxy.send_event(UserEvent::Pin);
      }
      "devtools" => {
        let _ = proxy.send_event(UserEvent::Devtools);
      }
      _ => {}
    }
  };

  let size = window.inner_size().to_logical::<u32>(window.scale_factor());

  let build_webview = |builder: WebViewBuilder<'_>| -> wry::Result<wry::WebView> {
    #[cfg(any(
      target_os = "windows",
      target_os = "macos",
      target_os = "ios",
      target_os = "android"
    ))]
    let webview = builder.build_as_child(&window)?;

    #[cfg(not(any(
      target_os = "windows",
      target_os = "macos",
      target_os = "ios",
      target_os = "android"
    )))]
    let webview = {
      use gtk::prelude::*;
      use tao::platform::unix::WindowExtUnix;
      use wry::WebViewBuilderExtUnix;

      let fixed = gtk::Fixed::new();
      let vbox = window.default_vbox().unwrap();
      vbox.pack_start(&fixed, true, true, 0);
      fixed.show_all();
      builder.build_gtk(&fixed)?
    };

    Ok(webview)
  };

  let menu_builder = WebViewBuilder::new()
    .with_bounds(Rect {
      position: LogicalPosition::new(0, 0).into(),
      size: LogicalSize::new(size.width, 60).into(),
    })
    .with_transparent(true)
    .with_ipc_handler(handler)
    .with_accept_first_mouse(true)
    .with_html(html_content);

  let mp_builder = WebViewBuilder::new()
    .with_transparent(true)
    .with_url("https://www.baidu.com")
    .with_bounds(Rect {
      position: LogicalPosition::new(0, 60).into(),
      size: LogicalSize::new(size.width, 500).into(),
    });

  let menu_webview = build_webview(menu_builder).unwrap();
  let mp_webview = build_webview(mp_builder).unwrap();

  let mut menu_webview = Some(menu_webview);

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      }
      | Event::UserEvent(UserEvent::CloseWindow) => {
        let _ = menu_webview.take();
        *control_flow = ControlFlow::Exit
      }

      Event::UserEvent(e) => match e {
        UserEvent::Minimize => window.set_minimized(true),
        UserEvent::Maximize => window.set_maximized(!window.is_maximized()),
        UserEvent::DragWindow => window.drag_window().unwrap(),
        UserEvent::CloseWindow => { /* handled above */ }
        UserEvent::Pin => {
          let is_pin = window.is_always_on_top();
          window.set_always_on_top(!is_pin);
        }
        UserEvent::Devtools => {
          let is_devtools = mp_webview.is_devtools_open();
          if is_devtools {
            mp_webview.close_devtools();
          } else {
            mp_webview.open_devtools();
          }
        }
      },
      _ => (),
    }
  });
}
