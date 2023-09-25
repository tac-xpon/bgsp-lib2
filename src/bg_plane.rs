pub use super::bg_resources::*;

#[inline(always)]
fn u_mod(x: i32, p: i32) -> i32 {
    (x % p + p) % p
}

pub type DrawRects = Vec<([f64; 4], [f64; 4])>;

pub struct BgPlane<'a> {
    resources: BgResources<'a>,
    buffer_rect_size: (i32, i32),
    whole_size: (i32, i32),
    view_size: (i32, i32),
    view_pos: (i32, i32),
    cur_idx: i32,
    pixel_scale: i32,
    draw_rects: DrawRects,
}

impl <'a> BgPlane<'a> {
    pub fn new(
        buffer_rect_size: (i32, i32),
        view_size: (i32, i32),
        char_data: &'a [[u64; PATTERN_SIZE]],
        pal_data: &'a [[Rgba<u8>; NUM_PALETTE_COL]],
        pixel_scale: i32,
    ) -> Self {
        let draw_rects: DrawRects = vec![
            (
                [0.0, 0.0, (view_size.0 * pixel_scale) as f64, (view_size.1 * pixel_scale) as f64],
                [0.0, 0.0, (view_size.0 * pixel_scale) as f64, (view_size.1 * pixel_scale) as f64],
            )
        ];
        Self {
            resources: BgResources::new(buffer_rect_size, char_data, pal_data, pixel_scale),
            buffer_rect_size,
            whole_size: (buffer_rect_size.0 * PATTERN_SIZE as i32, buffer_rect_size.1 * PATTERN_SIZE as i32),
            view_size,
            view_pos: (0, 0),
            cur_idx: 0,
            pixel_scale,
            draw_rects,
        }
    }

    pub const fn buffer_width(&self) -> i32 {
        self.buffer_rect_size.0
    }

    pub const fn buffer_height(&self) -> i32 {
        self.buffer_rect_size.1
    }

    pub const fn buffer_rect_size(&self) -> (i32, i32) {
        self.buffer_rect_size
    }

    pub const fn buffer_linear_size(&self) -> i32 {
        self.resources.linear_size()
    }

    pub const fn view_width(&self) -> i32 {
        self.view_size.0
    }

    pub const fn view_height(&self) -> i32 {
        self.view_size.1
    }

    pub const fn view_size(&self) -> (i32, i32) {
        self.view_size
    }

    pub const fn view_pos(&self) -> (i32, i32) {
        self.view_pos
    }

    pub const fn cur_idx(&self) -> i32 {
        self.cur_idx
    }

    pub fn cur_pos(&self) -> (i32, i32) {
        let idx = u_mod(self.cur_idx, self.resources.linear_size());
        (idx % self.buffer_rect_size.0, idx / self.buffer_rect_size.1)
    }

    pub const fn pixel_scale(&self) -> i32 {
        self.pixel_scale
    }

    pub const fn base_symmetry(&self) -> BgSymmetry {
        self.resources.base_symmetry()
    }

    pub fn set_view_pos(&mut self, x: i32, y: i32) -> &mut Self {
        self.view_pos = (x, y);
        self
    }

    pub fn set_cur_idx(&mut self, idx: i32) -> &mut Self {
        self.cur_idx = idx;
        self
    }

    pub fn set_cur_pos(&mut self, x: i32, y: i32) -> &mut Self {
        self.cur_idx = x + y * self.resources.rect_size().0;
        self
    }

    pub fn set_base_symmetry(&mut self, base_symmetry: BgSymmetry) -> &mut Self {
        self.resources.set_base_symmetry(base_symmetry);
        self
    }

    pub fn read_achar(&self) -> AChar {
        self.resources.get_achar(self.cur_idx)
    }

    pub fn get_achar(&self, idx: i32) -> AChar {
        self.resources.get_achar(idx)
    }

    pub fn get_achar_at(&self, x: i32, y: i32) -> AChar {
        self.resources.get_achar_at(x, y)
    }

