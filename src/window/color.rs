/// The PixelFormat describes how a pixel is stored in memory
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PixelFormat {
    /// red, green, blue, 8 bits each
    Rgb24,
    /// red, green, blue, alpha, 8 bits each
    Rgba32,
    /// alpha, red, green, blue, 8 bits each
    Argb32,
}

impl PixelFormat {
    pub const fn bits(&self) -> usize {
        match self {
            PixelFormat::Rgb24 => 24,
            PixelFormat::Rgba32 => 32,
            PixelFormat::Argb32 => 32,
        }
    }

    pub const fn alpha_bits(&self) -> usize {
        match self {
            PixelFormat::Rgb24 => 0,
            PixelFormat::Rgba32 => 8,
            PixelFormat::Argb32 => 8,
        }
    }

    pub const fn has_alpha(&self) -> bool {
        match self {
            PixelFormat::Rgb24 => false,
            PixelFormat::Rgba32 | PixelFormat::Argb32 => true,
        }
    }
}

pub trait Color: Clone + Sized + Default {
    fn get_format() -> PixelFormat;
    fn r8(&self) -> u8;
    fn g8(&self) -> u8;
    fn b8(&self) -> u8;
    fn a8(&self) -> u8;
    fn as_rgb24(self) -> ColorRgb24 {
        ColorRgb24 {
            r: self.r8(),
            g: self.g8(),
            b: self.b8(),
        }
    }
    fn as_rgba32(self) -> ColorRgba32 {
        ColorRgba32 {
            r: self.r8(),
            g: self.g8(),
            b: self.b8(),
            a: self.a8(),
        }
    }
}

/// A [`Color`] of [`PixelFormat::Rgba32`]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ColorRgba32 {
    /// red channel
    pub r: u8,
    /// green channel
    pub g: u8,
    /// blue channel
    pub b: u8,
    /// alpha channel
    pub a: u8,
}

impl Color for ColorRgba32 {
    fn get_format() -> PixelFormat {
        PixelFormat::Rgba32
    }
    fn r8(&self) -> u8 {
        self.r
    }
    fn g8(&self) -> u8 {
        self.g
    }
    fn b8(&self) -> u8 {
        self.b
    }
    fn a8(&self) -> u8 {
        self.a
    }
}

/// A [`Color`] of [`PixelFormat::Rgb24`]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ColorRgb24 {
    /// red channel
    pub r: u8,
    /// green channel
    pub g: u8,
    /// blue channel
    pub b: u8,
}

impl Color for ColorRgb24 {
    fn get_format() -> PixelFormat {
        PixelFormat::Rgb24
    }
    fn r8(&self) -> u8 {
        self.r
    }
    fn g8(&self) -> u8 {
        self.g
    }
    fn b8(&self) -> u8 {
        self.b
    }
    fn a8(&self) -> u8 {
        255
    }
}
