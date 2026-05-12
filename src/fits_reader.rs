use std::fs::File;
use std::io::{BufReader, Read};

use crate::dimensions::Dimensions;

pub fn get_image_dimensions(path: &str) -> Result<Dimensions, Box<dyn std::error::Error>> {
    let file: File = File::open(path)?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut width: Option<i32> = None;
    let mut height: Option<i32> = None;

    let mut card: [u8; 80] = [0u8; 80];

    loop {
        reader.read_exact(&mut card)?;

        let line: &str = std::str::from_utf8(&card)?;
        let keyword: &str = line[..8].trim();

        if keyword == "END" {
            break;
        }

        match keyword {
            "NAXIS1" => {
                width = Some(parse_value(line)?);
            }
            "NAXIS2" => {
                height = Some(parse_value(line)?);
            }
            _ => {}
        }

        if width.is_some() && height.is_some() {
            break;
        }
    }

    match (width, height) {
        (Some(w), Some(h)) => Ok(Dimensions {
            width: w,
            height: h,
        }),
        _ => Err("Missing NAXIS1 or NAXIS2 in FITS header".into()),
    }
}

fn parse_value(line: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let equal_position: usize = line.find('=').ok_or("Missing '=' in FITS header line")?;

    let mut value: &str = &line[equal_position + 1..];

    if let Some(slash_pos) = value.find('/') {
        value = &value[..slash_pos];
    }

    Ok(value.trim().parse()?)
}
