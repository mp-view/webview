#![deny(clippy::all)]

mod user_event;
mod hit_test;

#[macro_use]
extern crate napi_derive;

use tao::{
  event::{Event, StartCause, WindowEvent},
  event_loop::{ControlFlow, EventLoopBuilder},
  window::WindowBuilder,
};
use wry::{http::Request, WebViewBuilder, WebViewBuilderExtWindows};
use window_vibrancy::*;

use user_event::UserEvent;
use hit_test::{hit_test, HitTestResult};

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
      use tao::platform::windows::WindowBuilderExtWindows;
      builder = builder.with_undecorated_shadow(false);
    }
    let window = builder.build(&event_loop).unwrap();
    
    #[cfg(target_os = "windows")]
    {
      use tao::platform::windows::WindowExtWindows;
      window.set_undecorated_shadow(true);
    }

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
      "mousedown" => {
        let x = req.next().unwrap().parse().unwrap();
        let y = req.next().unwrap().parse().unwrap();
        let _ = proxy.send_event(UserEvent::MouseDown(x, y));
      }
      "mousemove" => {
        let x = req.next().unwrap().parse().unwrap();
        let y = req.next().unwrap().parse().unwrap();
        let _ = proxy.send_event(UserEvent::MouseMove(x, y));
      }
      "pin" => {
        let _ = proxy.send_event(UserEvent::Pin);
      }
      _ => {}
    }
  };

  let builder = WebViewBuilder::new()
    .with_transparent(true)
    .with_ipc_handler(handler)
    .with_accept_first_mouse(true)
    .with_theme(wry::Theme::Dark)
    .with_html(html_content);

  #[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
  ))]
  let webview = builder.build(&window).unwrap();
  #[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
  )))]
  let webview = {
    use tao::platform::unix::WindowExtUnix;
    use wry::WebViewBuilderExtUnix;
    let vbox = window.default_vbox().unwrap();
    builder.build_gtk(vbox)?
  };

  #[cfg(target_os = "windows")]
  apply_mica(&window, None)
    .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

  let mut webview = Some(webview);

  #[cfg(target_os = "macos")]
  let _ = apply_vibrancy(webview.window(), NSVisualEffectMaterial::HudWindow, None, None)
    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");


  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      }
      | Event::UserEvent(UserEvent::CloseWindow) => {
        let _ = webview.take();
        *control_flow = ControlFlow::Exit
      }

      Event::UserEvent(e) => match e {
        UserEvent::Minimize => window.set_minimized(true),
        UserEvent::Maximize => window.set_maximized(!window.is_maximized()),
        UserEvent::DragWindow => window.drag_window().unwrap(),
        UserEvent::MouseDown(x, y) => {
          let res = hit_test(window.inner_size(), x, y, window.scale_factor());
          match res {
            HitTestResult::Client | HitTestResult::NoWhere => {}
            _ => res.drag_resize_window(&window),
          }
        }
        UserEvent::MouseMove(x, y) => {
          hit_test(window.inner_size(), x, y, window.scale_factor()).change_cursor(&window);
        }
        UserEvent::CloseWindow => { /* handled above */ }
        UserEvent::Pin => {
          let is_pin = window.is_always_on_top();
          window.set_always_on_top(!is_pin);
        }
      },
      _ => (),
    }
  });
}
