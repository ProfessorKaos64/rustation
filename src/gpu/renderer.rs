
pub struct Vertex {
    position: [i16; 2],
    color: [u8; 3],
}

impl Vertex {
    pub fn new(position: [i16; 2], color: [u8; 3]) -> Vertex {
        Vertex {
            position: position,
            color: color,
        }
    }

    pub fn position(&self) -> [i16; 2] {
        self.position
    }

    pub fn color(&self) -> [u8; 3] {
        self.color
    }
}

pub trait Renderer {
    fn set_draw_offset(&mut self, x: i16, y: i16);

    fn push_triangle(&mut self, &[Vertex; 3]);
    fn push_quad(&mut self, &[Vertex; 4]);
}
