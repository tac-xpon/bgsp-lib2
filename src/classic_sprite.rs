pub use super::bgsp_common::{SpPos, SpCode, SpPalette, SpSymmetry};

#[derive(Debug, Default, Clone)]
pub struct ClassicSprite {
    pub pos: SpPos,
    pub code: SpCode,
    pub palette: SpPalette,
    pub symmetry: SpSymmetry,
    pub visible: bool,
    pub priority: i32,
}

impl ClassicSprite {
    pub fn pos(&mut self, pos: SpPos) -> &mut Self {
        self.pos = pos;
        self
    }
    pub fn xy(&mut self, x: i32, y:i32) -> &mut Self {
        (self.pos.x, self.pos.y) = (x, y);
        self
    }

    pub fn code(&mut self, code: SpCode) -> &mut Self {
        self.code = code;
        self
    }

    pub fn palette(&mut self, palette: SpPalette) -> &mut Self {
        self.palette = palette;
        self
    }

    pub fn symmetry(&mut self, symmetry: SpSymmetry) -> &mut Self {
        self.symmetry = symmetry;
        self
    }

    pub fn visible(&mut self, visible: bool) -> &mut Self {
        self.visible = visible;
        self
    }

    pub fn priority(&mut self, priority: i32) -> &mut Self {
        self.priority = priority;
        self
    }
}