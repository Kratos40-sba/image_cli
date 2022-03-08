use std::{fs, io};
use std::path::PathBuf;
use std::time::Instant;

use image::ImageFormat;

use crate::errors::ImageErrors;

pub fn get_image_files(src: PathBuf) -> Result<Vec<PathBuf>, ImageErrors> {
    let entries = fs::read_dir(src)
        .map_err(|e| ImageErrors::UserInputError("invalid user input".to_string()))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .into_iter()
        .filter(|r| {
            r.extension() == Some("JPG".as_ref())
                || r.extension() == Some("jpg".as_ref())
                || r.extension() == Some("PNG".as_ref())
                || r.extension() == Some("png".as_ref())
        })
        .collect();
    Ok(entries)
}

fn resize_image(size: u32, src_folder: &mut PathBuf) -> Result<(), ImageErrors> {
    let new_file_name = src_folder
        .file_stem()
        .unwrap()
        .to_str()
        .ok_or(std::io::ErrorKind::InvalidInput)
        .map(|f| format!("{}.png", f));
    let mut dest_folder = src_folder.clone();
    dest_folder.pop();
    dest_folder.push("tmp/");
    if !dest_folder.exists() {
        fs::create_dir(&dest_folder)?;
    }
    dest_folder.pop();
    dest_folder.push("tmp/tmp.png");
    dest_folder.set_file_name(new_file_name?.as_str());
    let timer = Instant::now();
    let img = image::open(&src_folder)?;
    let scaled = img.thumbnail(size, size);
    let mut output = fs::create_dir(&dest_folder)?;
    scaled.write_to(&mut output, ImageFormat::Png);
    println!(
        "Thumbnail file: {:?} to size {}x{} in {}. Output
        file
        in {:?}",
        src_folder,
        size,
        size,
        Elapsed::from(&timer),
        dest_folder
    );
    Ok(())
}
