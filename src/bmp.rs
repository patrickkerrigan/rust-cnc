use std::io::Cursor;
use std::ops::{BitAnd, Div};
use bmp::{Image, Pixel};

pub(crate) fn bmp_to_gcode(bmp_contents: &[u8], dpi: u16) -> String {
    let mut cursor = Cursor::new(bmp_contents);

    match bmp::from_reader(&mut cursor) {
        Ok(image) => process_image(image, dpi),
        _ => "".into()
    }
}

fn process_image(bitmap: Image, dpi: u16) -> String {
    let width = bitmap.get_width();
    let height = bitmap.get_height();

    let mm_per_pixel: f32 = 25.4 / dpi as f32;

    let mut last_power: u8 = 0;

    let mut gcode = String::new();

    for (x, y) in bitmap.coordinates() {
        let x = if y.bitand(1) == 1 { width - 1 - x} else { x };
        let power = 255 - to_greyscale(&bitmap.get_pixel(x, y));

        if power == last_power && ((x != 0 && x != width - 1) || last_power == 0) {
            continue;
        }

        let mm_x = x as f32 * mm_per_pixel;
        let mm_y = (height - 1 - y) as f32 * mm_per_pixel;

        if last_power == 0 {
            gcode += format!("M05 F2000 X{:.2} Y{:.2}\n", mm_x, mm_y).as_str();
        } else {
            gcode += format!("M03 F1000 X{:.2} Y{:.2} S{}\n", mm_x, mm_y, last_power).as_str();
        }

        last_power = power;
    }

    gcode
}

fn to_greyscale(pixel: &Pixel) -> u8 {
    (pixel.r as u16 + pixel.g as u16 + pixel.b as u16).div(3) as u8
}
