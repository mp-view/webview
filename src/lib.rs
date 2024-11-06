#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use tao::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};
use wry::WebViewBuilder;

use napi::Result;

#[napi]
pub fn create_webview() -> Result<()> {
  let event_loop = EventLoop::new();
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

  let html_content = include_str!("./ui/index.html");

  let builder = WebViewBuilder::new()
    .with_transparent(true)
    .with_html(html_content);

  #[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
  ))]
  let _webview = builder.build(&window).unwrap();
  #[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
  )))]
  let _webview = {
    use tao::platform::unix::WindowExtUnix;
    use wry::WebViewBuilderExtUnix;
    let vbox = window.default_vbox().unwrap();
    builder.build_gtk(vbox)?
  };

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    if let Event::WindowEvent {
      event: WindowEvent::CloseRequested,
      ..
    } = event
    {
      *control_flow = ControlFlow::Exit;
    }
  });
}
