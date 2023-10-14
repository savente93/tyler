#![allow(unused_imports, unused_variables, dead_code)]

struct TestCase<'a> {
    img_path: &'a str,
    length_mm: f64,
    width_mm: f64,
    printed_length_mm: f64,
    printed_width_mm: f64,
    target_width_mm: f64,
    target_height_mm: f64,
}

impl<'a> TestCase<'a> {
    pub fn leviathan() -> TestCase<'a> {
        // leviathan length to head ratio seems very consistently 2.75
        TestCase {
            img_path: "testing_assets/leviathan.png",
            length_mm: 919.0,
            width_mm: 334.18, //using sneaky maths, a ruler, and promotional material
            printed_length_mm: 92.0,
            printed_width_mm: 260.0,
            target_width_mm: 812.8,
            target_height_mm: 295.56,
        }
    }

    pub fn ruler() -> TestCase<'a> {
        TestCase {
            img_path: "testing_assets/ruler.jpg",
            length_mm: 190.0,
            width_mm: 710.0,
            printed_length_mm: 70.0,
            printed_width_mm: 240.0,
            target_width_mm: 710.0,
            target_height_mm: 190.0,
        }
    }
}
