use rand::Rng;
use std::env;

mod utils;

/// Example of how to run with cargo:
///     - cargo run /home/ryan/Pictures/Wallpapers/scrapedImage.jpg
/// 
/// Note that this will only work with gnome when in dark mode (see set_desktop_backgound() definition)

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide an absolute path to the resulting image.")
    }

    let image_path_local = &args[1];

    let mut url = String::from("https://jrozalski.com");

    let url_projects = utils::from_here_get_this(&url, "a.album-insta-item", "href")
        .await
        .expect("Failed to get the project urls");

    let mut rng = rand::thread_rng();
    let ridx = rng.gen_range(0..url_projects.len());

    url += &url_projects[ridx];

    let url_images = utils::from_here_get_this(&url, "img.project-assets-image", "src")
        .await
        .expect("Failed to get image urls from project page");

    // Download the first image (the resulting image) on the page
    // all other images are intermediate images
    utils::download_image(&url_images[0], image_path_local).await.expect("Failed to download image");

    utils::set_desktop_backgound(image_path_local).expect("Failed to set desktop background");
}
