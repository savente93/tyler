#![allow(unused_imports, unused_variables, dead_code)]

pub fn calc_num_tiles(total_dim: u32, sub_dim: u32) -> u32 {
    num::integer::div_ceil(total_dim, sub_dim)
}

// will remove this warning slience when I refactor this
#[allow(clippy::too_many_arguments)]
pub fn calc_current_tile_bounds(
    total_width_px: u32,
    total_height_px: u32,
    base_height_px: u32,
    base_width_px: u32,
    overlap_horz_px: u32,
    overlap_vert_px: u32,
    horz_idx: u32,
    vert_idx: u32,
) -> (u32, u32, u32, u32) {
    let pannel_h_start = (horz_idx * base_width_px).saturating_sub(overlap_horz_px);

    let pannel_v_start = (vert_idx * base_height_px).saturating_sub(overlap_vert_px);

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
