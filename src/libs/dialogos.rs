use std::env;

use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

pub fn confirm(titulo: &str, mensaje: &str) -> bool {
    let confirmed = MessageDialog::new()
        .set_title(titulo)
        .set_description(mensaje)
        .set_level(MessageLevel::Info)
        .set_buttons(MessageButtons::YesNo)
        .show();
    return confirmed == MessageDialogResult::Yes;
}

pub fn filepicker(name: &str, tipo: &[&str]) -> String {
    let archivo = rfd::FileDialog::new()
        .set_title(format!("Selecionar {name}"))
        .add_filter(name, tipo)
        .pick_file();
    return archivo.unwrap_or_default().to_string_lossy().to_string();
}

pub fn filepicker_path(name: &str, tipo: &[&str], path: &str) -> String {
    let archivo = rfd::FileDialog::new()
        .set_title(format!("Selecionar {name}"))
        .add_filter(name, tipo)
        .set_directory(path)
        .pick_file();
    return archivo.unwrap_or_default().to_string_lossy().to_string();
}

pub fn folderpicker() -> String {
    let carpeta = rfd::FileDialog::new()
        .set_title("Selecionar carpeta")
        .pick_folder();
    return carpeta.unwrap_or_default().to_string_lossy().to_string();
}

pub fn iconpicker() -> String {
    let home = env::home_dir().unwrap().to_string_lossy().to_string();
    let archivo = rfd::FileDialog::new()
        .set_title("Selecione icono")
        .add_filter("Imagenes", &["exe", "png", "jpeg", "ico", "svg"])
        .set_directory(format!("{}/.local/share/icons/hicolor/128x128/apps/", home))
        .pick_file();
    return archivo.unwrap_or_default().to_string_lossy().to_string();
}

pub fn folderpicker_title(titulo: &str) -> String {
    let carpeta = rfd::FileDialog::new().set_title(titulo).pick_folder();
    return carpeta.unwrap_or_default().to_string_lossy().to_string();
}
