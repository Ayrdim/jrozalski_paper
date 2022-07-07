use reqwest;
use scraper::{Html, Selector};
use std::process::Command;

pub async fn from_here_get_this(url: &str, parse: &str, attr: &str) -> Option<Vec<String>> {
    let response = reqwest::get(url)
        .await
        .expect(&format!("Failed to get HTML from '{}'", url));

    // We havent received an error but we cannot continue if the status code isnt OK

    match response.status() {
        reqwest::StatusCode::OK => {
            let mut attribute_values: Vec<String> = Vec::new();
            let html = response
                .text()
                .await
                .expect("Failed to parse HTML response to text");

            let document = Html::parse_document(&html);
            let selector = Selector::parse(parse).expect("Failed to create parser");
            let selections = document.select(&selector);

            for element in selections {
                if let Some(target_url) = element.value().attr(attr) {
                    attribute_values.push(target_url.to_owned());
                }
            }

            Some(attribute_values)
        }
        _ => None,
    }
}

pub async fn download_image(url: &str, save_to: &str) {
    let image_bytes = reqwest::get(url)
        .await
        .expect(&format!("Failed to get HTML from '{}'", url))
        .bytes()
        .await
        .expect("Failed to convert HTML to bytes");

    image::load_from_memory(&image_bytes)
        .expect("Failed to get load bytes as image")
        .save(&save_to)
        .expect(&format!("Failed to save image as '{}'", save_to));
}

pub fn set_desktop_backgound(image_path: &str) {
    Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri-dark")
        .arg(format!("file://{}", &image_path))
        .output()
        .expect(&format!(
            "Failed to set '{}' image as background",
            image_path
        ));
}
