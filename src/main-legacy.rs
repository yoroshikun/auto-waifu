extern crate base64;
extern crate fantoccini;
extern crate image;
extern crate tokio;

use std::path::Path;

use base64::decode;
use fantoccini::{Client, Locator};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    // Start the webdriver client
    let mut c = Client::new("http://localhost:9515")
        .await
        .expect("failed to connect to WebDriver");

    // Head to waifulabs
    c.goto("https://waifulabs.com/").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://waifulabs.com/");

    // find and click start
    c.find(Locator::Css(".blue")).await?.click().await?;
    c.wait_for_find(Locator::Css(".sc-dnqmqq"))
        .await?
        .click()
        .await?;
    // Find first girl and click on her
    c.wait_for_find(Locator::Css(".girl"))
        .await?
        .click()
        .await?;
    // Find the first girl again 3 times to get the final result
    c.wait_for_find(Locator::Css(".cross-fade-enter-done"))
        .await?;
    c.wait_for_find(Locator::Css(".girl"))
        .await?
        .click()
        .await?;
    c.wait_for_find(Locator::Css(".cross-fade-enter-done"))
        .await?;
    c.wait_for_find(Locator::Css(".girl"))
        .await?
        .click()
        .await?;
    c.wait_for_find(Locator::Css(".cross-fade-enter-done"))
        .await?;
    c.wait_for_find(Locator::Css(".girl"))
        .await?
        .click()
        .await?;
    c.wait_for_find(Locator::Css(".products")).await?;
    // Wait for my girl to load
    c.wait_for_find(Locator::Css(".my-girl-loaded")).await?;
    // Select my girl and download the image
    let image_src = c
        .find(Locator::Css(".my-girl-image"))
        .await?
        .attr("src")
        .await?;

    // If image (almost certain it exists) Decode base64, load from memory, then save output
    match image_src {
        Some(image_src) => {
            // Strip out the initial part data:image/png;charset=utf-8;base64,
            let base64 = image_src.replace("data:image/png;charset=utf-8;base64,", "");
            image::load_from_memory_with_format(&decode(base64).unwrap(), image::ImageFormat::Png)
                .unwrap()
                .save_with_format(&Path::new("avatar.png"), image::ImageFormat::Png)
                .unwrap();
        }
        None => println!("No Image src found?"),
    }

    c.close().await
}
