pub use super::classic_sprite::*;
use super::texture_bank;
pub type SpTextureBank<'a> = texture_bank::TextureBank<'a>;

pub struct SpResources<'a> {
    pub sp: Vec<ClassicSprite>,
    pub texture_bank: SpTextureBank<'a>,
    pub pixel_scale: i32,
    pub base_symmetry: SpSymmetry,
}

use super::bgsp_common::{
    Rgba, RgbaImage, imageops,
    NUM_PALETTE_COL, PIXEL_SCALE_MAX,
};
use std::collections::BTreeMap;
impl<'a> SpResources<'a> {

    pub fn with_base_symmetry(
        max_sprites: usize,
        pattern_tbl: &'a [Option<(u32, u32, &'a [u64])>],
        palette_tbl: &'a [[Rgba<u8>; NUM_PALETTE_COL]],
        pixel_scale: i32,
        base_symmetry: SpSymmetry,
    ) -> Self {
        let mut sp: Vec<ClassicSprite> = Vec::with_capacity(max_sprites);
        for _ in 0..max_sprites {
            sp.push(ClassicSprite { ..Default::default()});
        }
        let texture_bank = SpTextureBank::new(
            pattern_tbl,
            palette_tbl,
            pixel_scale,
        );
        let pixel_scale = if pixel_scale > 0 {
            if pixel_scale > PIXEL_SCALE_MAX { PIXEL_SCALE_MAX } else { pixel_scale }
        } else { 1 };
        Self {
            sp,
            texture_bank,
            pixel_scale,
            base_symmetry,
        }
    }

    pub fn new(
        num_sprites: usize,
        pattern_tbl: &'a [Option<(u32, u32, &'a [u64])>],
        palette_tbl: &'a [[Rgba<u8>; NUM_PALETTE_COL]],
        pixel_scale: i32,
    ) -> Self {
        let base_symmetry = SpSymmetry::default();
        Self::with_base_symmetry(num_sprites, pattern_tbl, palette_tbl, pixel_scale, base_symmetry)
    }

    pub fn sp(&mut self, sp_no: usize) -> &mut ClassicSprite {
        &mut self.sp[sp_no]
    }

    pub fn max_sprites(&self) -> usize {
        self.sp.len()
    }

    pub const fn pixel_scale(&self) -> i32 {
        self.pixel_scale
    }

    pub const fn base_symmetry(&self) -> SpSymmetry {
        self.base_symmetry
    }

    pub fn set_base_symmetry(&mut self, base_symmetry: SpSymmetry) {
        self.base_symmetry = base_symmetry;
    }

    pub fn rendering(&mut self, view_w: i32, view_h: i32) -> RgbaImage {
        let mut priority_map = BTreeMap::new();
        let mut image_buffer = RgbaImage::new((view_w * self.pixel_scale) as u32, (view_h * self.pixel_scale) as u32);
        for (idx, a_sp) in self.sp.iter().enumerate() {
            priority_map.insert((a_sp.priority << 12) + idx as i32, idx);
        }
        for (_priority, idx) in priority_map.iter().rev() {
            let a_sp = &self.sp[*idx];
            if !a_sp.visible
            || a_sp.pos.x < -72 || a_sp.pos.x >= view_w + 8
            || a_sp.pos.y < -72 || a_sp.pos.y >= view_h + 8 {
                continue;
            }
            let symmetry = self.base_symmetry.compose(a_sp.symmetry);
            if let Some(t) = self.texture_bank.texture(a_sp.code, a_sp.palette, symmetry) {
                imageops::overlay(
                    &mut image_buffer,
                    &*t,
                    (a_sp.pos.x * self.pixel_scale as i32) as i64,
                    (a_sp.pos.y * self.pixel_scale as i32) as i64,
                );
            }
        }
        image_buffer
    }
}