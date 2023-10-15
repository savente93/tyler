#![allow(unused_imports, unused_variables, dead_code)]
// sorry about the file name, will think of something better later
pub fn mm_to_px(ppmm: f64, value: f64) -> u32 {
    (value / ppmm) as u32
}

pub fn px_to_mm(ppmm: f64, value: u32) -> f64 {
    value as f64 * ppmm
}
