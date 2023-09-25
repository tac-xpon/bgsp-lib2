use std::collections::BTreeMap;
use std::rc::Rc;

use super::bgsp_common::{PATTERN_SIZE, NUM_PALETTE_COL, Rgba, RgbaImage, SpCode, SpPalette, SpSymmetry};
use super::sp_lib;

type SpTexture = RgbaImage;
type RcSpTexture = Rc<SpTexture>;

pub struct SpTextureBank<'a> {
    pattern_tbl: &'a [Option<(u32, u32, &'a [u64])>],
    palette_tbl: &'a [[Rgba<u8>; NUM_PALETTE_COL]],
    pixel_scale: i32,
    texture_cache: BTreeMap<(SpCode, SpPalette, SpSymmetry), RcSpTexture>,
}

impl<'a> SpTextureBank<'a> {
    pub fn new(pattern_tbl: &'a [Option<(u32, u32, &'a [u64])>], palette_tbl: &'a [[Rgba<u8>; NUM_PALETTE_COL]], pixel_scale: i32) -> Self {
        Self {
            pattern_tbl,
            palette_tbl,
            pixel_scale,
            texture_cache: BTreeMap::new(),
        }
    }

    pub const fn pixel_scale(&self) -> i32 {
        self.pixel_scale
    }

    pub fn clear_cache(&mut self) {
        self.texture_cache.clear()
    }

    pub fn cashed_num(&self) -> usize {
        self.texture_cache.len()
    }

    pub fn texture(&mut self, pattern_no: SpCode, palette_no: SpPalette, symmetry: SpSymmetry) -> Option<RcSpTexture> {
        if let Some(result) = self.texture_cache.get(&(pattern_no, palette_no, symmetry)) {
            Some(result.clone())
        } else {
            if let Some(pattern_info) = self.pattern_tbl[pattern_no as usize] {
                let scale = self.pixel_scale as u32;
                let size = if !symmetry.has_rotate90() {
                    (pattern_info.0, pattern_info.1)
                } else {
                    (pattern_info.1, pattern_info.0)
                };
                if size.0 > 0 && size.1 > 0 {
                    let mut buffer = SpTexture::new(size.0 * PATTERN_SIZE as u32 * scale, size.1 * PATTERN_SIZE as u32 * scale);
                    sp_lib::drawsp((pattern_info.0, pattern_info.1), pattern_info.2, &self.palette_tbl[palette_no as usize], symmetry, (0, 0), (scale, scale), &mut buffer);
                    let rc_texture = Rc::new(buffer);
                    let _ = self.texture_cache.insert((pattern_no, palette_no, symmetry), rc_texture.clone());
                    Some(rc_texture)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
