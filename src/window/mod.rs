//! Tools for managing windows platform independently

pub mod xwindow;
pub mod event;

pub trait Display {
    /// Platform specific display error type
    type Error: std::error::Error;

    fn new() -> Result<Self, Self::Error>
        where Self: Sized;

    /// Creates a new Window Builder
    fn new_window_builder<'a>(&'a self) -> WindowBuilder<'a, Self>
            where Self: Sized {
        WindowBuilder::new(self)
    }

    fn get_screen_count(&self) -> usize;
    fn get_screen_dimension(&self, n: usize) -> Option<(u64, u64)>;
    fn get_main_screen(&self) -> usize;
}

/// Constructor for building a window
pub struct WindowBuilder<'a, D: Display> {
    dis: &'a D,
    title: Option<String>,
    pos: (Option<i64>, Option<i64>),
    size: (Option<u64>, Option<u64>),
    screen: Option<usize>,
    transparency: bool,
}

impl<'a, D: Display> WindowBuilder<'a, D> {
    fn new(dis: &'a D) -> Self {
        Self {
            dis,
            title: None,
            pos: (None, None),
            size: (None, None),
            screen: None,
            transparency: false,
        }
    }

    /// Sets the window title
    pub fn title(mut self, title: String) -> Self { self.title = Some(title); self }

    /// Sets the window position
    pub fn pos(mut self, x: i64, y: i64) -> Self { self.pos = (Some(x), Some(y)); self }
    /// Sets the window position's x-coordinate
    pub fn x(mut self, x: i64) -> Self { self.pos.0 = Some(x); self }
    /// Sets the window position's y-coordinate
    pub fn y(mut self, y: i64) -> Self { self.pos.1 = Some(y); self }

    /// Sets the window size
    pub fn size(mut self, w: u64, h: u64) -> Self { self.size = (Some(w), Some(h)); self }
    /// Sets the window width
    pub fn w(mut self, w: u64) -> Self { self.size.0 = Some(w); self }
    /// Sets the window height
    pub fn h(mut self, h: u64) -> Self { self.size.1 = Some(h); self }
    /// Sets the screen id
    pub fn screen(mut self, screen: usize) -> Self { self.screen = Some(screen); self }

    /// Set transparency support
    pub fn transparency(mut self, b: bool) -> Self { self.transparency = b; self }

    /// Gets the window title
    pub fn get_title(&self) -> Option<&str> { self.title.as_ref().map(String::as_str) }

    /// Gets the window position
    pub fn get_pos(&self) -> (Option<i64>, Option<i64>) { self.pos }
    /// Gets the window position's x-coordinate
    pub fn get_x(&self) -> Option<i64> { self.pos.0 }
    /// Gets the window position's y-coordinate
    pub fn get_y(&self) -> Option<i64> { self.pos.1 }

    /// Gets the window size
    pub fn get_size(&self) -> (Option<u64>, Option<u64>) { self.size }
    /// Gets the window width
    pub fn get_w(&self) -> Option<u64> { self.size.0 }
    /// Gets the window height
    pub fn get_h(&self) -> Option<u64> { self.size.1 }
    /// Gets the screen id
    pub fn get_screen(&self) -> Option<usize> { self.screen }
    
    /// Get transparency support
    pub fn get_transparency(&self) -> bool { self.transparency }

    /// Gets the display
    pub fn get_display(self) -> &'a D { self.dis }

    /// Tries to build a window by given configuration.
    /// On failiure returns the platform specific error type `Window::Error`.
    pub fn build<W: Window<'a, D>>(self) -> Result<W, W::Error> {
        W::new(self)
    }
}

/// A trait for windows of a specific platform
pub trait Window<'a, D: Display>: Iterator<Item=event::Event> {
    /// Platform specific window error type
    type Error: std::error::Error;
    /// Tries to create a new window. Use `WindowBuilder::build` instead
    fn new(wb: WindowBuilder<'a, D>) -> Result<Self, Self::Error>
            where Self: Sized;
}
