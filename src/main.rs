use clap::Parser;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/*
    https://www.hnsky.org/astap_astrometric_solving.htm
*/

#[derive(Parser)]
#[command(rename_all = "snake_case")]
struct Cli {
    #[arg(long)]
    file_path: String,
}

#[derive(Debug)]
struct Star {
    x: i32,
    y: i32,
    size: i32,
}

#[derive(Debug)]
struct Image {
    bits_per_data_pixel: i32,
    width: i32,
    height: i32,
    stars: Vec<Star>,
}

fn get_image_data(path: &str) -> Result<Image, Box<dyn std::error::Error>> {
    let file: File = File::open(path)?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut bits_per_data_pixel: Option<i32> = None;
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
            "BITPIX" => {
                bits_per_data_pixel = Some(parse_value(line)?);
            }
            "NAXIS1" => {
                width = Some(parse_value(line)?);
            }
            "NAXIS2" => {
                height = Some(parse_value(line)?);
            }
            _ => {}
        }

        if bits_per_data_pixel.is_some() && width.is_some() && height.is_some() {
            break;
        }
    }

    match (bits_per_data_pixel, width, height) {
        (Some(bpdp), Some(w), Some(h)) => Ok(Image {
            bits_per_data_pixel: bpdp,
            width: w,
            height: h,
            stars: vec![],
        }),
        _ => Err("Missing BITPIX or NAXIS1 or NAXIS2 in FITS header".into()),
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

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Cli = Cli::parse();

    let file_path: String = arguments.file_path;
    let extension: &str = Path::new(&file_path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    // TODO: get all image data into structure from this match, instead of just image dimensions
    // TODO: merge all readers into one reader
    match extension {
        "fits" => {
            // get image image_data
            let image_data: Image = get_image_data(&file_path)?;
            println!("{:?}", image_data);
        }
        "fit" => {
            // TODO: add FitReader for .fit files
            todo!()
        }
        "tif" => {
            // TODO: add TifReader for .tif files
            todo!()
        }
        "tifs" => {
            // TODO: add TifsReader for .tifs files
            todo!()
        }
        "png" => {
            // TODO: add PngReader for .png files
            todo!()
        }
        _ => panic!("Unrecognised file extension."),
    }

    // get image data

    // identify stars

    // find brightest star

    // look at 4 nearest stars to that star and measure distances between
    // them (hash code)

    // repeat these steps for the stars in the star database

    // star database hash codes will be compared with the image hash codes until
    // some matches are found.

    // Once some matches are found it is possible to calculate the precise position
    // of the image with the matching database stars.

    Ok(())
}
