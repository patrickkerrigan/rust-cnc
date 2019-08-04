use std::os::raw::c_char;
use std::ffi::{CStr, CString};

#[derive(PartialEq)]
enum ParserState {
    PolylineStart,
    PolylineVertexCount,
    PolyLineVertex,
}

#[derive(Debug)]
pub struct Vertex {
    pub x: f64,
    pub y: f64
}

pub struct PartialVertex {
    pub x: Option<f64>,
    pub y: Option<f64>
}

type PolyLine = Vec<Vertex>;

impl Vertex {
    pub fn from_partial(partial: &PartialVertex) -> Option<Vertex> {
        match partial {
            PartialVertex{x: Some(dx), y: Some(dy)} => Some(Vertex{x: *dx, y: *dy}),
            _ => None
        }
    }
}

impl PartialVertex {
    pub fn new() -> PartialVertex {
        PartialVertex {x: None, y: None}
    }
}

#[no_mangle]
pub extern fn dxf_to_gcode(s: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let dxf = c_str.to_str().unwrap();

    let gcode = process(dxf);

    let c_str_song = CString::new(gcode).unwrap();
    c_str_song.into_raw()
}

#[no_mangle]
pub extern fn gcode_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

pub fn process(dxf_contents: &str) -> String {
    generate_gcode(&convert(&collect_pairs(&dxf_contents)))
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

fn convert(pairs: &Vec<(&str, &str)>) -> Vec<PolyLine> {
    let mut lines = vec![];
    let mut vertices = vec![];
    let mut state = ParserState::PolylineStart;
    let mut vert = PartialVertex::new();
    let mut vertices_found :u64 = 0;
    let mut vertices_expected :u64 = 0;

    for &pair in pairs.iter() {
        match pair {
            ("100", "AcDbPolyline") if state == ParserState::PolylineStart => {
                state = ParserState::PolylineVertexCount;
            },

            ("90", n) if state == ParserState::PolylineVertexCount => {
                vertices_expected = n.parse().unwrap();
                state = ParserState::PolyLineVertex;
            },

            ("10", x) if state == ParserState::PolyLineVertex => {
                vert.x = Some(x.parse().unwrap());
            },

            ("20", y) if state == ParserState::PolyLineVertex => {
                vert.y = Some(y.parse().unwrap());
            }

            _ => continue
        }

        if let Some(v) = Vertex::from_partial(&vert) {
            vertices.push(v);
            vert = PartialVertex::new();
            vertices_found += 1;
        }

        if state == ParserState::PolyLineVertex && vertices_expected == vertices_found {
            lines.push(vertices);
            vertices = vec![];
            state = ParserState::PolylineStart;
            vertices_found = 0;
            vertices_expected = 0;
        }
    }

    lines
}

fn generate_gcode(lines: &Vec<PolyLine>) -> String {
    lines.iter()
        .map(polyline_to_gcode)
        .collect()
}

fn polyline_to_gcode(line: &PolyLine) -> String {
    let mut gcode = String::new();
    let mut iterator = line.iter();

    if let Some(first_line) = iterator.next() {
        gcode += format!("M05\nX{:.2} Y{:.2} F2000\n", first_line.x, first_line.y).as_str();
    }

    for line in iterator {
        gcode += format!("X{:.2} Y{:.2} M03 S255 F1000\n", line.x, line.y).as_str();
    }

    gcode
}
