mod coordinates;
mod dimensions;
mod fits_reader;
mod image;
mod star;

use std::error::Error;
use std::fs;

use crate::dimensions::Dimensions;

/*
    https://www.hnsky.org/astap_astrometric_solving.htm
*/

fn main() -> Result<(), Box<dyn Error>> {
    // TODO REMOVE: get test file path
    let buffer: Vec<u8> = fs::read("test_file_path.txt")?;
    let test_file_path: String = String::from_utf8(buffer)?.trim().to_string();

    // get image dimensions
    // TODO: add FitReader for .fit files
    // TODO: add TifReader for .tif files
    // TODO: add TifsReader for .tifs files
    // TODO: add PngReader for .png files
    // TODO: merge all readers into one reader
    let dimensions: Dimensions = fits_reader::get_image_dimensions(&test_file_path)?;

    println!("{:?}", dimensions);

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
