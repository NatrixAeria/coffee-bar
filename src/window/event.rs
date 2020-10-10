#[derive(Debug, Clone, Copy)]
pub enum Button {
    Left,
    Right,
    Middle,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
}

#[derive(Debug, Clone)]
pub enum Event {
    ButtonDown(Button, (i32, i32)),
    ButtonUp(Button, (i32, i32)),
    ButtonMove(Button, (i32, i32)),
}
