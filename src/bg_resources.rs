pub use std::rc::Rc;
pub use std::cell::RefCell;

pub use super::bgsp_common::{
    PATTERN_SIZE, NUM_PALETTE_COL, PIXEL_SCALE_MAX,
    Rgba, RgbaImage, imageops,
    BgCode, BgPalette, BgSymmetry
};
use super::texture_bank;
pub type BgTextureBank<'a> = texture_bank::TextureBank<'a>;

const DIRTY_MARK: u32 = 0x1000_0000;

#[inline(always)]
fn u_mod(x: i32, p: i32) -> i32 {
    (x % p + p) % p
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CharAttributes {
    pub palette: BgPalette,
    pub symmetry: BgSymmetry,
}
impl CharAttributes {
    pub fn new(palette: BgPalette, symmetry: BgSymmetry) -> Self {
        Self {
            palette,
            symmetry
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct AChar {
    pub code: BgCode,
    pub palette: BgPalette,
    pub symmetry: BgSymmetry,
}
impl AChar {
    pub fn new<T: Into<BgCode>>(code: T, palette: BgPalette, symmetry: BgSymmetry) -> Self {
        Self {
            code: code.into(),
            palette,
            symmetry
        }
    }

    #[inline]
    pub fn force_dirty() -> Self {
        Self {
            code: DIRTY_MARK.into(),
            palette: DIRTY_MARK.into(),
            symmetry: BgSymmetry::non_default(),
        }
    }
}

pub struct BgResources<'a> {
    rect_size: (i32, i32),
    linear_size: i32,
    cur_buffer: Vec<AChar>,
    alt_buffer: Vec<AChar>,
    texture_bank: Rc<RefCell<&'a mut BgTextureBank<'a>>>,
    pixel_scale: i32,
    base_symmetry: BgSymmetry,
    rendered_image: RgbaImage,
}

const WIDTH_MAX: i32 = 8192;    // = 32 * 256 Characters
const HEIGHT_MAX: i32 = 8192;   // = 32 * 256 Characters

impl<'a> BgResources<'a> {

    pub fn with_base_symmetry(
        rect_size: (i32, i32),
        texture_bank: Rc<RefCell<&'a mut BgTextureBank<'a>>>,
        base_symmetry: BgSymmetry,
    ) -> Self {
        let width =
            if rect_size.0 > 0 {
                if rect_size.0 > WIDTH_MAX { WIDTH_MAX } else { rect_size.0 }
            } else { 1 };
        let height =
            if rect_size.1 > 0 {
                if rect_size.1 > HEIGHT_MAX { HEIGHT_MAX } else { rect_size.1 }
            } else { 1 };
        let linear_size = width * height;
        let cur_buffer = vec![AChar::default(); linear_size as usize];
        let alt_buffer = vec![AChar::force_dirty(); linear_size as usize];
        let pixel_scale = texture_bank.borrow().pixel_scale();
        let rendered_image =
            RgbaImage::new(
                (width * PATTERN_SIZE as i32 * pixel_scale) as u32,
                (height * PATTERN_SIZE as i32 * pixel_scale) as u32,
            );

        Self {
            rect_size: (width, height),
            linear_size,
            cur_buffer,
            alt_buffer,
            texture_bank,
            pixel_scale,
            base_symmetry,
            rendered_image,
        }
    }

    pub fn new(
        rect_size: (i32, i32),
        texture_bank: Rc<RefCell<&'a mut BgTextureBank<'a>>>,
    ) -> Self {
        let base_symmetry = BgSymmetry::default();
        Self::with_base_symmetry(rect_size, texture_bank, base_symmetry)
    }

    pub const fn width(&self) -> i32 {
        self.rect_size.0
    }

    pub const fn height(&self) -> i32 {
        self.rect_size.1
    }

    pub const fn rect_size(&self) -> (i32, i32) {
        self.rect_size
    }

    pub const fn linear_size(&self) -> i32 {
        self.linear_size
    }

    pub const fn pixel_scale(&self) -> i32 {
        self.pixel_scale
    }

    pub const fn base_symmetry(&self) -> BgSymmetry {
        self.base_symmetry
    }

    pub fn set_base_symmetry(&mut self, base_symmetry: BgSymmetry) -> &mut Self {
        self.base_symmetry = base_symmetry;
        self
    }

    #[inline(always)]
    fn _get_achar(&self, idx: i32) -> AChar {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffer[idx]
    }

    #[inline(always)]
    fn _set_achar(&mut self, idx: i32, achar: &AChar) -> &mut Self {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffer[idx] = *achar;
        self
    }

    pub fn get_achar(&self, idx: i32) -> AChar {
        self._get_achar(idx)
    }

    pub fn get_achar_at(&self, x: i32, y: i32) -> AChar {
        let idx = x + y * self.rect_size.0;
        self._get_achar(idx)
    }

    pub fn set_achar(&mut self, idx: i32, achar: &AChar) -> &mut Self {
        self._set_achar(idx, achar)
    }

    pub fn set_achar_n(&mut self, idx: i32, achar: &AChar, n: i32) -> &mut Self {
        for i in 0..n {
            self._set_achar(idx + i, achar);
        }
        self
    }

    pub fn set_achar_at(&mut self, x: i32, y: i32, achar: &AChar) -> &mut Self {
        let idx = x + y * self.rect_size.0;
        self._set_achar(idx, achar)
    }

    pub fn set_achar_n_at(&mut self, x: i32, y: i32, achar: &AChar, n: i32) -> &mut Self {
        let idx = x + y * self.rect_size.0;
        for i in 0..n {
            self._set_achar(idx + i, achar);
        }
        self
    }

    pub fn fill_achar(&mut self, achar: &AChar) -> &mut Self {
        for idx in 0..self.linear_size {
            self._set_achar(idx, achar);
        }
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        let achar: AChar = Default::default();
        self.fill_achar(&achar)
    }

    #[inline(always)]
    fn _get_attributes(&self, idx: i32) -> CharAttributes {
        let idx = u_mod(idx, self.linear_size) as usize;
        CharAttributes {
            palette: self.cur_buffer[idx].palette,
            symmetry: self.cur_buffer[idx].symmetry,
        }
    }

    #[inline(always)]
    fn _set_attributes(&mut self, idx: i32, attributes: &CharAttributes) -> &mut Self {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffer[idx].palette = attributes.palette;
        self.cur_buffer[idx].symmetry = attributes.symmetry;
        self
    }

    pub fn get_attributes(&self, idx: i32) -> CharAttributes {
        self._get_attributes(idx)
    }

    pub fn get_attributes_at(&self, x: i32, y: i32) -> CharAttributes {
        let idx = x + y * self.rect_size.0;
        self._get_attributes(idx)
    }

    pub fn set_attributes(&mut self, idx: i32, attributes: &CharAttributes) -> &mut Self {
        self._set_attributes(idx, attributes)
    }

    pub fn set_attributes_n(&mut self, idx: i32, attributes: &CharAttributes, n: i32) -> &mut Self {
        for i in 0..n {
            self._set_attributes(idx + i, attributes);
        }
        self
    }

    pub fn set_attributes_at(&mut self, x: i32, y: i32, attributes: &CharAttributes) -> &mut Self {
        let idx = x + y * self.rect_size.0;
        self._set_attributes(idx, attributes)
    }

    pub fn set_attributes_n_at(&mut self, x: i32, y: i32, attributes: &CharAttributes, n: i32) -> &mut Self {
        let idx = x + y * self.rect_size.0;
        for i in 0..n {
            self._set_attributes(idx + i, attributes);
        }
        self
    }

    pub fn fill_attributes(&mut self, attributes: &CharAttributes) -> &mut Self {
        for idx in 0..self.linear_size {
            self._set_attributes(idx, attributes);
        }
        self
    }

    #[inline(always)]
    fn _get_code(&self, idx: i32) -> BgCode {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffer[idx].code
    }

    #[inline(always)]
    fn _set_code(&mut self, idx: i32, code: BgCode) -> &mut Self {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffer[idx].code = code;
        self
    }

    pub fn get_code(&self, idx: i32) -> BgCode {
        self._get_code(idx)
    }

    pub fn get_code_at(&self, x: i32, y: i32) -> BgCode {
        let idx = x + y * self.rect_size.0;
        self._get_code(idx)
    }

    pub fn set_code<T: Into<BgCode>>(&mut self, idx: i32, code: T) -> &mut Self {
        self._set_code(idx, code.into())
    }

    pub fn set_code_n<T: Into<BgCode>>(&mut self, idx: i32, code: T, n: i32) -> &mut Self {
        let code = code.into();
        for i in 0..n {
            self._set_code(idx + i, code);
        }
        self
    }

    pub fn set_code_at<T: Into<BgCode>>(&mut self, x: i32, y: i32, code: T) -> &mut Self {
        let idx = x + y * self.rect_size.0;
        self._set_code(idx, code.into())
    }

    pub fn set_code_n_at<T: Into<BgCode>>(&mut self, x: i32, y: i32, code: T, n: i32) -> &mut Self {
        let idx = x + y * self.rect_size.0;
        let code = code.into();
        for i in 0..n {
            self._set_code(idx + i, code);
        }
        self
    }

    pub fn fill_code<T: Into<BgCode>>(&mut self, code: T) -> &mut Self {
        let code = code.into();
        for idx in 0..self.linear_size {
            self._set_code(idx, code);
        }
        self
    }

    #[inline(always)]
    fn _get_palette(&self, idx: i32) -> BgPalette {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffer[idx].palette
    }

    #[inline(always)]
    fn _set_palette(&mut self, idx: i32, palette: BgPalette) -> &mut Self {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffer[idx].palette = palette;
        self
    }

    pub fn get_palette(&self, idx: i32) -> BgPalette {
        self._get_palette(idx)
    }

    pub fn get_palette_at(&self, x: i32, y: i32) -> BgPalette {
        let idx = x + y * self.rect_size.0;
        self._get_palette(idx)
    }

    pub fn set_palette(&mut self, idx: i32, palette: BgPalette) -> &mut Self {
        self._set_palette(idx, palette)
    }

    pub fn set_palette_n(&mut self, idx: i32, palette: BgPalette, n: i32) -> &mut Self {
        for i in 0..n {
            self._set_palette(idx + i, palette);
        }
        self
    }

    pub fn set_palette_at(&mut self, x: i32, y: i32, palette: BgPalette) -> &mut Self {
        let idx = x + y * self.rect_size.0;
        self._set_palette(idx, palette)
    }

    pub fn set_palette_n_at(&mut self, x: i32, y: i32, palette: BgPalette, n: i32) -> &mut Self {
        let idx = x + y * self.rect_size.0;
        for i in 0..n {
            self._set_palette(idx + i, palette);
        }
        self
    }

    pub fn fill_palette(&mut self, palette: BgPalette) -> &mut Self {
        for idx in 0..self.linear_size {
            self._set_palette(idx, palette);
        }
        self
    }

    #[inline(always)]
    fn _get_symmetry(&self, idx: i32) -> BgSymmetry {
        let idx = u_mod(idx ,self.linear_size) as usize;
        self.cur_buffer[idx].symmetry
    }

    #[inline(always)]
    fn _set_symmetry(&mut self, idx: i32, symmetry: BgSymmetry) -> &mut Self {
        let idx = u_mod(idx ,self.linear_size) as usize;
        self.cur_buffer[idx].symmetry = symmetry;
        self
    }

    pub fn get_symmetry(&self, idx: i32) -> BgSymmetry {
        self._get_symmetry(idx)
    }

    pub fn get_symmetry_at(&self, x: i32, y: i32) -> BgSymmetry {
        let idx = x + y * self.rect_size.0;
        self._get_symmetry(idx)
    }

    pub fn set_symmetry(&mut self, idx: i32, symmetry: BgSymmetry) -> &mut Self {
        self._set_symmetry(idx, symmetry)
    }

    pub fn set_symmetry_n(&mut self, idx: i32, symmetry: BgSymmetry, n: i32) -> &mut Self {
        for i in 0..n {
            self._set_symmetry(idx + i, symmetry);
        }
        self
    }

    pub fn set_symmetry_at(&mut self, x: i32, y: i32, symmetry: BgSymmetry) -> &mut Self {
        let idx = x + y * self.rect_size.0;
        self._set_symmetry(idx, symmetry)
    }

    pub fn set_symmetry_n_at(&mut self, x: i32, y: i32, symmetry: BgSymmetry, n: i32) -> &mut Self {
        let idx = x + y * self.rect_size.0;
        for i in 0..n {
            self._set_symmetry(idx + i, symmetry);
        }
        self
    }

    pub fn fill_symmetry(&mut self, symmetry: BgSymmetry) -> &mut Self {
        for idx in 0..self.linear_size {
            self._set_symmetry(idx, symmetry);
        }
        self
    }

    pub fn rendering(&mut self) -> i32 {
        let mut done = 0;
        let mut idx = 0;
        for y in 0..self.rect_size.1 {
            for x in 0..self.rect_size.0 {
                if self.cur_buffer[idx] != self.alt_buffer[idx] {
                    self.alt_buffer[idx] = self.cur_buffer[idx];
                    // rendering proc
                    if let Some(t) = self.texture_bank.borrow_mut().texture(
                        self.cur_buffer[idx].code,
                        self.cur_buffer[idx].palette,
                        self.cur_buffer[idx].symmetry.compose(self.base_symmetry),
                    ) {
                        imageops::replace(
                            &mut self.rendered_image,
                            &*t,
                            (x * PATTERN_SIZE as i32 * self.pixel_scale) as i64,
                            (y * PATTERN_SIZE as i32 * self.pixel_scale) as i64,
                        );
                    }
                    done += 1;
                }
                idx += 1;
            }
        }
        done
    }

    pub fn rendered_image(&self) -> &RgbaImage {
        &self.rendered_image
    }
}
