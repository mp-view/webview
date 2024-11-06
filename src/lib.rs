#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use tao::{
  dpi::PhysicalSize,
  event::{Event, StartCause, WindowEvent},
  event_loop::{ControlFlow, EventLoopBuilder},
  window::{CursorIcon, ResizeDirection, Window, WindowBuilder},
};
use wry::{http::Request, WebViewBuilder};

use napi::Result;

#[derive(Debug)]
enum HitTestResult {
  Client,
  Left,
  Right,
  Top,
  Bottom,
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
  NoWhere,
}

impl HitTestResult {
  fn drag_resize_window(&self, window: &Window) {
    let _ = window.drag_resize_window(match self {
      HitTestResult::Left => ResizeDirection::West,
      HitTestResult::Right => ResizeDirection::East,
      HitTestResult::Top => ResizeDirection::North,
      HitTestResult::Bottom => ResizeDirection::South,
      HitTestResult::TopLeft => ResizeDirection::NorthWest,
      HitTestResult::TopRight => ResizeDirection::NorthEast,
      HitTestResult::BottomLeft => ResizeDirection::SouthWest,
      HitTestResult::BottomRight => ResizeDirection::SouthEast,
      _ => unreachable!(),
    });
  }

  fn change_cursor(&self, window: &Window) {
    window.set_cursor_icon(match self {
      HitTestResult::Left => CursorIcon::WResize,
      HitTestResult::Right => CursorIcon::EResize,
      HitTestResult::Top => CursorIcon::NResize,
      HitTestResult::Bottom => CursorIcon::SResize,
      HitTestResult::TopLeft => CursorIcon::NwResize,
      HitTestResult::TopRight => CursorIcon::NeResize,
      HitTestResult::BottomLeft => CursorIcon::SwResize,
      HitTestResult::BottomRight => CursorIcon::SeResize,
      _ => CursorIcon::Default,
    });
  }
}

fn hit_test(window_size: PhysicalSize<u32>, x: i32, y: i32, scale: f64) -> HitTestResult {
  const BORDERLESS_RESIZE_INSET: f64 = 5.0;

  const CLIENT: isize = 0b0000;
  const LEFT: isize = 0b0001;
  const RIGHT: isize = 0b0010;
  const TOP: isize = 0b0100;
  const BOTTOM: isize = 0b1000;
  const TOPLEFT: isize = TOP | LEFT;
  const TOPRIGHT: isize = TOP | RIGHT;
  const BOTTOMLEFT: isize = BOTTOM | LEFT;
  const BOTTOMRIGHT: isize = BOTTOM | RIGHT;

  let top = 0;
  let left = 0;
  let bottom = top + window_size.height as i32;
  let right = left + window_size.width as i32;

  let inset = (BORDERLESS_RESIZE_INSET * scale) as i32;

  #[rustfmt::skip]
      let result =
          (LEFT * (if x < (left + inset) { 1 } else { 0 }))
        | (RIGHT * (if x >= (right - inset) { 1 } else { 0 }))
        | (TOP * (if y < (top + inset) { 1 } else { 0 }))
        | (BOTTOM * (if y >= (bottom - inset) { 1 } else { 0 }));

  match result {
    CLIENT => HitTestResult::Client,
    LEFT => HitTestResult::Left,
    RIGHT => HitTestResult::Right,
    TOP => HitTestResult::Top,
    BOTTOM => HitTestResult::Bottom,
    TOPLEFT => HitTestResult::TopLeft,
    TOPRIGHT => HitTestResult::TopRight,
    BOTTOMLEFT => HitTestResult::BottomLeft,
    BOTTOMRIGHT => HitTestResult::BottomRight,
    _ => HitTestResult::NoWhere,
  }
}

enum UserEvent {
  Minimize,
  Maximize,
  DragWindow,
  CloseWindow,
  MouseDown(i32, i32),
  MouseMove(i32, i32),
}

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
      _ => {}
    }
  };

  let builder = WebViewBuilder::new()
    .with_transparent(true)
    .with_ipc_handler(handler)
    .with_accept_first_mouse(true)
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

  let mut webview = Some(webview);


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
      },
      _ => (),
    }
  });
}
