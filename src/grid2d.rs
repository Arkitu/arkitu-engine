pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 360;

#[derive(Clone, Copy)]
pub struct Color (u32);
impl Color {
    pub fn new(r:u8, g:u8, b:u8) -> Self {
        Self(u32::from_le_bytes([0, r, g, b]))
    }
    pub fn rgb(&self) -> [u8; 3] {
        let b = self.0.to_be_bytes();
        [b[1], b[2], b[3]]
    }
}

pub type Pos2D = (usize, usize);

pub struct Grid2D (pub Vec<u32>);
impl Default for Grid2D {
    fn default() -> Self {
        Grid2D(vec![0; WIDTH * HEIGHT])
    }
}
impl Grid2D {
    pub fn get(&self, pos:Pos2D) -> Color {
        Color(self.0[pos.0 + (pos.1 * WIDTH)])
    }
    pub fn set(&mut self, pos:Pos2D, val:Color) {
        self.0[pos.0 + (pos.1 * WIDTH)] = val.0;
    }
}