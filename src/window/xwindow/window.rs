use super::super::{
    color::{Color, PixelFormat},
    draw::DrawCommand,
    event, Display, Surface, WindowBuilder, WindowType,
};
use super::Display as XDisplay;
use super::XError;

pub struct Window<'a> {
    dis: &'a XDisplay,
    screen: xcb::Screen<'a>,
    win: xcb::Window,
    transparency: bool,
    size: (u16, u16),
}

impl<'a> Window<'a> {
    fn create_pixmap(&self, w: u64, h: u64) -> (xcb::VoidCookie, u32) {
        let con = self.dis.con();
        let pix = con.generate_id();
        let depth = if self.transparency { 24 } else { 32 };
        (
            xcb::create_pixmap(con, depth, pix, self.win, w as u16, h as u16),
            pix,
        )
    }

    fn translate_button(button: xcb::Button) -> Option<event::Button> {
        match button.into() {
            xcb::BUTTON_INDEX_1 => Some(event::Button::Left),
            xcb::BUTTON_INDEX_2 => Some(event::Button::Middle),
            xcb::BUTTON_INDEX_3 => Some(event::Button::Right),
            xcb::BUTTON_INDEX_4 => Some(event::Button::ScrollUp),
            xcb::BUTTON_INDEX_5 => Some(event::Button::ScrollDown),
            6 => Some(event::Button::ScrollLeft),
            7 => Some(event::Button::ScrollRight),
            _ => None,
        }
    }

    fn create_gc(&mut self) -> (xcb::VoidCookie, u32) {
        let gc = self.dis.con().generate_id();
        (
            xcb::create_gc(
                self.dis.con(),
                gc,
                self.screen.root(),
                &[(xcb::GC_FOREGROUND, self.screen.black_pixel())],
            ),
            gc,
        )
    }

    fn translate_button_mask(button: u16) -> Option<event::Button> {
        let zeros = (button & !0xff).trailing_zeros();
        if zeros < 8 {
            None
        } else {
            Self::translate_button(zeros as u8 - 7)
        }
    }

    fn redraw(&mut self, x: u16, y: u16, w: u16, h: u16) -> Result<(), XError> {
        println!("redrawing {},{} - {},{}", x, y, w, h);
        Ok(())
    }

    pub fn fetch_event(&mut self) -> Option<Result<event::Event, Option<XError>>> {
        let con = self.dis.con();
        Some(con.wait_for_event().ok_or(None).and_then(|event| {
            let r = event.response_type();
            match r {
                xcb::EXPOSE => unsafe {
                    let event: &xcb::ExposeEvent = xcb::cast_event(&event);
                    Err(self
                        .redraw(event.x(), event.y(), event.width(), event.height())
                        .err())
                },
                xcb::BUTTON_PRESS => unsafe {
                    let event: &xcb::ButtonPressEvent = xcb::cast_event(&event);
                    Self::translate_button(event.detail())
                        .map(|b| (b, (event.event_x().into(), event.event_y().into())))
                }
                .map(|(b, s)| event::Event::ButtonDown(b, s))
                .ok_or(None),
                xcb::BUTTON_RELEASE => unsafe {
                    let event: &xcb::ButtonReleaseEvent = xcb::cast_event(&event);
                    Self::translate_button(event.detail())
                        .map(|b| (b, (event.event_x().into(), event.event_y().into())))
                }
                .map(|(b, s)| event::Event::ButtonUp(b, s))
                .ok_or(None),
                xcb::MOTION_NOTIFY => unsafe {
                    let event: &xcb::MotionNotifyEvent = xcb::cast_event(&event);
                    Self::translate_button_mask(event.state())
                        .map(|b| (b, (event.event_x().into(), event.event_y().into())))
                }
                .map(|(b, s)| event::Event::ButtonMove(b, s))
                .ok_or(None),
                _ => Err(None),
            }
        }))
    }
}

