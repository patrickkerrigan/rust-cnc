use std::fs::read_to_string;
use rust_cnc::{process};

const DATA_PATH: &str = "tests/data/";

#[test]
fn dxf_shape_test() {
    test_dxf("shapes.dxf", "shapes.nc");
}

#[test]
fn dxf_text_test() {
    test_dxf("text.dxf", "text.nc");
}

#[test]
fn dxf_bulge_clockwise_test() {
    test_dxf("bulge.dxf", "bulge.nc");
}

#[test]
fn dxf_bulge_anticlockwise_test() {
    test_dxf("bulge2.dxf", "bulge2.nc");
}

fn test_dxf(dxf_file: &str, nc_file: &str) {
    let dxf = read_to_string(DATA_PATH.to_owned() + dxf_file).unwrap();
    let gcode = read_to_string(DATA_PATH.to_owned() + nc_file).unwrap();

    assert_eq!(gcode.trim_end(), process(&dxf).trim_end());
}
