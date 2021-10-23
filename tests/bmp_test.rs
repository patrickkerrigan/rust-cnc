use std::fs::read_to_string;
use rust_cnc::process_bmp;

const DATA_PATH: &str = "tests/data/bmp/";

#[test]
fn bmp_test() {
    test_bmp("test.bmp", "test.nc", 200);
}

fn test_bmp(bmp_file: &str, nc_file: &str, dpi: u16) {
    let bmp = std::fs::read(DATA_PATH.to_owned() + bmp_file).unwrap();
    let gcode = read_to_string(DATA_PATH.to_owned() + nc_file).unwrap().replace('\r', "");

    assert_eq!(gcode.trim_end(), process_bmp(&bmp, dpi).trim_end());
}