impl<'a> super::super::Window<'a, XDisplay> for Window<'a> {
    type Error = XError;
    fn new(wb: WindowBuilder<'a, XDisplay>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let transparency = wb.get_transparency();
        let pos = (
            wb.get_x().unwrap_or(0) as i16,
            wb.get_y().unwrap_or(0) as i16,
        );
        let size = (
            wb.get_w().unwrap_or(100) as u16,
            wb.get_h().unwrap_or(100) as u16,
        );
        let screen_id = wb.get_screen();
        let window_type = wb.get_window_type();
        let dis = wb.get_display();

        let con = dis.con();
        let win = con.generate_id();
        let screen_id = screen_id.unwrap_or_else(|| dis.get_main_screen());
        let screen = dis
            .get_screen(screen_id)
            .ok_or_else(|| XError::ScreenError(String::from("could not find requested screen")))?;
        let mut visual = None;
        let mut depth_val = xcb::COPY_FROM_PARENT as u8;
        let mut cw_values = vec![];

        if transparency {
            if let Some((depth, vis)) =
                dis.get_depth(screen_id, 32, xcb::VISUAL_CLASS_TRUE_COLOR as u8)
            {
                let colormap = con.generate_id();
                xcb::create_colormap(
                    con,
                    // Colormap entries to be allocated (AllocNone or AllocAll)
                    xcb::COLORMAP_ALLOC_NONE as u8,
                    // Id of the color map
                    colormap,
                    // Window on whose screen the colormap will be created
                    screen.root(),
                    // Id of the visual supported by the screen
                    vis.visual_id(),
                )
                .request_check()?;
                visual = Some(vis.visual_id());
                cw_values.push((xcb::CW_COLORMAP, colormap));
                cw_values.push((xcb::CW_BORDER_PIXEL, screen.black_pixel()));
                depth_val = depth.depth();
            }
        }
        // Defines how the window should be repositioned if the parent is resized
        cw_values.push((xcb::CW_WIN_GRAVITY, xcb::GRAVITY_NORTH));
        cw_values.push((xcb::CW_BACK_PIXEL, screen.black_pixel()));
        cw_values.push((
            xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_EXPOSURE
                | xcb::EVENT_MASK_BUTTON_PRESS
                | xcb::EVENT_MASK_BUTTON_RELEASE
                | xcb::EVENT_MASK_BUTTON_MOTION,
        ));
        xcb::create_window(
            con,
            depth_val,
            win,
            screen.root(),
            pos.0,
            pos.1,
            size.0,
            size.1,
            0, // border width
            xcb::xproto::WINDOW_CLASS_INPUT_OUTPUT as u16,
            visual.unwrap_or_else(|| screen.root_visual()),
            &cw_values,
        )
        .request_check()?;

        match window_type {
            WindowType::Normal => (),
            WindowType::Docking => {
                let window_type = dis.get_intern_atom("_NET_WM_WINDOW_TYPE").unwrap().unwrap();
                let docking = dis
                    .get_intern_atom("_NET_WM_WINDOW_TYPE_DOCK")
                    .unwrap()
                    .unwrap();
                xcb::change_property(
                    con,
                    xcb::PROP_MODE_REPLACE as u8,
                    win,
                    window_type,
                    xcb::ATOM as u32,
                    // Specifies whether the data should be viewed as a
                    // list of 8-bit, 16-bit, or 32-bit quantities
                    32,
                    &[docking],
                )
                .request_check()?;
            }
        }

        xcb::map_window(con, win).request_check()?;
        con.flush();

        Ok(Self {
            dis,
            screen,
            transparency,
            win,
            size,
        })
    }
}

pub struct WindowSurface<'s, 'a> {
    win: &'s mut Window<'a>,
}

impl<'s, 'a, C: Color> Surface<C> for WindowSurface<'s, 'a>
where
    'a: 's,
{
    type Error = XError;
    fn get_width(&self) -> u64 {
        self.win.size.0.into()
    }
    fn get_height(&self) -> u64 {
        self.win.size.1.into()
    }
    fn get_pixel_format(&self) -> PixelFormat {
        if self.win.transparency {
            PixelFormat::Rgba32
        } else {
            PixelFormat::Rgb24
        }
    }
    fn draw(&self, draw: DrawCommand<C>) -> Result<(), Self::Error> {
        todo!();
    }
}

impl<'a> Iterator for Window<'a> {
    type Item = Result<event::Event, XError>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            break match self.fetch_event() {
                Some(Ok(event)) => Some(Ok(event)),
                Some(Err(Some(err))) => Some(Err(err)),
                None => None,
                Some(Err(None)) => continue,
            };
        }
    }
}
