#![deny(clippy::all)]

mod user_event;

#[macro_use]
extern crate napi_derive;

use tao::{
  dpi::{PhysicalSize, Size}, 
  event::{Event, StartCause, WindowEvent}, 
  event_loop::{ControlFlow, EventLoopBuilder}, 
  window::{Theme, WindowBuilder}
};
use user_event::UserEvent;
use wry::{
  dpi::{LogicalPosition, LogicalSize},
  http::Request,
  Rect, WebViewBuilder,
};

#[cfg(target_os = "windows")]
use tao::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};

use napi::Result;

use serde::Deserialize;

#[derive(Deserialize)]
struct Device {
    name: String,
    size: [u32; 2],
    user_agent: String,
}

#[napi(object)]
pub struct WebviewOptions {
  pub url: String,
  pub width: u32,
  pub height: u32,
}

#[napi]
pub fn create_webview(options: WebviewOptions) -> Result<()> {
 
  const MENU_HEIGHT: u32 = 65;
  const HTML_CONTENT: &str = include_str!("ui/index.html");
  let default_width = options.width;
  let default_height = options.height;
 
 
 let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

  #[allow(unused_mut)]
  let mut builder = WindowBuilder::new()
    .with_decorations(false)
    .with_theme(Some(Theme::Dark))
    .with_always_on_top(true)
    .with_resizable(false)
    .with_inner_size(LogicalSize {
      width: options.width,
      height: options.height + MENU_HEIGHT,
    })
    .with_title("mp-view")
    .with_transparent(true);

  #[cfg(target_os = "windows")]
  {
    builder = builder.with_undecorated_shadow(false);
  }

  let window = builder.build(&event_loop).unwrap();



  // #[cfg(target_os = "windows")] {
  //   use window_vibrancy::apply_tabbed;
  //   apply_tabbed(&window, None)
  //   .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
  //   window.set_undecorated_shadow(true);
  // }

  // #[cfg(target_os = "macos")] {
  //   use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
  //   apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
  //   .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
  // }



  let proxy = event_loop.create_proxy();
  let handler = move |req: Request<String>| {
    let body = req.body();
    let mut req = body.split(['-']);
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
      "menu_maximize" => {
        let _ = proxy.send_event(UserEvent::MenuMaximize);
      }
      "device_info" => {
        let info_json = req.next().unwrap().parse().unwrap();
        println!("device: {:?}", info_json);
        let _ = proxy.send_event(UserEvent::Deviceinfo(info_json));
      }
      _ => {}
    }
  };

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
    .with_transparent(true)
    .with_ipc_handler(handler)
    .with_accept_first_mouse(true)
    .with_html(HTML_CONTENT)
    .with_bounds(Rect {
      position: LogicalPosition::new(0, 0).into(),
      size: LogicalSize::new(default_width, MENU_HEIGHT).into(),
    });

  let mp_builder = WebViewBuilder::new()
    .with_transparent(true)
    .with_accept_first_mouse(true)
    .with_url(options.url)
    .with_bounds(Rect {
      position: LogicalPosition::new(0, MENU_HEIGHT).into(),
      size: LogicalSize::new(default_width, default_height).into(),
    });

    let mp_webview = build_webview(mp_builder).unwrap();
    let menu_webview = build_webview(menu_builder).unwrap();

  

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      }
      | Event::UserEvent(UserEvent::CloseWindow) => {
        // let _ = menu_webview.take();
        *control_flow = ControlFlow::Exit
      }

      Event::UserEvent(e) => match e {
        UserEvent::Minimize => window.set_minimized(true),
        UserEvent::Maximize => window.set_maximized(!window.is_maximized()),
        UserEvent::DragWindow => window.drag_window().unwrap(),
        UserEvent::CloseWindow => { /* handled above */ }
        UserEvent::Pin => {
          let is_pin = window.is_always_on_top();
          println!("is_pin: {}", is_pin);
          window.set_always_on_top(!is_pin);
        }
        UserEvent::Devtools => {
          let is_devtools = mp_webview.is_devtools_open();

          println!("is_devtools: {}", is_devtools);
          if is_devtools {
            mp_webview.close_devtools();
          } else {
            mp_webview.open_devtools();
          }
        }
        UserEvent::MenuMaximize => {
          // 获取window的窗口高度
          let window_height = window.inner_size().height;
          let scale_factor = window.scale_factor();
          // 获取mencu_webview的窗口高度
          let current_height: u32 = menu_webview.bounds().unwrap().size.to_logical(scale_factor).height;
          let current_width: u32 = menu_webview.bounds().unwrap().size.to_logical(scale_factor).width;
          let height = if current_height == MENU_HEIGHT {
            window_height
          } else {
            MENU_HEIGHT
          };
          println!("current_height: {}", current_height);
          println!("height: {}", height);
          // 设置menu_webview的高度
          menu_webview.set_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(current_width, height).into(),
          }).unwrap();
        }
        UserEvent::Deviceinfo(device_json) => {
          let device: Device = serde_json::from_str(&device_json).unwrap();
          let size = window.inner_size();
          println!("size: {:?}", size);
          window.set_inner_size(LogicalSize {
            width: device.size[0],
            height: device.size[1] + MENU_HEIGHT,
          });
          menu_webview.set_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(device.size[0], MENU_HEIGHT).into(), 
          }).unwrap();
          mp_webview.set_bounds(Rect {
            position: LogicalPosition::new(0, MENU_HEIGHT).into(),
            size: LogicalSize::new(device.size[0], device.size[1]).into(),
          }).unwrap();
          println!("device_json: {:?}", device.size);
        }
      },
      _ => (),
    }
  });
}
