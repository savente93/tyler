#![allow(unused_imports, unused_variables, dead_code)]

pub struct PaperSize {
    pub width_mm: f64,
    pub height_mm: f64,
}

impl PaperSize {
    pub fn a4() -> PaperSize {
        PaperSize {
            width_mm: 297.0,
            height_mm: 210.0,
        }
    }
}
