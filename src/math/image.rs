#![allow(unused_imports, unused_variables, dead_code)]
pub struct ScreenSize {
    pub width_mm: f64,
    pub height_mm: f64,
    pub width_px: u32,
    pub height_px: u32,
}

impl ScreenSize {
    pub fn laptop() -> ScreenSize {
        ScreenSize {
            width_mm: 340.0,
            height_mm: 210.0,
            width_px: 3456,
            height_px: 2160,
        }
    }

    pub fn monitor() -> ScreenSize {
        ScreenSize {
            width_mm: 595.0,
            height_mm: 340.0,
            width_px: 2560,
            height_px: 1440,
        }
    }
}
