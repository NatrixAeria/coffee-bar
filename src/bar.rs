use crate::window::{Display, Window, WindowType};
use crate::BarError;
use std::pin::Pin;

pub struct Bar<D: Display, W: Window<'static, D>> {
    dis: Pin<Box<D>>,
    win: W,
    _pin: std::marker::PhantomPinned,
}

impl<D: Display + 'static, W: Window<'static, D>> Bar<D, W> {
    pub fn new() -> Result<Self, BarError> {
        let dis = D::new().map_err(BarError::from_dis)?;

        let size = dis
            .get_screen_dimension(dis.get_main_screen())
            .ok_or_else(|| BarError(String::from("No screen available")))?;

        let dis = Box::new(dis);
        let disref = unsafe { core::mem::transmute::<&'_ D, &'static D>(&dis) };
        let win: W = disref
            .new_window_builder()
            .title(String::from("coffee bar"))
            .pos(0, 0)
            .size(size.0, 40)
            .transparency(true)
            .window_type(WindowType::Docking)
            .build()
            .map_err(BarError::from_dis)?;

        Ok(Self {
            dis: dis.into(),
            win,
            _pin: std::marker::PhantomPinned,
        })
    }

    pub fn main_loop(self) -> Result<(), BarError> {
        for event in self.win {
            let event = match event {
                Ok(v) => v,
                Err(e) => {
                    println!("warning: {}", e);
                    continue;
                }
            };
            println!("{:?}", event);
        }
        Ok(())
    }
}

pub type X11Bar = Bar<crate::window::xwindow::Display, crate::window::xwindow::Window<'static>>;
