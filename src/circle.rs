use crate::vertex::Vertex;
use crate::polyline::PolyLine;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Circle {
    pub centre: Vertex,
    pub radius: f64
}

impl Circle {
    pub fn into_polyline(self) -> PolyLine {
        let mut points = vec![];

        let circumference: f64 = 2.0 * PI * self.radius;
        let resolution = 360.0 / circumference.max(6.0).min(180.0);

        for i in (0..360).step_by(resolution as usize) {
            let rad = (i as f64).to_radians();
            let vector = Vertex::from_polar(self.radius, rad);
            points.push(self.centre + vector);
        }

        PolyLine {vertices: points, closed: true}
    }
}
