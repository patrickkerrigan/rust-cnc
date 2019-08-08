use crate::vertex::Vertex;
use crate::polyline::PolyLine;

#[derive(Debug)]
pub struct Spline {
    pub control_points: Vec<Vertex>
}

impl Spline {
    pub fn into_polyline(self) -> PolyLine {
        let mut points = vec![Vertex{x: self.control_points[0].x, y: self.control_points[0].y}];
        for s in 1..99 {
            let t = s as f64 * 0.01;
            let cx = (1.0 - t).powf(3.0) * self.control_points[0].x + 3.0 * (1.0 - t).powf(2.0) * t * self.control_points[1].x + 3.0 * (1.0 - t) * t.powf(2.0) * self.control_points[2].x + t.powf(3.0) * self.control_points[3].x;
            let cy = (1.0 - t).powf(3.0) * self.control_points[0].y + 3.0 * (1.0 - t).powf(2.0) * t * self.control_points[1].y + 3.0 * (1.0 - t) * t.powf(2.0) * self.control_points[2].y + t.powf(3.0) * self.control_points[3].y;

            points.push(Vertex { x: cx, y: cy });
        }

        points.push(Vertex {x: self.control_points[3].x, y: self.control_points[3].y});

        PolyLine { vertices: points, closed: false }
    }
}
