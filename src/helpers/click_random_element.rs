use headless_chrome::Element;
use rand::Rng;

pub fn click_random_element(elements: Vec<Element>) -> Result<(), failure::Error> {
  let mut rng = rand::thread_rng();
  elements[rng.gen_range(0, elements.len() - 1)].click()?;
  Ok(())
}
