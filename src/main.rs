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
    data: ImageData,
}

#[derive(Debug)]
enum ImageData {
    U8(Vec<u8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    F32(Vec<f32>),
    F64(Vec<f64>),
}

fn extract_stars(image: &Image) -> Vec<Star> {
    // Placeholder: a real implementation would analyze pixels to find stars.
    // Here we just return an empty Vec for demonstration.
    Vec::new()
}

fn get_image_data(path: &str) -> Result<Image, Box<dyn std::error::Error>> {
    let file: File = File::open(path)?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut bits_per_data_pixel: Option<i32> = None;
    let mut width: Option<i32> = None;
    let mut height: Option<i32> = None;

    let mut card: [u8; 80] = [0u8; 80];

    loop {
        read_exact_bytes(&mut reader, &mut card)?;
        let line: String = std::str::from_utf8(&card)?.to_string();
        let keyword: &str = line[..8].trim();

        if keyword == "END" {
            break;
        }

        match keyword {
            "BITPIX" => {
                bits_per_data_pixel = Some(parse_value(&line)?);
            }
            "NAXIS1" => {
                width = Some(parse_value(&line)?);
            }
            "NAXIS2" => {
                height = Some(parse_value(&line)?);
            }
            _ => {}
        }
    }

    let (bpdp, w, h): (i32, usize, usize) = match (bits_per_data_pixel, width, height) {
        (Some(bpdp), Some(w), Some(h)) => (bpdp, w as usize, h as usize),
        _ => return Err("Missing BITPIX or NAXIS1 or NAXIS2 in FITS header".into()),
    };

    use std::io::{Seek, SeekFrom};
    let mut inner: File = reader.into_inner();
    let position: u64 = inner.stream_position()?;
    let block_size: u64 = 2880;
    let next_block: u64 = position.div_ceil(block_size) * block_size;
    if next_block > position {
        inner.seek(SeekFrom::Start(next_block))?;
    }

    let pixel_count: usize = w.checked_mul(h).ok_or("image too large")?;

    let data: ImageData = match bpdp {
        8 => {
            // unsigned bytes
            let mut buffer: Vec<u8> = vec![0u8; pixel_count];
            inner.read_exact(&mut buffer)?;
            ImageData::U8(buffer)
        }
        16 => {
            // 16-bit integers (big-endian). FITS uses signed integers for BITPIX=16.
            let mut raw: Vec<u8> = vec![0u8; pixel_count * size_of::<i16>()];
            inner.read_exact(&mut raw)?;
            let mut vector: Vec<i16> = Vec::with_capacity(pixel_count);
            for chunk in raw.chunks_exact(2) {
                vector.push(to_i16_be(chunk));
            }
            ImageData::I16(vector)
        }
        32 => {
            // 32-bit signed integers
            let mut raw: Vec<u8> = vec![0u8; pixel_count * size_of::<i32>()];
            inner.read_exact(&mut raw)?;
            let mut vector: Vec<i32> = Vec::with_capacity(pixel_count);
            for chunk in raw.chunks_exact(4) {
                vector.push(to_i32_be(chunk));
            }
            ImageData::I32(vector)
        }
        -32 => {
            // 32-bit IEEE float
            let mut raw: Vec<u8> = vec![0u8; pixel_count * size_of::<f32>()];
            inner.read_exact(&mut raw)?;
            let mut vector: Vec<f32> = Vec::with_capacity(pixel_count);
            for chunk in raw.chunks_exact(4) {
                vector.push(to_f32_be(chunk));
            }
            ImageData::F32(vector)
        }
        -64 => {
            // 64-bit IEEE float
            let mut raw: Vec<u8> = vec![0u8; pixel_count * size_of::<f64>()];
            inner.read_exact(&mut raw)?;
            let mut vector: Vec<f64> = Vec::with_capacity(pixel_count);
            for chunk in raw.chunks_exact(8) {
                vector.push(to_f64_be(chunk));
            }
            ImageData::F64(vector)
        }
        other => return Err(format!("Unsupported BITPIX value: {}", other).into()),
    };

    Ok(Image {
        bits_per_data_pixel: bpdp,
        width: w as i32,
        height: h as i32,
        data,
    })
}

fn parse_value_inner(line: &str) -> Result<String, Box<dyn std::error::Error>> {
    let raw_value: &str = if let Some(eq_pos) = line.find('=') {
        let after_eq: &str = &line[eq_pos + 1..];
        after_eq.split('/').next().unwrap_or("").trim()
    } else {
        if line.len() >= 11 {
            line[10..].split('/').next().unwrap_or("").trim()
        } else {
            line.trim()
        }
    };

    let value: &str = raw_value.trim();
    let value: &str = if value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2 {
        &value[1..value.len() - 1]
    } else {
        value
    };

    Ok(value.to_string())
}

fn parse_value(line: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let string: String = parse_value_inner(line)?;
    let number: i32 = string.parse()?;
    Ok(number)
}

fn read_exact_bytes<R: Read>(
    reader: &mut R,
    buf: &mut [u8],
) -> Result<(), Box<dyn std::error::Error>> {
    reader.read_exact(buf)?;
    Ok(())
}

fn to_i16_be(bytes: &[u8]) -> i16 {
    i16::from_be_bytes([bytes[0], bytes[1]])
}
fn to_i32_be(bytes: &[u8]) -> i32 {
    i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}
fn to_f32_be(bytes: &[u8]) -> f32 {
    f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}
fn to_f64_be(bytes: &[u8]) -> f64 {
    f64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Cli = Cli::parse();

    let file_path: String = arguments.file_path;
    let extension: &str = Path::new(&file_path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    // TODO: merge all readers into one reader
    match extension {
        "fits" => {
            let image: Image = get_image_data(&file_path)?;
            println!("{:?}", image);
            let stars: Vec<Star> = extract_stars(&image);
            println!("Found {} stars (placeholder)", stars.len());
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
        _ => panic!("Invalid file extension."),
    }

    Ok(())

    // identify stars

    // find brightest star

    // look at 4 nearest stars to that star and measure distances between
    // them (hash code)

    // repeat these steps for the stars in the star database

    // star database hash codes will be compared with the image hash codes until
    // some matches are found.

    // Once some matches are found it is possible to calculate the precise position
    // of the image with the matching database stars.
}
