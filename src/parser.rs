use crate::polyline::PolyLine;
use crate::vertex::{PartialVertex, Vertex};
use std::slice;
use crate::spline::Spline;
use crate::circle::Circle;
use crate::bulge::{VertexWithBulge, explode_bulged_vertices};

#[derive(PartialEq)]
enum LineParserState {
    Start,
    End
}

#[derive(PartialEq)]
enum PolylineParserState {
    Closed,
    Vertex,
    Finish
}

#[derive(PartialEq)]
enum SplineParserState {
    ControlPointCount,
    ControlPoint,
}

#[derive(PartialEq)]
enum CircleParserState {
    Centre,
    Radius
}

type DataPair<'a> = (&'a str, &'a str);

pub fn parse(dxf_contents: &str) -> Vec<PolyLine> {
    convert(&collect_pairs(dxf_contents))
}

fn collect_pairs(dxf_contents: &str) -> Vec<DataPair> {
    let mut pairs = vec![];

    let mut line_iterator = dxf_contents.lines();

    while let Some(line) = line_iterator.next() {
        match line_iterator.next() {
            Some(l) => pairs.push((line.trim(), l.trim())),
            None => continue
        }
    }

    pairs
}

fn parse_line(iterator: &mut slice::Iter<DataPair>) -> Option<PolyLine> {
    let mut vertices = vec![];
    let mut state = LineParserState::Start;
    let mut vert = PartialVertex::new();

    while let Some(&pair) = iterator.next() {
        match pair {
            ("10", x) if state == LineParserState::Start => {
                vert.x = Some(x.parse().unwrap());
            },

            ("20", y) if state == LineParserState::Start => {
                vert.y = Some(y.parse().unwrap());
            },

            ("11", x) if state == LineParserState::End => {
                vert.x = Some(x.parse().unwrap());
            },

            ("21", y) if state == LineParserState::End => {
                vert.y = Some(y.parse().unwrap());
            }

            _ => continue
        }

        if let Some(v) = Vertex::from_partial(&vert) {
            vertices.push(v);
            vert = PartialVertex::new();
            state = LineParserState::End;
        }

        if state == LineParserState::End && vertices.len() == 2 {
            return Some(PolyLine{vertices, closed: false});
        }
    }

    None
}

fn parse_polyline(iterator: &mut slice::Iter<DataPair>) -> Option<PolyLine> {
    let mut vertices: Vec<VertexWithBulge> = vec![];
    let mut state = PolylineParserState::Closed;
    let mut vert = PartialVertex::new();
    let mut closed = false;
    let mut vertices_found :u64 = 0;

    while let Some(&pair) = iterator.next() {
        match pair {
            ("70", n) if state == PolylineParserState::Closed => {
                closed = n == "1";
                state = PolylineParserState::Vertex;
            },

            ("10", x) if state == PolylineParserState::Vertex => {
                vert.x = Some(x.parse().unwrap());
            },

            ("20", y) if state == PolylineParserState::Vertex => {
                vert.y = Some(y.parse().unwrap());
            },

            ("42", b) if state == PolylineParserState::Vertex => {
                vertices.last_mut().unwrap().bulge = b.parse().unwrap();
            }

            ("0", _) => {
                state = PolylineParserState::Finish;
            }

            _ => continue
        }

        if let Some(v) = Vertex::from_partial(&vert) {
            vertices.push(VertexWithBulge{ vertex: v, bulge: 0.0 });
            vert = PartialVertex::new();
            vertices_found += 1;
        }

        if state == PolylineParserState::Finish {
            if vertices_found > 1 {
                return Some(PolyLine { vertices: explode_bulged_vertices(vertices), closed });
            }

            return None;
        }
    }

    None
}

fn parse_spline(iterator: &mut slice::Iter<DataPair>) -> Option<Spline> {
    let mut vertices = vec![];
    let mut state = SplineParserState::ControlPointCount;
    let mut vert = PartialVertex::new();
    let mut control_points_found :u64 = 0;
    let mut control_points_expected :u64 = 0;

    while let Some(&pair) = iterator.next() {
        match pair {
            ("73", n) if state == SplineParserState::ControlPointCount => {
                control_points_expected = n.parse().unwrap();
                state = SplineParserState::ControlPoint;
            },

            ("10", x) if state == SplineParserState::ControlPoint => {
                vert.x = Some(x.parse().unwrap());
            },

            ("20", y) if state == SplineParserState::ControlPoint => {
                vert.y = Some(y.parse().unwrap());
            }

            _ => continue
        }

        if let Some(v) = Vertex::from_partial(&vert) {
            vertices.push(v);
            vert = PartialVertex::new();
            control_points_found += 1;
        }

        if state == SplineParserState::ControlPoint && control_points_expected == control_points_found {
            if control_points_found == 4 {
                return Some(Spline {control_points: vertices });
            }

            return None;
        }
    }

    None
}

fn parse_circle(iterator: &mut slice::Iter<DataPair>) -> Option<Circle> {
    let mut state = CircleParserState::Centre;
    let mut vert = PartialVertex::new();
    let mut radius: f64 = 0.0;

    while let Some(&pair) = iterator.next() {
        match pair {
            ("10", x) if state == CircleParserState::Centre => {
                vert.x = Some(x.parse().unwrap());
            },

            ("20", y) if state == CircleParserState::Centre => {
                vert.y = Some(y.parse().unwrap());
            },

            ("40", r) if state == CircleParserState::Radius => {
                radius = r.parse().unwrap();
            }

            _ => continue
        }

        if let Some(_) = Vertex::from_partial(&vert) {
            state = CircleParserState::Radius;
        }

        if state == CircleParserState::Radius && radius > 0.0 {
            return Some(Circle{centre: Vertex::from_partial(&vert).unwrap(), radius});
        }
    }

    None
}

fn convert(pairs: &Vec<DataPair>) -> Vec<PolyLine> {
    let mut lines = vec![];
    let mut iterator = pairs.iter();

    while let Some(&pair) = iterator.next() {
        match pair {
            ("100", "AcDbLine") => {
                if let Some(line) = parse_line(&mut iterator) {
                    lines.push(line);
                }
            },

            ("100", "AcDbPolyline") => {
                if let Some(line) = parse_polyline(&mut iterator) {
                    lines.push(line);
                }
            },

            ("100", "AcDbSpline") => {
                if let Some(spline) = parse_spline(&mut iterator) {
                    lines.push(spline.into_polyline());
                }
            },

            ("100", "AcDbCircle") => {
                if let Some(circle) = parse_circle(&mut iterator) {
                    lines.push(circle.into_polyline());
                }
            },

            _ => continue
        }
    }

    lines
}
