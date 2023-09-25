pub use super::sp_texture_bank::SpTextureBank;
pub use super::classic_sprite::*;

pub struct SpResources<'a> {
    pub sp: Vec<ClassicSprite>,
    pub texture_bank: SpTextureBank<'a>,
    pub default_symmetry: SpSymmetry,
    pub pixel_scale: i32,
}

use super::bgsp_common::{RgbaImage, imageops};
use std::collections::BTreeMap;
impl<'a> SpResources<'a> {
    pub fn sp(&mut self, sp_no: usize) -> &mut ClassicSprite {
        &mut self.sp[sp_no]
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
            let symmetry = self.default_symmetry.compose(a_sp.symmetry);
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