    pub fn put_achar(&mut self, achar: &AChar) -> &mut Self {
        self.resources.set_achar(self.cur_idx, achar);
        self.cur_idx += 1;
        self
    }

    pub fn put_achar_n(&mut self, achar: &AChar, n: i32) -> &mut Self {
        self.resources.set_achar_n(self.cur_idx, achar, n);
        self.cur_idx += n;
        self
    }

    pub fn set_achar(&mut self, idx: i32, achar: &AChar) -> &mut Self {
        self.resources.set_achar(idx, achar);
        self
    }

    pub fn set_achar_n(&mut self, idx: i32, achar: &AChar, n: i32) -> &mut Self {
        self.resources.set_achar_n(idx, achar, n);
        self
    }

    pub fn set_achar_at(&mut self, x: i32, y: i32, achar: &AChar) -> &mut Self {
        self.resources.set_achar_at(x, y, achar);
        self
    }

    pub fn set_achar_n_at(&mut self, x: i32, y: i32, achar: &AChar, n: i32) -> &mut Self {
        self.resources.set_achar_n_at(x, y, achar, n);
        self
    }

    pub fn fill_achar(&mut self, achar: &AChar) -> &mut Self {
        self.resources.fill_achar(achar);
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.resources.clear();
        self
    }

    pub fn read_attributes(&self) -> CharAttributes {
        self.resources.get_attributes(self.cur_idx)
    }

    pub fn get_attributes(&self, idx: i32) -> CharAttributes {
        self.resources.get_attributes(idx)
    }

    pub fn get_attributes_at(&self, x: i32, y: i32) -> CharAttributes {
        self.resources.get_attributes_at(x, y)
    }

    pub fn put_attributes(&mut self, attributes: &CharAttributes) -> &mut Self {
        self.resources.set_attributes(self.cur_idx, attributes);
        self.cur_idx += 1;
        self
    }

    pub fn put_attributes_n(&mut self, attributes: &CharAttributes, n: i32) -> &mut Self {
        self.resources.set_attributes_n(self.cur_idx, attributes, n);
        self.cur_idx += n;
        self
    }

    pub fn set_attributes(&mut self, idx: i32, attributes: &CharAttributes) -> &mut Self {
        self.resources.set_attributes(idx, attributes);
        self
    }

    pub fn set_attributes_n(&mut self, idx: i32, attributes: &CharAttributes, n: i32) -> &mut Self {
        self.resources.set_attributes_n(idx, attributes, n);
        self
    }

    pub fn set_attributes_at(&mut self, x: i32, y: i32, attributes: &CharAttributes) -> &mut Self {
        self.resources.set_attributes_at(x, y, attributes);
        self
    }

    pub fn set_attributes_n_at(&mut self, x: i32, y: i32, attributes: &CharAttributes, n: i32) -> &mut Self {
        self.resources.set_attributes_n_at(x, y, attributes, n);
        self
    }

    pub fn fill_attributes(&mut self, attributes: &CharAttributes) -> &mut Self {
        self.resources.fill_attributes(attributes);
        self
    }

    pub fn read_code(&self) -> BgCode {
        self.resources.get_code(self.cur_idx)
    }

    pub fn get_code(&self, idx: i32) -> BgCode {
        self.resources.get_code(idx)
    }

    pub fn get_code_at(&self, x: i32, y: i32) -> BgCode {
        self.resources.get_code_at(x, y)
    }

    pub fn put_code<T: Into<BgCode>>(&mut self, code: T) -> &mut Self {
        self.resources.set_code(self.cur_idx, code);
        self.cur_idx += 1;
        self
    }

    pub fn put_code_n<T: Into<BgCode>>(&mut self, code: T, n: i32) -> &mut Self {
        self.resources.set_code_n(self.cur_idx, code, n);
        self.cur_idx += n;
        self
    }

    pub fn set_code<T: Into<BgCode>>(&mut self, idx: i32, code: T) -> &mut Self {
        self.resources.set_code(idx, code);
        self
    }

    pub fn set_code_n<T: Into<BgCode>>(&mut self, idx: i32, code: T, n: i32) -> &mut Self {
        self.resources.set_code_n(idx, code, n);
        self
    }

