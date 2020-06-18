extern crate base64;
extern crate headless_chrome;
extern crate image;
extern crate rand;

mod helpers;

use std::path::Path;

use base64::decode;
use headless_chrome::Browser;

use helpers::click_random_element::click_random_element;

fn main() -> Result<(), failure::Error> {
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;
    tab.set_default_timeout(std::time::Duration::from_secs(15));

    // Head to waifulabs
    tab.navigate_to("https://waifulabs.com/")?;
    // let url = c.current_url().await?;
    // assert_eq!(url.as_ref(), "https://waifulabs.com/");

    // find and click start
    tab.wait_for_element("a.blue.block.button")?.click()?;
    tab.wait_for_element(".sc-dnqmqq")?.click()?;
    // Find girls and click a random one
    let girls = tab.wait_for_elements(".girl")?;
    click_random_element(girls)?;

    // Find the first girl again 3 times to get the final result
    tab.wait_for_element(".cross-fade-enter-done")?;
    let girls = tab.wait_for_elements(".girl")?;
    click_random_element(girls)?;
    tab.wait_for_element(".cross-fade-enter-done")?;
    let girls = tab.wait_for_elements(".girl")?;
    click_random_element(girls)?;
    tab.wait_for_element(".cross-fade-enter-done")?;
    let girls = tab.wait_for_elements(".girl")?;
    click_random_element(girls)?;
    tab.wait_for_element(".products")?;
    // Wait for my girl to load
    tab.wait_for_element(".my-girl-loaded")?;
    // Select my girl and download the image
    let image_attributes = tab
        .wait_for_element(".my-girl-image")?
        .get_attributes()?
        .unwrap();
    let src = image_attributes.get("src");

    // If image (almost certain it exists) Decode base64, load from memory, then save output
    match src {
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

    Ok(())
}
