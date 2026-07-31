use std::fs::File;
use std::io::copy;
use std::path::Path;

use serde_json::Value;

pub fn download_file(url: &str, file: &str) -> anyhow::Result<()> {
    let mut resp = ureq::get(url).call()?;
    let mut file = File::create(file)?;
    copy(&mut resp.body_mut().as_reader(), &mut file)?;
    return Ok(());
}

pub fn get_icon(name: &str, path: &str) -> anyhow::Result<String> {
    println!("⌕ Buscando icono de {name} online....");
    let mut json = ureq::get(format!(
        "https://lutris.net/api/games/{}",
        name.replace(" ", "-")
    ))
    .call()?;
    let body = json.body_mut().read_to_string()?;
    let json: Value = serde_json::from_str(&body.as_str())?;
    if let Some(url) = json.get("icon_url").and_then(|v| v.as_str()) {
        if let Some(parent) = Path::new(path).parent() {
            let file = parent.join("icon.png").to_string_lossy().to_string();
            download_file(&url, &file)?;
            println!("{} {}", &file, &name);
            return Ok(file);
        }
    }
    println!("Icono no encontrado...");
    Err(anyhow::anyhow!(""))
}
