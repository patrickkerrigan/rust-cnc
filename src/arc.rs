use crate::vertex::Vertex;
use crate::polyline::PolyLine;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Arc {
    pub centre: Vertex,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64
}

impl Arc {
    pub fn into_polyline(self) -> PolyLine {
        let mut points = vec![];
        let angle_size = self.end_angle - self.start_angle;

        let circumference = (2.0 * PI * self.radius) * (angle_size.abs() / (2.0 * PI));
        let resolution = ((circumference * 1.5) as i64).min(25).max(6);
        let step = 1.0 / (resolution as f64);

        for s in 1..resolution {
            let i = s as f64 * step * angle_size;
            let angle = self.start_angle + i;
            let x = self.centre.x + (angle.cos() * self.radius);
            let y = self.centre.y + (angle.sin() * self.radius);

            points.push(Vertex {x, y});
        }

        PolyLine {vertices: points, closed: false}
    }
}