use std::fs::read_to_string;
use rust_cnc::{process};

#[test]
fn dxf_shape_test() {
    let dxf = read_to_string("tests/data/shapes.dxf").unwrap();
    let gcode = read_to_string("tests/data/shapes.nc").unwrap();

    assert_eq!(gcode.trim_end(), process(&dxf));
}

#[test]
fn dxf_text_test() {
    let dxf = read_to_string("tests/data/text.dxf").unwrap();
    let gcode = read_to_string("tests/data/text.nc").unwrap();

    assert_eq!(gcode.trim_end(), process(&dxf));
}
