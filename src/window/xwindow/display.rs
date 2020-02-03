use super::XError;

pub struct Display {
    main_screen: i32,
    con: xcb::Connection,
}

impl<'a> super::super::Display for Display {
    type Error = XError;
    fn new() -> Result<Self, Self::Error> {
        let (con, screen_count) = xcb::Connection::connect(None).map_err(XError::ConnError)?;
        let setup = con.get_setup();
        Ok(Self {
            con,
            main_screen: screen_count
        })
    }
}
