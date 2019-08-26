use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use crate::polyline::{PolyLine, glue_polylines};
use crate::parser::parse;

mod vertex;
mod polyline;
mod spline;
mod parser;
mod circle;

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
    String::from("G01\n")
        + generate_gcode(&glue_polylines(parse(&dxf_contents))).as_str()
        + "M05 F2000 X0 Y0"
}

fn generate_gcode(lines: &Vec<PolyLine>) -> String {
    lines.iter()
        .map(polyline_to_gcode)
        .collect()
}

fn polyline_to_gcode(line: &PolyLine) -> String {
    let mut gcode = String::new();
    let mut gcode_end = String::new();
    let mut iterator = line.vertices.iter();

    if let Some(first_line) = iterator.next() {
        gcode += format!("M05 F2000 X{:.2} Y{:.2}\n", first_line.x, first_line.y).as_str();

        if line.closed {
            gcode_end += format!("M03 F1000 X{:.2} Y{:.2} S255\n", first_line.x, first_line.y).as_str();
        }
    }

    for line in iterator {
        gcode += format!("M03 F1000 X{:.2} Y{:.2} S255\n", line.x, line.y).as_str();
    }

    gcode + gcode_end.as_str()
}

