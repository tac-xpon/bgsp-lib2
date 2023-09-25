#![allow(clippy::too_many_arguments)]
use super::bgsp_common::{self, PATTERN_SIZE, NUM_PALETTE_TBL, NUM_PALETTE_COL, Rgba, RgbaImage, BgCode, BgPalette, BgSymmetry};

pub fn drawchar(
    char_data: &[[u64; PATTERN_SIZE]],
    pal_data: &[[Rgba<u8>; NUM_PALETTE_COL]],
    code: BgCode,
    palette: BgPalette,
    symmetry: BgSymmetry,
    position: (u32, u32),
    scalar: (u32, u32),
    gbuf: &mut RgbaImage,
) {
    bgsp_common::draw(
        (1, 1),
        &char_data[code as usize],
        &pal_data[palette as usize % NUM_PALETTE_TBL],
        symmetry,
        position,
        scalar,
        gbuf,
    );
}