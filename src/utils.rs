use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use std::process::Command;

pub async fn from_here_get_this(
    url: &str,
    parse: &str,
    attr: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let response = reqwest::get(url).await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let mut attribute_values: Vec<String> = Vec::new();
            let html = response.text().await?;

            let document = Html::parse_document(&html);
            let selector = match Selector::parse(parse) {
                Ok(selector) => selector,
                Err(_) => return Err("Failed to create css parser".into())
            };
            let selections = document.select(&selector);

            for element in selections {
                if let Some(target_url) = element.value().attr(attr) {
                    attribute_values.push(target_url.to_owned());
                }
            }

            Ok(attribute_values)
        }
        _ => Err(format!("Received non OK result when getting HTML from '{}'", &url).into()),
    }
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
