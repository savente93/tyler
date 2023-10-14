#![allow(unused_imports, unused_variables, dead_code)]

use clap::Parser;
mod cli;
mod math;

use crate::cli::Args;
use image::{imageops, ImageFormat, SubImage};
use math::image::ScreenSize;
use math::paper::PaperSize;
use math::tiles::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let args = Args::parse();
    let length_mm = 190.0;
    let width_mm = 710.0;
    let printed_length_mm = 70.0;
    let printed_width_mm = 240.0;
    let target_width_mm = 710.0;
    let target_height_mm = 190.0;
    let img = match image::open("testing_assets/ruler.jpg") {
        Ok(img) => img,
        Err(e) => {
            println!("Error: {} \nCheck if the image file is actually an image or a link to a file in git-lfs", e);
            return;
        }
    };

    let screen_size = ScreenSize::monitor();
    let paper_size = PaperSize::a4();

    let ppmm = screen_size.width_px as f64 / screen_size.width_mm;
    let scale_factor = target_width_mm / width_mm;

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