    pub fn set_code_at<T: Into<BgCode>>(&mut self, x: i32, y: i32, code: T) -> &mut Self {
        self.resources.set_code_at(x, y, code);
        self
    }

    pub fn set_code_n_at<T: Into<BgCode>>(&mut self, x: i32, y: i32, code: T, n: i32) -> &mut Self {
        self.resources.set_code_n_at(x, y, code, n);
        self
    }

    pub fn fill_code<T: Into<BgCode>>(&mut self, code: T) -> &mut Self {
        self.resources.fill_code(code);
        self
    }

    pub fn read_palette(&self) -> BgPalette {
        self.resources.get_palette(self.cur_idx)
    }

    pub fn get_palette(&self, idx: i32) -> BgPalette {
        self.resources.get_palette(idx)
    }

    pub fn get_palette_at(&self, x: i32, y: i32) -> BgPalette {
        self.resources.get_palette_at(x, y)
    }

    pub fn put_palette(&mut self, palette: BgPalette) -> &mut Self {
        self.resources.set_palette(self.cur_idx, palette);
        self.cur_idx += 1;
        self
    }

    pub fn put_palette_n(&mut self, palette: BgPalette, n: i32) -> &mut Self {
        self.resources.set_palette_n(self.cur_idx, palette, n);
        self.cur_idx += n;
        self
    }

    pub fn set_palette(&mut self, idx: i32, palette: BgPalette) -> &mut Self {
        self.resources.set_palette(idx, palette);
        self
    }

    pub fn set_palette_n(&mut self, idx: i32, palette: BgPalette, n: i32) -> &mut Self {
        self.resources.set_palette_n(idx, palette, n);
        self
    }

    pub fn set_palette_at(&mut self, x: i32, y: i32, palette: BgPalette) -> &mut Self {
        self.resources.set_palette_at(x, y, palette);
        self
    }

    pub fn set_palette_n_at(&mut self, x: i32, y: i32, palette: BgPalette, n: i32) -> &mut Self {
        self.resources.set_palette_n_at(x, y, palette, n);
        self
    }

    pub fn fill_palette(&mut self, palette: BgPalette) -> &mut Self {
        self.resources.fill_palette(palette);
        self
    }

    pub fn read_symmetry(&self) -> BgSymmetry {
        self.resources.get_symmetry(self.cur_idx)
    }

    pub fn get_symmetry(&self, idx: i32) -> BgSymmetry {
        self.resources.get_symmetry(idx)
    }

    pub fn get_symmetry_at(&self, x: i32, y: i32) -> BgSymmetry {
        self.resources.get_symmetry_at(x, y)
    }

    pub fn put_symmetry(&mut self, symmetry: BgSymmetry) -> &mut Self {
        self.resources.set_symmetry(self.cur_idx, symmetry);
        self.cur_idx += 1;
        self
    }

    pub fn put_symmetry_n(&mut self, symmetry: BgSymmetry, n: i32) -> &mut Self {
        self.resources.set_symmetry_n(self.cur_idx, symmetry, n);
        self.cur_idx += n;
        self
    }

    pub fn set_symmetry(&mut self, idx: i32, symmetry: BgSymmetry) -> &mut Self {
        self.resources.set_symmetry(idx, symmetry);
        self
    }

    pub fn set_symmetry_at(&mut self, x: i32, y: i32, symmetry: BgSymmetry) -> &mut Self {
        self.resources.set_symmetry_at(x, y, symmetry);
        self
    }

    pub fn set_symmetry_n_at(&mut self, x: i32, y: i32, symmetry: BgSymmetry, n: i32) -> &mut Self {
        self.resources.set_symmetry_n_at(x, y, symmetry, n);
        self
    }

    pub fn fill_symmetry(&mut self, symmetry: BgSymmetry) -> &mut Self {
        self.resources.fill_symmetry(symmetry);
        self
    }

