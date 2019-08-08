use crate::polyline::PolyLine;
use crate::vertex::{PartialVertex, Vertex};
use std::slice;
use crate::spline::Spline;

#[derive(PartialEq)]
enum PolylineParserState {
    VertexCount,
    Closed,
    Vertex,
}

#[derive(PartialEq)]
enum SplineParserState {
    ControlPointCount,
    ControlPoint,
}

pub fn parse(dxf_contents: &str) -> Vec<PolyLine> {
    convert(&collect_pairs(dxf_contents))
}

fn collect_pairs(dxf_contents: &str) -> Vec<(&str, &str)> {
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

fn parse_polyline(iterator: &mut slice::Iter<(&str, &str)>) -> Option<PolyLine> {
    let mut vertices = vec![];
    let mut state = PolylineParserState::VertexCount;
    let mut vert = PartialVertex::new();
    let mut closed = false;
    let mut vertices_found :u64 = 0;
    let mut vertices_expected :u64 = 0;

    while let Some(&pair) = iterator.next() {
        match pair {
            ("90", n) if state == PolylineParserState::VertexCount => {
                vertices_expected = n.parse().unwrap();
                state = PolylineParserState::Closed;
            },

            ("70", n) if state == PolylineParserState::Closed => {
                closed = n == "1";
                state = PolylineParserState::Vertex;
            },

            ("10", x) if state == PolylineParserState::Vertex => {
                vert.x = Some(x.parse().unwrap());
            },

            ("20", y) if state == PolylineParserState::Vertex => {
                vert.y = Some(y.parse().unwrap());
            }

            _ => continue
        }

        if let Some(v) = Vertex::from_partial(&vert) {
            vertices.push(v);
            vert = PartialVertex::new();
            vertices_found += 1;
        }

        if state == PolylineParserState::Vertex && vertices_expected == vertices_found {
            if vertices_found > 1 {
                return Some(PolyLine { vertices, closed });
            }

            return None;
        }
    }

    None
}

fn parse_spline(iterator: &mut slice::Iter<(&str, &str)>) -> Option<Spline> {
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

fn convert(pairs: &Vec<(&str, &str)>) -> Vec<PolyLine> {
    let mut lines = vec![];
    let mut iterator = pairs.iter();

    while let Some(&pair) = iterator.next() {
        match pair {
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

            _ => continue
        }
    }

    lines
}
