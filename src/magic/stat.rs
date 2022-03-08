use std::path::PathBuf;

use crate::errors::ImageErrors;
use crate::resize::get_image_files;

pub fn get_stat(src_folder: PathBuf) -> Result<(usize, f64), ImageErrors> {
    let image_files = get_image_files(src_folder.to_path_buf())?;
    let size = image_files
        .iter()
        .map(move |f| f.metadata().unwrap().len())
        .sum::<f64>();
    Ok((image_files.len(), (size / 1_000_000) as f64))
}
