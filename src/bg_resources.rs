pub use super::bgsp_common::{PATTERN_SIZE, NUM_PALETTE_COL, Rgba, RgbaImage, BgCode, BgPalette, BgSymmetry};
use super::bg_lib;

const DIRTY_MARK: u32 = 0x1000_0000;

#[inline(always)]
fn u_mod(x: i32, p: i32) -> i32 {
    (x % p + p) % p
}

#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
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
}

#[derive(Default, Clone)]
struct Buffers {
    code: Vec<BgCode>,
    palette: Vec<BgPalette>,
    symmetry: Vec<BgSymmetry>,
}

#[derive(Clone)]
pub struct BgResources<'a> {
    rect_size: (i32, i32),
    linear_size: i32,
    cur_buffers: Buffers,
    alt_buffers: Buffers,
    char_data: &'a [[u64; PATTERN_SIZE]],
    pal_data: &'a [[Rgba<u8>; NUM_PALETTE_COL]],
    pixel_scale: i32,
    base_symmetry: BgSymmetry,
    rendered_image: RgbaImage,
}

const WIDTH_MAX: i32 = 8192;    // = 32 * 256 Characters
const HEIGHT_MAX: i32 = 8192;   // = 32 * 256 Characters
const PIXEL_SCALE_MAX: i32 = 4;

impl<'a> BgResources<'a> {

    pub fn with_base_symmetry(
        rect_size: (i32, i32),
        char_data: &'a [[u64; PATTERN_SIZE]],
        pal_data: &'a [[Rgba<u8>; NUM_PALETTE_COL]],
        pixel_scale: i32,
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
        let cur_buffers = Buffers {
            code: vec![BgCode::default(); linear_size as usize],
            palette: vec![BgPalette::default(); linear_size as usize],
            symmetry: vec![BgSymmetry::default(); linear_size as usize],
        };
        let alt_buffers = Buffers {
            code: vec![DIRTY_MARK; linear_size as usize],
            palette: vec![DIRTY_MARK; linear_size as usize],
            symmetry: vec![BgSymmetry::non_default(); linear_size as usize],
        };
        let pixel_scale =
            if pixel_scale > 0 {
                if pixel_scale > PIXEL_SCALE_MAX { PIXEL_SCALE_MAX } else { pixel_scale }
            } else { 1 };
        let rendered_image =
            RgbaImage::new(
                (width * PATTERN_SIZE as i32 * pixel_scale) as u32,
                (height * PATTERN_SIZE as i32 * pixel_scale) as u32,
            );

        Self {
            rect_size: (width, height),
            linear_size,
            cur_buffers,
            alt_buffers,
            char_data,
            pal_data,
            pixel_scale,
            base_symmetry,
            rendered_image,
        }
    }

    pub fn new(
        rect_size: (i32, i32),
        char_data: &'a [[u64; PATTERN_SIZE]],
        pal_data: &'a [[Rgba<u8>; NUM_PALETTE_COL]],
        pixel_scale: i32,
    ) -> Self {
        Self::with_base_symmetry(rect_size, char_data, pal_data, pixel_scale, BgSymmetry::default())
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
        AChar {
            code: self.cur_buffers.code[idx],
            palette: self.cur_buffers.palette[idx],
            symmetry: self.cur_buffers.symmetry[idx],
        }
    }

    #[inline(always)]
    fn _set_achar(&mut self, idx: i32, achar: &AChar) -> &mut Self {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffers.code[idx] = achar.code;
        self.cur_buffers.palette[idx] = achar.palette;
        self.cur_buffers.symmetry[idx] = achar.symmetry;
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
            palette: self.cur_buffers.palette[idx],
            symmetry: self.cur_buffers.symmetry[idx],
        }
    }

    #[inline(always)]
    fn _set_attributes(&mut self, idx: i32, attributes: &CharAttributes) -> &mut Self {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffers.palette[idx] = attributes.palette;
        self.cur_buffers.symmetry[idx] = attributes.symmetry;
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
        self.cur_buffers.code[idx]
    }

    #[inline(always)]
    fn _set_code(&mut self, idx: i32, code: BgCode) -> &mut Self {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffers.code[idx] = code;
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
        self.cur_buffers.palette[idx]
    }

    #[inline(always)]
    fn _set_palette(&mut self, idx: i32, palette: BgPalette) -> &mut Self {
        let idx = u_mod(idx, self.linear_size) as usize;
        self.cur_buffers.palette[idx] = palette;
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
        self.cur_buffers.symmetry[idx]
    }

    #[inline(always)]
    fn _set_symmetry(&mut self, idx: i32, symmetry: BgSymmetry) -> &mut Self {
        let idx = u_mod(idx ,self.linear_size) as usize;
        self.cur_buffers.symmetry[idx] = symmetry;
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
                if self.cur_buffers.code[idx] != self.alt_buffers.code[idx]
                || self.cur_buffers.palette[idx] != self.alt_buffers.palette[idx]
                || self.cur_buffers.symmetry[idx] != self.alt_buffers.symmetry[idx] {
                    self.alt_buffers.code[idx] = self.cur_buffers.code[idx];
                    self.alt_buffers.palette[idx] = self.cur_buffers.palette[idx];
                    self.alt_buffers.symmetry[idx] = self.cur_buffers.symmetry[idx];
                    // rendering proc
                    bg_lib::drawchar(
                        self.char_data,
                        self.pal_data,
                        self.cur_buffers.code[idx],
                        self.cur_buffers.palette[idx],
                        self.cur_buffers.symmetry[idx].compose(self.base_symmetry),
                        ((x * 8 * self.pixel_scale) as u32, (y * 8 * self.pixel_scale) as u32),
                        (self.pixel_scale as u32, self.pixel_scale as u32),
                        &mut self.rendered_image,
                    );
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
