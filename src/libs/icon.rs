use std::{path::Path, process::Command};

use crate::libs::lutris;

#[derive(Clone)]
pub struct IconProcesor {
    wrestool: bool,
    icoextract: bool,
}

impl IconProcesor {
    pub fn command_exists(cmd: &str) -> bool {
        Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {} >/dev/null 2>&1", cmd))
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    pub fn locateicon(&self, file: &String, name: &String) -> String {
        let folder = Path::new(file).parent().unwrap();
        for i in vec![
            "icon.png",
            "icon.jpg",
            "icon.jpeg",
            "icon.ico",
            "ico.ico",
            "logo.png",
        ] {
            let icono = folder.join(i);
            if icono.exists() {
                return icono.to_string_lossy().to_string();
            }
        }
        println!("No se encontro icono en la capeta.");
        if let Ok(r) = lutris::get_icon(name, file) {
            return r;
        }
        return String::new();
    }
    // pub fn locate_icon(&self, file: &String, game: &String) -> String {
    //     let carpeta = Path::new(file).parent().unwrap();
    //     for i in vec!["icon.png", "icon.jpg", "icon.jpeg", "logo.png"] {
    //         let icono = carpeta.join(i);
    //         if icono.exists() {
    //             if self
    //                 .image_resize(&carpeta.to_string_lossy().to_string(), game)
    //                 .is_ok()
    //             {
    //                 return carpeta.to_string_lossy().to_string();
    //             }
    //         }
    //     }
    //     let icono = carpeta.join("icon.ico");
    //     if icono.exists() {
    //         return self.ico_to_png(&icono.to_string_lossy().to_string(), &game);
    //     };
    //     return String::new();
    // }

    // pub fn image_resize(&self, file: &String, game: &String) -> anyhow::Result<()> {
    //     let reader = image::open(file)?;
    //     let img = reader.resize(128, 128, imageops::Lanczos3);
    //     println!("{:?}", self.gen_outpath(game));
    //     img.save_with_format(self.gen_outpath(game), image::ImageFormat::Png)?;
    //     return Ok(());
    // }

    // pub fn ico_to_png(&self, ico: &String, game: &String) -> String {
    //     let file = File::open(ico).unwrap();
    //     let icon_dir = IconDir::read(file).unwrap();
    //     let entry = icon_dir
    //         .entries()
    //         .iter()
    //         .max_by_key(|e| e.width() * e.height())
    //         .ok_or("No icon entries")
    //         .unwrap();
    //     let image = entry.decode().unwrap();
    //     let buffer =
    //         ImageBuffer::from_raw(image.width(), image.height(), image.rgba_data().to_vec())
    //             .unwrap();
    //     let image = DynamicImage::ImageRgba8(buffer).resize(128, 128, FilterType::Lanczos3);
    //     image.save(self.gen_outpath(game)).unwrap();
    //     return self.gen_outpath(game).to_string_lossy().to_string();
    // }

    pub fn icon_exe(&self, path: &str) -> String {
        if self.wrestool || self.icoextract {
            let folder = Path::new(path).parent().unwrap();
            let ico = folder.join("icon.ico").to_string_lossy().to_string();
            if self.icoextract {
                eprintln!("Usando icoxtract...icoextract {path} {ico}");
                let _ = Command::new("icoextract")
                    .current_dir(folder)
                    .args([path, ico.as_str()])
                    .status();
            } else {
                eprintln!("Usando wrestool...wrestool -x -t 14 {path} -o {ico}");
                let _ = Command::new("wrestool")
                    .current_dir(folder)
                    .args(["-x", "-t", "14", path, "-o", ico.as_str()])
                    .status();
            }
            return ico;
        } else {
            println!("Error: No tienes instalado wrestool ni icoextract.");
        }
        return String::new();
    }
    // pub fn resolve_icon(&self, input: &String, cwd: &String, name: &String) -> String {
    //     if !input.is_empty() {
    //         let found = self.locate_icon(input, name);
    //         if !found.is_empty() {
    //             return found;
    //         }
    //     }
    //     if input.ends_with(".exe") {
    //         let out = self.icon_exe(cwd, input, name);
    //         if !out.is_empty() {
    //             return out;
    //         }
    //     };
    //     if input.ends_with("ico") {
    //         self.ico_to_png(input, name);
    //         return self.gen_outpath(name).to_string_lossy().to_string();
    //     }
    //     if input.ends_with("png") || input.ends_with("jpg") || input.ends_with("jpeg") {
    //         self.image_resize(input, name).ok();
    //     }
    //     return input.clone();
    // }
}

impl Default for IconProcesor {
    fn default() -> Self {
        Self {
            wrestool: IconProcesor::command_exists("wrestool"),
            icoextract: IconProcesor::command_exists("icoextract"),
        }
    }
}
