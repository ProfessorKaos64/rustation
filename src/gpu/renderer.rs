
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
}

pub trait Renderer {
    fn push_triangle(&mut self, &[Vertex; 3]);
}
