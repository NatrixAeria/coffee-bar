use super::XError;

pub struct Screen<'a> {
    screen: xcb::Screen<'a>
}

pub struct Display {
    main_screen: i32,
    con: xcb::Connection,
}

impl Display {
    pub fn get_screen<'a>(&'a self, n: usize) -> Option<xcb::Screen<'a>> {
        self.con.get_setup().roots().nth(n)
    }

    pub fn get_depth(&self, screen: usize, depth: u32, class: u8) -> Option<(xcb::Depth, xcb::Visualtype)> {
        self.get_screen(screen).and_then(|screen|
            screen.allowed_depths()
                  .filter(|d| d.depth() as u32 == depth)
                  .filter_map(|d| d.visuals().find(|v| v.class() == class).map(|v| (d, v)))
                  .next()
        )
    }

    pub fn con(&self) -> &xcb::Connection {
        &self.con
    }
}

impl super::super::Display for Display {
    type Error = XError;
    fn new() -> Result<Self, Self::Error> {
        let (con, screen_count) = xcb::Connection::connect(None).map_err(XError::ConnError)?;
        Ok(Self {
            con,
            main_screen: screen_count,
        })
    }

    fn get_screen_count(&self) -> usize {
        self.con.get_setup().roots_len().into()
    }

    fn get_screen_dimension(&self, n: usize) -> Option<(u64, u64)> {
        self.get_screen(n).map(|s| (s.width_in_pixels().into(), s.height_in_pixels().into()))
    }

    fn get_main_screen(&self) -> usize {
        self.main_screen as usize
    }
}
