use crate::dxf::polyline::PolyLine;
use crate::dxf::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Spline {
    pub control_points: Vec<Vertex>
}

impl Spline {
    pub fn into_polyline(self) -> PolyLine {
        let mut points = vec![Vertex{x: self.control_points[0].x, y: self.control_points[0].y}];
        let resolution = (self.control_points[0].distance_to(&self.control_points[3]) as i64 * 4).min(25).max(6);
        let step = 1f64 / (resolution as f64 - 1f64);

        for s in 1..(resolution - 2) {
            let t = s as f64 * step;
            let cx = (1.0 - t).powf(3.0) * self.control_points[0].x + 3.0 * (1.0 - t).powf(2.0) * t * self.control_points[1].x + 3.0 * (1.0 - t) * t.powf(2.0) * self.control_points[2].x + t.powf(3.0) * self.control_points[3].x;
            let cy = (1.0 - t).powf(3.0) * self.control_points[0].y + 3.0 * (1.0 - t).powf(2.0) * t * self.control_points[1].y + 3.0 * (1.0 - t) * t.powf(2.0) * self.control_points[2].y + t.powf(3.0) * self.control_points[3].y;

            points.push(Vertex { x: cx, y: cy });
        }

        points.push(Vertex {x: self.control_points[3].x, y: self.control_points[3].y});

        PolyLine { vertices: points, closed: false }
    }
}
