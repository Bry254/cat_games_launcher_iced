use ico::IconDir;
use image::{
    DynamicImage, ImageBuffer,
    imageops::{self, FilterType},
};
use std::{
    env,
    fs::{self, File},
    path::Path,
    process::Command,
};

use crate::libs::vars::Variables;

#[derive(Clone)]
pub struct IconProcesor {
    wrestool: bool,
    icoextract: bool,
}

impl IconProcesor {
    pub fn gen_outpath(&self, game: &String) -> std::path::PathBuf {
        return env::home_dir()
            .unwrap()
            .join(".local/share/icons/hicolor/128x128/apps")
            .join(format!("{}_cat_game.png", game));
    }

    pub fn command_exists(cmd: &str) -> bool {
        Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {} >/dev/null 2>&1", cmd))
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    pub fn locate_icon(&self, file: &String, game: &String) -> String {
        let carpeta = Path::new(file).parent().unwrap();
        for i in vec!["icon.png", "icon.jpg", "icon.jpeg", "logo.png"] {
            let icono = carpeta.join(i);
            if icono.exists() {
                if self
                    .image_resize(&carpeta.to_string_lossy().to_string(), game)
                    .is_ok()
                {
                    return carpeta.to_string_lossy().to_string();
                }
            }
        }
        let icono = carpeta.join("icon.ico");
        if icono.exists() {
            return self.ico_to_png(&icono.to_string_lossy().to_string(), &game);
        };
        return String::new();
    }

    pub fn image_resize(&self, file: &String, game: &String) -> anyhow::Result<()> {
        let reader = image::open(file)?;
        let img = reader.resize(128, 128, imageops::Lanczos3);
        println!("{:?}", self.gen_outpath(game));
        img.save_with_format(self.gen_outpath(game), image::ImageFormat::Png)?;
        return Ok(());
    }

    pub fn ico_to_png(&self, ico: &String, game: &String) -> String {
        let file = File::open(ico).unwrap();
        let icon_dir = IconDir::read(file).unwrap();
        let entry = icon_dir
            .entries()
            .iter()
            .max_by_key(|e| e.width() * e.height())
            .ok_or("No icon entries")
            .unwrap();
        let image = entry.decode().unwrap();
        let buffer =
            ImageBuffer::from_raw(image.width(), image.height(), image.rgba_data().to_vec())
                .unwrap();
        let image = DynamicImage::ImageRgba8(buffer).resize(128, 128, FilterType::Lanczos3);
        image.save(self.gen_outpath(game)).unwrap();
        return self.gen_outpath(game).to_string_lossy().to_string();
    }

    pub fn icon_exe(&self, path: &str, file: &str, game: &String) -> String {
        if self.wrestool || self.icoextract {
            let ico = Path::new(path)
                .join("icon.ico")
                .to_string_lossy()
                .to_string();
            if self.icoextract {
                let _ = Command::new("icoextract")
                    .current_dir(path)
                    .args([file, ico.as_str()])
                    .status();
            } else {
                let _ = Command::new("wrestool")
                    .current_dir(path)
                    .args(["-x", "-t", "14", file, "-o", ico.as_str()])
                    .status();
            }
            self.ico_to_png(&ico, &game.replace(".exe", ""));
            return self
                .gen_outpath(&game.replace(".exe", ""))
                .to_string_lossy()
                .to_string();
        } else {
            println!("No tienes instalado wrestool ni icoextract");
        }
        return String::new();
    }

    pub fn resolve_icon(&self, input: &String, cwd: &String, name: &String) -> String {
        if !input.is_empty() {
            let found = self.locate_icon(input, name);
            if !found.is_empty() {
                return found;
            }
        }
        if input.ends_with(".exe") {
            let out = self.icon_exe(cwd, input, name);
            if !out.is_empty() {
                return out;
            }
        };
        if input.ends_with("ico") {
            self.ico_to_png(input, name);
            return self.gen_outpath(name).to_string_lossy().to_string();
        }
        if input.ends_with("png") || input.ends_with("jpg") || input.ends_with("jpeg") {
            self.image_resize(input, name).ok();
        }
        return input.clone();
    }
}

impl Default for IconProcesor {
    fn default() -> Self {
        Self {
            wrestool: IconProcesor::command_exists("wrestool"),
            icoextract: IconProcesor::command_exists("icoextract"),
        }
    }
}
