use super::XError;
use super::super::{Display, WindowBuilder, event::Event, WindowType};
use super::Display as XDisplay;

pub struct Window<'a> {
    dis: &'a XDisplay,
    win: xcb::Window,
    transparency: bool,
}

impl<'a> Window<'a> {
    fn create_pixmap(&self, w: u64, h: u64) -> Result<xcb::Pixmap, XError> {
        let con = self.dis.con();
        let pix = con.generate_id();
        let depth = if self.transparency { 24 } else { 32 };
        xcb::create_pixmap(con, depth, pix, self.win, w as u16, h as u16)
            .request_check()?;
        Ok(pix)
    }
}

impl<'a> super::super::Window<'a, XDisplay> for Window<'a> {
    type Error = XError;
    fn new(wb: WindowBuilder<'a, XDisplay>) -> Result<Self, Self::Error>
            where Self: Sized {
        let transparency = wb.get_transparency();
        let pos = (wb.get_x().unwrap_or(  0) as i16, wb.get_y().unwrap_or(  0) as i16);
        let size = (wb.get_w().unwrap_or(100) as u16, wb.get_h().unwrap_or(100) as u16);
        let screen_id = wb.get_screen();
        let window_type = wb.get_window_type();
        let dis = wb.get_display();

        let con = dis.con();
        let win = con.generate_id();
        let screen_id = screen_id.unwrap_or_else(|| dis.get_main_screen());
        let screen = dis.get_screen(screen_id)
                        .ok_or(XError::ScreenError(
                                String::from("could not find requested screen")))?;
        let mut visual = None;
        let mut depth_val = xcb::COPY_FROM_PARENT as u8;
        let mut cw_values = vec![];

        let foreground_gc = con.generate_id();
        xcb::create_gc(con, foreground_gc, screen.root(), &[
            (xcb::GC_FOREGROUND, screen.black_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
        ]);

        if transparency {
            if let Some((depth, vis)) = dis.get_depth(
                                screen_id, 32, xcb::VISUAL_CLASS_TRUE_COLOR as u8) {
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
                         vis.visual_id()
                ).request_check()?;
                visual = Some(vis.visual_id());
                cw_values.push((xcb::CW_COLORMAP, colormap));
                cw_values.push((xcb::CW_BORDER_PIXEL, screen.black_pixel()));
                depth_val = depth.depth();
            }
        }
        // Defines how the window should be repositioned if the parent is resized
        cw_values.push((xcb::CW_WIN_GRAVITY, xcb::GRAVITY_NORTH));
        cw_values.push((xcb::CW_BACK_PIXEL, screen.black_pixel()));
        cw_values.push((xcb::CW_EVENT_MASK,
             xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS));
        xcb::create_window(con,
                depth_val,
                win,
                screen.root(),
                pos.0, pos.1, size.0, size.1,
                0,  // border width
                xcb::xproto::WINDOW_CLASS_INPUT_OUTPUT as u16,
                visual.unwrap_or_else(|| screen.root_visual()),
                &cw_values
        ).request_check()?;

        match window_type {
            WindowType::Normal => (),
            WindowType::Docking => {
                let window_type = dis.get_intern_atom("_NET_WM_WINDOW_TYPE").unwrap().unwrap();
                let docking = dis.get_intern_atom("_NET_WM_WINDOW_TYPE_DOCK").unwrap().unwrap();
                xcb::change_property(
                    con,
                    xcb::PROP_MODE_REPLACE as u8,
                    win,
                    window_type,
                    xcb::ATOM as u32,
                    // Specifies whether the data should be viewed as a
                    // list of 8-bit, 16-bit, or 32-bit quantities
                    32,
                    &[docking]
                ).request_check()?;
            }
        }

        xcb::map_window(con, win).request_check()?;
        con.flush();

        Ok(Self {
            dis,
            transparency,
        })
    }
}

impl<'a> Iterator for Window<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        let con = self.dis.con();
        con.wait_for_event().and_then(|event| {
            let r = event.response_type() & !0x80;
            match r {
                xcb::EXPOSE => Some(Event::Redraw),
                xcb::KEY_PRESS => Some(Event::KeyDown),
                _ => None,
            }
        })
    }
}
