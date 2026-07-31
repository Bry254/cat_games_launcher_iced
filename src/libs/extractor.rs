use std::fs::{self, File};
use std::io::copy;
use std::path::Path;
use tar::Archive;
use zip::ZipArchive;

pub fn unzip(zip_path: &String, dest: &String) -> zip::result::ZipResult<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(dest).join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

pub fn tar(tar_path: &String, target_folder: &String) -> anyhow::Result<()> {
    let file = File::open(tar_path)?;
    std::fs::create_dir_all(target_folder)?;
    let mut archive = Archive::new(file);
    archive.unpack(target_folder)?;
    Ok(())
}
