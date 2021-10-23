use crate::dxf::parser::parse;
use crate::dxf::polyline::{glue_polylines, PolyLine};

mod vertex;
mod polyline;
mod spline;
mod parser;
mod circle;
mod arc;
mod bulge;

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
        gcode += format!("M05 F2000 X{:.2} Y{:.2}\n", first_line.x(), first_line.y()).as_str();

        if line.closed {
            gcode_end += format!("M03 F1000 X{:.2} Y{:.2} S255\n", first_line.x(), first_line.y()).as_str();
        }
    }

    for line in iterator {
        gcode += format!("M03 F1000 X{:.2} Y{:.2} S255\n", line.x(), line.y()).as_str();
    }

    gcode + gcode_end.as_str()
}

pub(crate) fn dxf_to_gcode(dxf_contents: &str) -> String {
    generate_gcode(&glue_polylines(parse(&dxf_contents)))
}
