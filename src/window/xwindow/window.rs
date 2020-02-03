use super::XError;
use super::super::{Display, WindowBuilder};

pub struct Window<'a, D: Display> {
    dis: &'a mut D,
}

impl<'a, D: Display> super::super::Window<'a, D> for Window<'a, D> {
    type Error = XError;
    fn new(wb: WindowBuilder<'a, D>) -> Result<Self, Self::Error>
            where Self: Sized {
        let dis = wb.get_display();
        Ok(Self {
            dis,
        })
    }
}
