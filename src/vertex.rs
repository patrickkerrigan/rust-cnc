use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
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
        self.vector_to(other).magnitude()
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn midpoint(&self, other: &Vertex) -> Vertex {
        Vertex {
            x: (other.x - self.x) / 2.0 + self.x,
            y: (other.y - self.y) / 2.0 + self.y
        }
    }

    pub fn vector_to(&self, other: &Vertex) -> Vertex {
        Vertex {
            x: other.x - self.x,
            y: other.y - self.y
        }
    }

    pub fn normal(&self) -> Vertex {
        Vertex {
            x: self.y,
            y: -self.x
        }
    }

    pub fn unit_vector(&self) -> Vertex {
        let magnitude = self.magnitude();

        Vertex {
            x: self.x / magnitude,
            y: self.y / magnitude
        }
    }

    pub fn with_magnitude(&self, magnitude: f64) -> Vertex {
        let unit = self.unit_vector();

        Vertex {
            x: unit.x * magnitude,
            y: unit.y * magnitude
        }
    }

}

impl Add for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Self) -> Self::Output {
        Vertex {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl PartialVertex {
    pub fn new() -> PartialVertex {
        PartialVertex {x: None, y: None}
    }
}
