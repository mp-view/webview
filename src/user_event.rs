pub enum UserEvent {
  Pin,
  Minimize,
  Maximize,
  DragWindow,
  CloseWindow,
  MouseDown(i32, i32),
  MouseMove(i32, i32),
}
