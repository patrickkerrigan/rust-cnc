use crate::vertex::Vertex;

#[derive(Debug)]
pub struct PolyLine {
    pub vertices: Vec<Vertex>,
    pub closed: bool
}

