pub enum UserEvent {
  Pin,
  Minimize,
  Maximize,
  DragWindow,
  CloseWindow,
  Devtools,
  MenuMaximize,
  Deviceinfo(String),
}
