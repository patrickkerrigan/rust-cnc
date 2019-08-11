#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Vertex {
    pub x: f64,
    pub y: f64
}

pub struct PartialVertex {
    pub x: Option<f64>,
    pub y: Option<f64>
}


impl Vertex {
    pub fn from_partial(partial: &PartialVertex) -> Option<Vertex> {
        match partial {
            PartialVertex{x: Some(dx), y: Some(dy)} => Some(Vertex{x: *dx, y: *dy}),
            _ => None
        }
    }

    pub fn distance_to(&self, other: &Vertex) -> f64 {
        ((other.x - self.x).powf(2f64) + (other.y - self.y).powf(2f64)).sqrt()
    }
}

impl PartialVertex {
    pub fn new() -> PartialVertex {
        PartialVertex {x: None, y: None}
    }
}
