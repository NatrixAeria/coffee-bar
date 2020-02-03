use super::XError;

pub struct Display {
    main_screen: i32,
    con: xcb::Connection,
}

impl Display {
    fn get_screen<'a>(&'a self, n: usize) -> Option<xcb::Screen<'a>> {
        self.con.get_setup().roots().nth(n)
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
        self.get_screen(n).map(|screen|
                (screen.width_in_pixels().into(), screen.height_in_pixels().into()))
    }

    fn get_main_screen(&self) -> usize {
        self.main_screen as usize
    }
}
