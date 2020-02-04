use super::XError;
use super::super::{Display, WindowBuilder};
use super::Display as XDisplay;

pub struct Window<'a> {
    dis: &'a XDisplay,
}

impl<'a> super::super::Window<'a, XDisplay> for Window<'a> {
    type Error = XError;
    fn new(wb: WindowBuilder<'a, XDisplay>) -> Result<Self, Self::Error>
            where Self: Sized {
        let transparency = wb.get_transparency();
        let pos = (wb.get_x().unwrap_or(  0) as i16, wb.get_y().unwrap_or(  0) as i16);
        let size = (wb.get_w().unwrap_or(100) as u16, wb.get_h().unwrap_or(100) as u16);
        let screen_id = wb.get_screen();
        let dis = wb.get_display();

        let con = dis.con();
        let win = con.generate_id();
        let screen_id = screen_id.unwrap_or_else(|| dis.get_main_screen());
        let screen = dis.get_screen(screen_id)
                        .ok_or(XError::ScreenError(
                                String::from("could not find requested screen")))?;
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
            }
        }
        xcb::create_window(con,
                xcb::COPY_FROM_PARENT as u8,
                win,
                screen.root(),
                pos.0, pos.1, size.0, size.1,
                0,  // border width ( =? )
                xcb::xproto::WINDOW_CLASS_INPUT_OUTPUT as u16,
                screen.root_visual(), &[
                    // Defines how the window should be repositioned if the parent is resized
                    (xcb::CW_WIN_GRAVITY, xcb::GRAVITY_NORTH),
                ]
        ).request_check()?;
        Ok(Self {
            dis,
        })
    }
}