    fn _set_string(&mut self, idx: i32, thestr: &str, op_attributes: Option<&CharAttributes>) -> i32 {
        let mut idx = idx;
        let mut dx = 0;
        let linear_size = self.resources.linear_size();
        if let Some(attributes) = op_attributes {
            let mut achar = AChar {code: 0, palette: attributes.palette, symmetry: attributes.symmetry};
            for character in thestr.chars() {
                if character == '\n' {
                    idx = idx - dx + self.buffer_rect_size.0;
                    dx = 0;
                    continue;
                }
                if (character as u32) < 0x80 {
                    achar.code = character as u32;
                    self.set_achar(idx, &achar);
                }
                idx = (idx + 1) % linear_size;
                dx += 1;
            }
        } else {
            for character in thestr.chars() {
                if character == '\n' {
                    idx = idx - dx + self.buffer_rect_size.0;
                    dx = 0;
                    continue;
                }
                if (character as u32) < 0x80 {
                    self.set_code(idx, character as u32);
                }
                idx = (idx + 1) % linear_size;
                dx += 1;
            }
        }
        idx
    }

    pub fn put_string(&mut self, thestr: &str, op_attributes: Option<&CharAttributes>) -> &mut Self {
        self.cur_idx = self._set_string(self.cur_idx, thestr, op_attributes);
        self
    }

    pub fn set_string(&mut self, idx: i32, thestr: &str, op_attributes: Option<&CharAttributes>) -> &mut Self {
        self._set_string(idx, thestr, op_attributes);
        self
    }

    pub fn set_string_at(&mut self, x: i32, y: i32, thestr: &str, op_attributes: Option<&CharAttributes>) -> &mut Self {
        let idx = x + y * self.buffer_rect_size.0;
        self._set_string(idx, thestr, op_attributes);
        self
    }

    pub fn rendering(&mut self) -> i32 {
        let scale = self.pixel_scale;
        let mut draw_rects: Vec<([f64; 4], [f64; 4])> = Vec::with_capacity(4);
        let (x0, y0) = (u_mod(self.view_pos.0, self.whole_size.0 as i32), u_mod(self.view_pos.1, self.whole_size.1 as i32));
        let (x_end, y_end) = (x0 + self.view_size.0, y0 + self.view_size.1);
        let (w0, y1, w1) = if x_end > self.whole_size.0 {
            (self.whole_size.0 - x0, y0, x_end - self.whole_size.0)
        } else {
            (self.view_size.0, 0, 0)
        };
        let (h0, x2, h2) = if y_end > self.whole_size.1 {
            (self.whole_size.1 - y0, x0, y_end - self.whole_size.1)
        } else {
            (self.view_size.1, 0, 0)
        };

        draw_rects.push((
            [0.0,                 0.0,                 (w0 * scale) as f64, (h0 * scale) as f64],
            [(x0 * scale) as f64, (y0 * scale) as f64, (w0 * scale) as f64, (h0 * scale) as f64],
        ));
        if w1 > 0 {
            draw_rects.push((
                [(w0 * scale) as f64, 0.0                , (w1 * scale) as f64, (h0 * scale) as f64],
                [0.0,                 (y1 * scale) as f64, (w1 * scale) as f64, (h0 * scale) as f64],
            ));
        }
        if h2 > 0 {
            draw_rects.push((
                [0.0,                 (h0 * scale) as f64, (w0 * scale) as f64, (h2 * scale) as f64],
                [(x2 * scale) as f64, 0.0,                 (w0 * scale) as f64, (h2 * scale) as f64],
            ));
        }
        if w1 > 0 && h2 > 0 {
            draw_rects.push((
                [(w0 * scale) as f64, (h0 * scale) as f64, (w1 * scale) as f64, (h2 * scale) as f64],
                [0.0,                 0.0,                 (w1 * scale) as f64, (h2 * scale) as f64],
            ));
        }
        self.draw_rects = draw_rects;
        self.resources.rendering()
    }

    pub fn whole_image(&self) -> &RgbaImage {
        self.resources.rendered_image()
    }

    pub fn draw_rects(&self) -> &DrawRects {
        &self.draw_rects
    }
}