use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use std::process::Command;

pub async fn from_here_get_this(
    url: &str,
    parse: &str,
    attr: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut v: Vec<String> = Vec::new();

    let response = reqwest::get(url).await?;

    // We havent received an error but we cannot continue if the status code isnt OK
    if reqwest::StatusCode::OK != response.status() {
        return Err("Request status code was not OK.".into());
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let selector = match Selector::parse(parse) {
        Ok(selector) => selector,
        Err(_) => return Err("Request status code was not OK.".into()),
    };

    let selections = document.select(&selector);

    for element in selections {
        if let Some(target_url) = element.value().attr(attr) {
            v.push(target_url.to_owned());
        }
    }

    return Ok(v);
}

pub async fn download_image(url: &str, save_to: &str) -> Result<(), Box<dyn Error>> {
    let image_bytes = reqwest::get(url).await?.bytes().await?;
    image::load_from_memory(&image_bytes)?.save(&save_to)?;

    Ok(())
}

pub fn set_desktop_backgound(image_path: &str) -> Result<(), Box<dyn Error>> {
    Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri-dark")
        .arg(format!("file://{}", &image_path))
        .output()?;

    Ok(())
}
