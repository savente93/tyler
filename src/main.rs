#![allow(unused_imports, unused_variables, dead_code)]
use clap::Parser;
use image::{imageops, ImageFormat, SubImage};
use std::fs::File;
use std::io::BufWriter;

// static LAPTOP_SCREEN_PPI_SQ: u32 = 64888;
// static PRINTER_DPI: u32 = 1200;

type PixelHeight = u32;
type PixelWidth = u32;
type PixelCoordX = u32;
type PixelCoordY = u32;

// reference link: https://www.youtube.com/watch?v=iuNjDzT6PWo
// should preserve aspect ratio, even under rotation.

struct TestCase<'a> {
    img_path: &'a str,
    length_mm: f64,
    width_mm: f64,
    printed_length_mm: f64,
    printed_width_mm: f64,
    target_width_mm: f64,
    target_height_mm: f64,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None,)]
// add `disable_help_flag = true` to command so that `horz_overlap_mm` and `help` do not have the same short flag
struct Args {
    /// Name of the person to greet
    #[arg(long, default_value_t = 20.0)]
    horz_overlap_mm: f64,

    #[arg(short, long, default_value_t = 20.0)]
    vert_overlap_mm: f64,
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

struct PaperSize {
    width_mm: f64,
    height_mm: f64,
}

struct ScreenSize {
    width_mm: f64,
    height_mm: f64,
    width_px: u32,
    height_px: u32,
}

impl PaperSize {
    pub fn a4() -> PaperSize {
        PaperSize {
            width_mm: 297.0,
            height_mm: 210.0,
        }
    }
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

fn main() {
    let args = Args::parse();
    let test_case = TestCase::ruler();
    let img = match image::open(test_case.img_path) {
        Ok(img) => img,
        Err(e) => {
            println!("Error: {} \nCheck if the image file is actually an image or a link to a file in git-lfs", e);
            return;
        }
    };

    let screen_size = ScreenSize::monitor();
    let paper_size = PaperSize::a4();

    let ppmm = screen_size.width_px as f64 / screen_size.width_mm;
    let scale_factor = test_case.target_width_mm / test_case.width_mm;

    let horz_overlap_px = mm_to_px(ppmm, args.horz_overlap_mm);
    let vert_overlap_px = mm_to_px(ppmm, args.vert_overlap_mm);

    let paper_width_px = mm_to_px(ppmm, paper_size.width_mm);
    let paper_height_px = mm_to_px(ppmm, paper_size.height_mm);

    let target_width_px = mm_to_px(ppmm, screen_size.width_mm * scale_factor);
    let target_height_px = mm_to_px(ppmm, screen_size.height_mm * scale_factor);

    let num_horz_tiles = calc_num_tiles(img.width(), paper_width_px - horz_overlap_px * 2);
    let num_vert_tiles = calc_num_tiles(img.height(), paper_height_px - vert_overlap_px * 2);

    let total_width_px = img.width();
    let total_height_px = img.height();

    // let scaled_img = imageops::resize(
    //     &img,
    //     target_width_px,
    //     target_height_px,
    //     imageops::FilterType::Lanczos3,
    // );
    let scaled_img = img.clone();

    println!("tyling");
    let mut sub_pannels = Vec::new();

    for v in 0..num_vert_tiles {
        for h in 0..num_horz_tiles {
            let (pannel_h_start, pannel_v_start, current_pannel_width, current_pannel_height) =
                calc_current_tile_bounds(
                    total_width_px,
                    total_height_px,
                    paper_height_px,
                    paper_width_px,
                    horz_overlap_px,
                    vert_overlap_px,
                    h,
                    v,
                );

            let sub_pannel = SubImage::new(
                &scaled_img,
                pannel_h_start,
                pannel_v_start,
                current_pannel_width,
                current_pannel_height,
            );

            sub_pannels.push(sub_pannel);
        }
    }

    println!("writing tyles");
    sub_pannels.iter().enumerate().for_each(|(idx, tile)| {
        let mut output = File::create(format!("testing_assets/out{}.png", idx)).unwrap();

        tile.to_image()
            .write_to(&mut output, ImageFormat::Png)
            .unwrap();
    })
}

fn mm_to_px(ppmm: f64, value: f64) -> u32 {
    (value / ppmm) as u32
}

fn px_to_mm(ppmm: f64, value: u32) -> f64 {
    value as f64 * ppmm
}

fn calc_num_tiles(total_dim: u32, sub_dim: u32) -> u32 {
    num::integer::div_ceil(total_dim, sub_dim)
}

fn calc_current_tile_bounds(
    total_width_px: u32,
    total_height_px: u32,
    base_height_px: u32,
    base_width_px: u32,
    overlap_horz_px: u32,
    overlap_vert_px: u32,
    horz_idx: u32,
    vert_idx: u32,
) -> (PixelCoordX, PixelCoordY, PixelWidth, PixelHeight) {
    let pannel_h_start = (horz_idx * base_width_px)
        .checked_sub(overlap_horz_px)
        .unwrap_or(0);

    let pannel_v_start = (vert_idx * base_height_px)
        .checked_sub(overlap_vert_px)
        .unwrap_or(0);

    let current_tile_width = if pannel_h_start != 0
        && (pannel_h_start - overlap_horz_px) + base_width_px > total_width_px
    {
        total_width_px - pannel_h_start - overlap_horz_px
    } else {
        base_width_px
    };

    let current_tile_height = if pannel_v_start != 0
        && (pannel_v_start - overlap_vert_px) + base_height_px > total_height_px
    {
        total_height_px - pannel_v_start - overlap_vert_px
    } else {
        base_height_px
    };

    assert!(current_tile_height != 0);
    assert!(current_tile_width != 0);

    (
        pannel_h_start,
        pannel_v_start,
        current_tile_width,
        current_tile_height,
    )
}
