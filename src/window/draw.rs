use super::color::Color;

#[derive(Debug, Clone)]
pub struct Rect {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

#[derive(Debug, Clone)]
pub struct Line {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Debug, Clone)]
pub struct LineInfo<C: Color> {
    width: u32,
    color: C,
}

#[derive(Debug, Clone)]
pub enum DrawCommand<C: Color> {
    FilledRect(Rect, C),
    RectOutline(Rect, LineInfo<C>),
    Line(Line, LineInfo<C>),
    Pixel(i32, i32, C),
    Chain(Vec<Self>),
}

impl<C: Color> DrawCommand<C> {
    pub const fn empty() -> Self {
        Self::Chain(Vec::new())
    }

    pub fn iter(&self) -> DrawIter<C> {
        match self {
            Self::Chain(ref vec) => DrawIter::Many(vec.iter()),
            s => DrawIter::Single(core::iter::once(&s)),
        }
    }
}

impl<C: Color> core::ops::Add for DrawCommand<C> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<C: Color> core::ops::AddAssign for DrawCommand<C> {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Chain(ref mut v1), Self::Chain(mut v2)) => v1.append(&mut v2),
            (Self::Chain(ref mut v), c) => v.push(c),
            (c, Self::Chain(mut v)) => {
                v.insert(0, c.clone());
                *c = Self::Chain(v);
            }
            (c1, c2) => *c1 = Self::Chain(vec![c1.clone(), c2]),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DrawIter<'a, C: Color> {
    Single(core::iter::Once<&'a DrawCommand<C>>),
    Many(core::slice::Iter<'a, DrawCommand<C>>),
}

impl<'a, C: Color> Iterator for DrawIter<'a, C> {
    type Item = &'a DrawCommand<C>;
    fn next(&mut self) -> Option<Self::Item> {
        let s: &mut dyn Iterator<Item = Self::Item> = match self {
            Self::Single(v) => v,
            Self::Many(v) => v,
        };
        s.next()
    }
}
